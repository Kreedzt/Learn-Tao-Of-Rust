use crypto::digest::Digest;
use crypto::sha2::Sha256;
use itertools::Itertools;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;

// 基础值
const BASE: usize = 42;
// 线程数
const THREADS: usize = 8;

// 更改 0 的位数调整难度, 越多难度越高
static DIFFICULTY: &'static str = "00000";

// 记录最终找到的数字及其加密后的结果
struct Solution(usize, String);

fn main() {
    println!(
        "PoW : Find a number,
SHA256(the number * {}) == \"{}......\" ",
        BASE, DIFFICULTY
    );

    println!("Started {} threads", THREADS);
    println!("Please wait... ");
    let is_solution_found = Arc::new(AtomicBool::new(false));

    let (sender, recevicer) = mpsc::channel();

    for i in 0..THREADS {
        let sender_n = sender.clone();
        let is_solution_found = is_solution_found.clone();
        thread::spawn(move || {
            find(i, sender_n, is_solution_found);
        });
    }

    match recevicer.recv() {
        Ok(Solution(i, hash)) => {
            println!("Found the solution: ");
            println!(
                "The number is : {},
and hash result is : {}.",
                i, hash
            );
        }
        Err(_) => panic!("Worker threads disconnected!"),
    }
}

// 验证函数
fn verify(number: usize) -> Option<Solution> {
    let mut hasher = Sha256::new();
    hasher.input_str(&(number * BASE).to_string());
    let hash: String = hasher.result_str();
    if hash.starts_with(DIFFICULTY) {
        Some(Solution(number, hash))
    } else {
        None
    }
}

// 查找函数
fn find(start_at: usize, sender: mpsc::Sender<Solution>, is_solution_found: Arc<AtomicBool>) {
    // 此处是无限递增的循环
    // 线程数作为步长, 是为了将查找的自然数进行分组, 以便于平均划分多线程任务.
    // eg: Thread1: 0, 8, 16 ...
    // eg: Thread2: 1, 9, 17 ...
    for number in (start_at..).step(THREADS) {
        // 此处自由顺序是安全的, 因为底层的线程执行顺序并不会影响结果
        if is_solution_found.load(Ordering::Relaxed) {
            return;
        }
        if let Some(solution) = verify(number) {
            is_solution_found.store(true, Ordering::Relaxed);
            // 发送出去
            sender.send(solution).unwrap();
            return;
        }
    }
}
