use std::sync::{
    atomic::{AtomicUsize, Ordering},
    mpsc::{channel, Receiver, Sender},
    Arc, Condvar, Mutex,
};
use std::thread;

trait FnBox {
    fn call_box(self: Box<Self>);
}

// 避免使用 `#![feature(fnbox)]` 特性
impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) { (*self)() }
}

type Thunk<'a> = Box<FnBox + Send + 'a>;

// 为了让线程池维护的线程可以共享相同的数据
// 还需要一个共享数据的结构体
struct ThreadPoolSharedData {
    // 标记线程名称
    // 线程池的线程都用统一的名称.
    name: Option<String>,
    // 用于存储从 Channel 中接收任务的接收端(rx)
    // 多线程环境下, `Receiver<Thunk<'static>>` 类型不能被安全共享, 所以要加锁
    // 此处 `Thunk<'static>` 代表 `Box<FnBox + Send + 'static>`
    // 要执行的具体任务均为闭包
    job_receiver: Mutex<Receiver<Thunk<'static>>>,
    // 代表空锁 - 用于实现线程池的 `join()` 方法
    empty_trigger: Mutex<()>,
    // 代表空的条件变量 - 需要配合互斥锁使用 - 用于实现线程池的 `join()` 方法
    empty_condvar: Condvar,
    // 代表线程池中的总队列数(多线程操作, 使用原子类型保证原子性)
    queued_count: AtomicUsize,
    // 代表正在执行任务的工作线程数(多线程操作, 使用原子类型保证原子性)
    active_count: AtomicUsize,
    // 代表线程池允许的最大工作线程数
    max_thread_count: AtomicUsize,
    // 用于记录线程池中发生恐慌的工作线程数, 同样适用原子类型 `AtomicUsize` 来保证原子性
    panic_count: AtomicUsize,
    // 用于设置工作线程栈大小, 若不设置, 默认为 8MB
    stack_size: Option<usize>,
}

impl ThreadPoolSharedData {
    // 当 `queued_count > 0` 或 `active_count > 0` 时, 表示线程池处于正常工作状态
    fn has_work(&self) -> bool {
        self.queued_count.load(Ordering::SeqCst) > 0 || self.active_count.load(Ordering::SeqCst) > 0
    }


    fn no_work_notify_all(&self) {
        // 判断工作状态
        // 若线程池中的工作线程处于闲置状态, 则代表所有任务均以完成
        if !self.has_work() {
            *self
                // 通过 `empty_trigger()` 拿到锁
                .empty_trigger
                .lock()
                .expect("Unable to notify all joining threads");
            // 再调用该方法通知所有线程解除阻塞状态, 该方法用于配合线程池的 `join()` 方法
            self.empty_condvar.notify_all();
        }
    }
}

// 线程结构体
pub struct ThreadPool {
    // 用于存储 Channel 发送端(tx)
    // 使用它给工作线程发送具体的任务
    jobs: Sender<Thunk<'static>>,
    // 记录工作线程共享的数据
    shared_data: Arc<ThreadPoolSharedData>,
}

impl ThreadPool {
    // 初始化线程池
    pub fn new(num_threads: usize) -> ThreadPool {
        // 使用构建者模式来定制工作线程
        Builder::new().num_threads(num_threads).build()
    }


    // 用于将任务添加到 Channel 队列中
    // 可以通过此方法向队列中多次添加任务
    pub fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // 同时使用 `AtomiUsize` 的 `fetch_add()` 方法将 `queued_count` 累加一次
        self.shared_data.queued_count.fetch_add(1, Ordering::SeqCst);
        self.jobs
            .send(Box::new(job))
            .expect("unable to send job into queue.");
    }

    // 用于在需要时阻塞主线程等待线程池中的所有任务执行完毕
    pub fn join(&self) {
        // 若线程池处于闲置状态, 提前返回
        if self.shared_data.has_work() == false {
            return ();
        }

        // 获得互斥锁
        let mut lock = self.shared_data.empty_trigger.lock().unwrap();

        // 若线程池中的工作线程一直处于正常工作状态
        while self.shared_data.has_work() {
            // 则调用 `empty_condvar` 的 `wait()` 方法来阻塞当前线程
            // 知道获得解除阻塞的通知
            lock = self.shared_data.empty_condvar.wait(lock).unwrap();
        }
    }
}


