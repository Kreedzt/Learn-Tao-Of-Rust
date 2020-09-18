use std::thread;

fn main() {
    // 线程不安全示例
    // 所有权已被转移依旧在使用
    // error[E0382]: use of moved value: `data`
    let mut data = vec![1, 2, 3];
    for i in 0..3 {
        thread::spawn(move || {
            data[i] += 1;
        });
    }

    thread::sleep_ms(50);
}
