use crossbeam::thread::scope;
use crossbeam::channel as channel;
use crossbeam::channel::select;
use std::thread;


// 11-61 使用 Crossbeam 提供的 `select!` 宏
fn fibonacci (
    fib: channel::Sender<u64>,
    quit: channel::Receiver<()>
) {
    let (mut x, mut y) = (0, 1);

    loop {
        select! {
            send(fib, x) -> msg => {
                let tmp = x;
                x = y;
                y = tmp + y;
            },
            recv(quit) -> msg => {
                println!("quit");
                return;
            }
        }
    }
}

fn main() {
    // 11-58 父线程中的引用无法在子线程中安全地使用
    // let array = [1, 2, 3];
    // let mut guards = vec![];

    // 需要 &i 来解构引用得到数组中的值, 才能在子线程中被安全地使用
    // 在子线程中无法完全地使用父线程的引用
    // for &i in &array {
    //     let guard = std::thread::spawn(move || {
    //         println!("element: {}", i);
    //     });

    //     guards.push(guard);
    // }

    // for guard in guards {
    //     guard.join().unwrap();
    // }

    
    // 11-59 使用 Crossbeam 提供的 Scoped 线程
    // let array = [1, 2, 3];
    // scope(|scope| {
    //     // 在闭包中由 scope 参数来生成子线程, 其可以安全地使用父线程(main 线程) 中 array 数组的元素引用
    //     for i in &array {
    //         // 警告: 新版必须传递参数
    //         scope.spawn(move |_| { println!("element: {}", i) });
    //     }
    // });

    
    // 11-60 使用 Crossbeam 提供的 MPMC Channel
    // 使用该函数来创建 `无界通道`
    // let (s, r) = channel::unbounded();
    // crossbeam::scope(|scope| {
    //     scope.spawn(|_| {
    //         s.send(1);
    //         r.recv().unwrap();
    //     });

    //     scope.spawn(|_| {
    //         s.send(2);
    //         r.recv().unwrap();
    //     });
    // });


    // 11-61 使用 Crossbeam 提供的 `select!` 宏
    // 0 容量通道, 会一直阻塞, 除非接收端可以对其进行操作
    let (fib_s, fib_r) = channel::bounded(0);
    let (quit_s, quit_r) = channel::bounded(0);

    thread::spawn(move || {
        for _ in 0..10 {
            // 在 for 循环过程中, 在 `fibonacci()` 函数的 `select!` 宏中只有 `send()` 操作准备就绪
            // 所以 `fibonacci()` 函数不需要担心突然收到 `quit` 消息而意外退出
            println!("{}", fib_r.recv().unwrap());
        }

        // for 循环执行完毕后, 发送消息让 `fibonacci` 函数退出
        quit_s.send(());
    });

    fibonacci(fib_s, quit_r);
}
