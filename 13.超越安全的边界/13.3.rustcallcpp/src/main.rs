// 13-57
// 表示和 Rust 链接的名为 libsorting 的静态库
// 该属性也可以省略, Rust 会使用默认生成的名字
// 这个属性主要用于在需要的时候指定链接库的名字
#[link(name = "sorting", kind = "static")]
extern "C" {
    // 数组 C++ 实际上是指针
    fn interop_sort(arr: &[i32; 10], n: u32);
}

pub fn sort_from_cpp(arr: &[i32; 10], n: u32) {
    unsafe {
        interop_sort(arr, n);
    }
}


fn main() {
    let my_arr: [i32; 10] = [10, 42, -9, 12, 8, 25, 7, 123, 55, -1];
    println!("Before sorting...");
    println!("{:?}\n", my_arr);
    sort_from_cpp(&my_arr, 10);
    println!("After sorting...");
    println!("{:?}", my_arr);
}