// Builder 结构体
pub struct Builder {
    num_threads: Option<usize>,
    thread_name: Option<String>,
    thread_stack_size: Option<usize>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            num_threads: None,
            thread_name: None,
            thread_stack_size: None,
        }
    }

    // 通过参数设置工作线程数
    pub fn num_threads(mut self, num_threads: usize) -> Builder {
        assert!(num_threads > 0);
        self.num_threads = Some(num_threads);
        self
    }

    // 初始化最终的线程池
    pub fn build(self) -> ThreadPool {
        // 创建无界队列
        let (tx, rx) = channel::<Thunk<'static>>();

        // 得到工作线程数, 若不存在, 获取当前 CPU 核心数
        let num_threads = self.num_threads.unwrap_or_else(num_cpus::get);

        let shared_data = Arc::new(ThreadPoolSharedData {
            name: self.thread_name,
            job_receiver: Mutex::new(rx),
            empty_condvar: Condvar::new(),
            empty_trigger: Mutex::new(()),
            queued_count: AtomicUsize::new(0),
            active_count: AtomicUsize::new(0),
            max_thread_count: AtomicUsize::new(num_threads),
            panic_count: AtomicUsize::new(0),
            stack_size: self.thread_stack_size,
        });

        // 迭代线程数
        for _ in 0..num_threads {
            // 用于生成工作线程
            spawn_in_pool(shared_data.clone());
        }

        ThreadPool {
            jobs: tx,
            shared_data: shared_data,
        }
    }
}


fn spawn_in_pool(shared_data: Arc<ThreadPoolSharedData>) {
    // 注意: 此处使用的是 `thread` 模块的方法
    let mut builder = thread::Builder::new();

    // 通过 `shared_ddata` 中存储的 `name` 和 `stack_size` 来定制生成线程
    if let Some(ref name) = shared_data.name {
        builder = builder.name(name.clone());
    }

    if let Some(ref stack_size) = shared_data.stack_size {
        builder = builder.stack_size(stack_size.to_owned());
    }

    builder
        // 创建工作线程
        .spawn(move || {
            let sentinel = Sentinel::new(&shared_data);

            // 阻塞当前工作现场从任务队列中取具体的任务来执行.
            loop {
                // 此处内存顺序为 `Ordering::Acquire`
                // 代表 `load()` 方法能看到之前所有线程对 `active_count` 所做的吸怪
                let thread_counter_val = shared_data.active_count.load(Ordering::Acquire);
                // 此处使用内存顺序为 `Ordering::Relaxed`
                // 这是因为 `max_thread_count` 的值不会被底层线程读取顺序影响到
                let max_thread_count_val = shared_data.max_thread_count.load(Ordering::Relaxed);

                // 若工作队列数大于最大的线程数, 则退出此循环
                if thread_counter_val >= max_thread_count_val {
                    break;
                }

                let message = {
                    let lock = shared_data
                        // 先得到 `job_receiver` 的锁
                        .job_receiver
                        .lock()
                        .expect("unable to lock job_receiver");
                    // 然后调用 `recv()` 方法从队列中获取任务
                    lock.recv()
                };

                // 匹配得到具体的闭包任务
                let job = match message {
                    Ok(job) => job,
                    // 当是错误类型跳出循环
                    Err(..) => break,
                };

                // 将 `shared_data` 中的 `queued_count - 1`
                // 因为已经从任务队列中取到了一个任务
                shared_data.queued_count.fetch_sub(1, Ordering::SeqCst);
                // 当前工作线程即将对该任务进行处理, 正在执行任务的工作线程数 + 1
                shared_data.active_count.fetch_add(1, Ordering::SeqCst);
                // 执行具体任务
                job.call_box();
                // 正在执行任务 - 1
                shared_data.active_count.fetch_sub(1, Ordering::SeqCst);
                // 通知使用条件变量 `wait()` 方法阻塞的线程在线程池中的任务执行完毕后解除阻塞
                shared_data.no_work_notify_all();
            }

            // 设置实例状态, 表示该线程正常执行完所有任务
            sentinel.cancel();
        })
        .unwrap();
}


// 该结构体用来对具体的工作线程进行监控
struct Sentinel<'a> {
    // 包装线程池共享数据
    shared_data: &'a Arc<ThreadPoolSharedData>,
    // true 当前工作线程为正在工作
    active: bool,
}

impl<'a> Sentinel<'a> {
    fn new(shared_data: &'a Arc<ThreadPoolSharedData>) -> Sentinel<'a> {
        Sentinel {
            shared_data: shared_data,
            active: true,
        }
    }

    fn cancel(mut self) { self.active = false; }
}

impl<'a> Drop for Sentinel<'a> {
    fn drop(&mut self) {
        if self.active {
            // -1, 将当前工作线程正常归还到线程池中
            self.shared_data.active_count.fetch_sub(1, Ordering::SeqCst);

            // 判断当前工作线程是否由于发生恐慌而退出
            if thread::panicking() {
                self.shared_data.panic_count.fetch_add(1, Ordering::SeqCst);
            }

            self.shared_data.no_work_notify_all();
            spawn_in_pool(self.shared_data.clone());
        }
    }
}

fn main() {
    let pool = ThreadPool::new(8);
    let test_count = Arc::new(AtomicUsize::new(0));

    for _ in 0..42 {
        let test_count = test_count.clone();

        pool.execute(move || {
            test_count.fetch_add(1, Ordering::Relaxed);
        });
    }

    pool.join();
    assert_eq!(42, test_count.load(Ordering::Relaxed));
}
