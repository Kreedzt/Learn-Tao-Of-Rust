fn foo(x: u32) {
    let y = x;
    let z = 100;
}

// 以 Rust 中的结构体为例验证结构体对齐规则
struct A {
    a: u8, // 1
    b: u32, // 4
    c: u16 // 2
}

// 额外的对齐示例: 对于 u8 情况下的内存对齐情况描述
// #[repr(align(8))]
struct A1 {
    // u8 是最小内存分配单元, 如果需要补齐, 则使用 #[repr(align(8))]
    a: [u8; 6],
    b: u8
}

struct B1 {
    a: [u8; 5],
    b: u16
}

// 联合体字节对齐
union U {
    f1: u32,
    f2: f32,
    f3: f64
}

fn main() {
    // 通过简单函数调用展示栈帧
    // main 函数现在栈里开辟了空间, 压入了 x 变量. 栈帧里 EBP 指向起始位置, 变量 x 保存在 EBP-4 偏移处
    let x = 42;
    // 调用 foo 函数时, 将返回地址压入栈中, 然后由 PC 指针(程序计数器)引导执行函数调用指令, 进入 foo 函数栈帧中.
    // 此时通用在栈中开辟空间, 依次将 `main` 函数的 EBP 地址, 参数 x 以及局部变量 y 和 z 压入栈中.
    // EBP 指针依旧指向地址为 0 的固定位置, 表明当前是在 foo 函数栈帧中.
    // 通过 EBP-4, EBP-8 和 EBP-12 就可以访问参数和遍历. 当 foo 函数执行完毕时, 其参数或局部变量会依次弹出
    // 知道得到 main 函数的 EBP 地址, 就可以调回 main 函数栈帧中, 然后通过返回的地址就可以继续执行 main 函数中其余的代码
    foo(x);

    // 以 Rust 中的结构体为例验证结构体对齐规则
    // 定义时: 7, 打印输出: 8
    println!("{:?}", std::mem::size_of::<A>());

    // 额外的对齐示例: 对于 u8 情况下的内存对齐情况描述
    println!("{:?}", std::mem::size_of::<A1>());
    println!("{:?}", std::mem::size_of::<B1>());

    // 联合体字节对齐
    println!("{:?}", std::mem::size_of::<U>());
}
