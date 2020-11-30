#![feature(untagged_unions)]

static mut COUNTER: u32 = 0;

// 13-7 使用 Union 联合体和 Struct 模拟 Enum 类型
// 该属性表明使用和 C 语言一样的内存布局
// #[repr(C)]
// union U {
//     i: i32,
//     f: f32,
// }

// #[repr(C)]
// struct Value {
//     tag: u8,
//     value: U,
// }

// #[repr(C)]
// union MyZero {
//     // Safe Rust 默认不支持 Union 联合体的字段为非 Copy 类型
//     i: Value,
//     f: Value,
// }

// enum MyEnumZero {
//     I(i32),
//     F(f32),
// }


// 13-8 重构
#[repr(u32)]
enum Tag {
    I,
    F,
}


#[repr(C)]
union U {
    i: i32,
    f: f32,
}

#[repr(C)]
struct Value {
    tag: Tag,
    u: U,
}

fn is_zero(v: Value) -> bool {
    // 对联合体的字段进行操作是不安全的行为
    unsafe {
        match v {
            Value {
                tag: Tag::I,
                u: U { i: 0 },
            } => true,
            Value {
                tag: Tag::F,
                // warning: floating-point types cannot be used in patterns
                u: U { f: 0.0 },
            } => true,
            _ => false,
        }
    }
}

fn main() {
    // // 13-1 unsafe 块中使用引用依旧会进行借用检查
    // // 因为 NLL 的特性, 以下代码现不会报错
    // // https://github.com/ZhangHanDong/tao-of-rust-codes/issues/296
    // unsafe {
    //     let mut a = "hello";
    //     let b = &a;
    //     let c = &mut a;
    // }

    // // 规避 NLL
    // unsafe {
    //     let mut a = "hello";
    //     let b = &a;
    //     let c = &mut a;
    //     b;
    // }


    // // 13-4 unsafe 块示意
    // let hello = vec![104, 101, 108, 108, 111];
    // let hello = unsafe { String::from_utf8_unchecked(hello) };
    // // error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
    // // let hello = String::from_utf8_unchecked(hello);
    // assert_eq!("hello", hello);


    // // 13-6 访问和修改可变静态变量必须在 unsafe 块中
    // let inc = 3;
    // unsafe {
    //     COUNTER += inc;
    //     println!("COUNTER: {}", COUNTER);
    // }


    // 13-7
    // let int_0 = MyZero {
    //     i: Value {
    //         tag: b'0',
    //         value: U { i: 0 },
    //     },
    // };
    // let float_0 = MyZero {
    //     i: Value {
    //         tag: b'1',
    //         value: U { f: 0.0 },
    //     },
    // };


    // 13-8
    // let int_0 = Value {
    //     tag: Tag::I,
    //     u: U { i: 0 },
    // };
    // let float_0 = Value {
    //     tag: Tag::I,
    //     u: U { f: 0.0 },
    // };
    // assert_eq!(true, is_zero(int_0));
    // assert_eq!(true, is_zero(float_0));
    // assert_eq!(4, std::mem::size_of::<U>());


    // 13-9 访问联合体中未初始化的字段
    let u = U { i: 1 };
    let i = unsafe { u.f };

    // 0.000000000000000000000000000000000000000000001
    println!("{}", i);
    // 等价于此调用: f32::from_bites(1)

    // 对于联合体来说, 不能同时使用两个字段, 也不可能同时借出 2 个字段的可变借用
    // unsafe {
    //     let i = &mut u.i;
    //     let f = &mut u.f;
    // };


    // 13-10 解引用裸指针是不安全行为
    let mut s = "hello".to_string();
    // 注意: 同时出现了不可变和可变的指针
    let r1 = &s as *const String;
    let r2 = &mut s as *mut String;

    assert_eq!(r1, r2);

    // 必须指明类型, 默认识别的 i32 无法编译通过
    // https://github.com/ZhangHanDong/tao-of-rust-codes/issues/310
    let address: u64 = 0x7fff1d72307d;
    let r3 = address as *const String;
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        // 段错误(C 语言常见错误)
        // [1]    12162 segmentation fault  cargo run
        // assert_eq!(*r1, *r3);
    }
}
