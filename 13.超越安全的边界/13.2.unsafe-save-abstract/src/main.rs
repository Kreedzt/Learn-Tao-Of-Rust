use std::marker::PhantomData;
use std::mem::transmute;

// 13-16 自定义内部可变类型 `MyCell<T>`
// struct MyCell<T> {
//     value: T,
// }

// impl<T: Copy> MyCell<T> {
//     fn new(x: T) -> MyCell<T> { MyCell { value: x } }

//     fn get(&self) -> T { self.value }

//     fn set(&self, value: T) {
//         use std::ptr;
//         unsafe {
//             // 转为不可变裸指针 -> 转为可变裸指针
//             ptr::write(&self.value as *const _ as *mut _, value);
//         }
//     }
// }


// 13-18 利用 `PhantomData<T>` 修改 `MyCell<T>` 为不变
// struct MyCell<T> {
//     value: T,
//     // 定义 `逆变` 类型: 因 `fn(T)` 指针类型在 Rust 中是逆变, 未来的 Rust 版本中可能会修改为 `不变`
//     mark: PhantomData<fn(T)>
// }

// impl<T: Copy> MyCell<T> {
//     fn new(x: T) -> MyCell<T> { MyCell { value: x, mark: PhantomData } }

//     fn get(&self) -> T { self.value }

//     fn set(&self, value: T) {
//         use std::ptr;
//         unsafe {
//             // 转为不可变裸指针 -> 转为可变裸指针
//             ptr::write(&self.value as *const _ as *mut _, value);
//         }
//     }
// }

// 13-17: 使用 `MyCell<T>` 示例
// fn step1<'a>(r_c1: &MyCell<&'a i32>) {
//     let val: i32 = 13;
//     // 设置值
//     // 13-18: error[E0597]: `val` does not live long enough
//     step2(&val, r_c1);
//     println!("step1 value: {}", r_c1.value);
//     // val 结束时会被清理, 会导致悬垂指针
// }

// fn step2<'b>(r_val: &'b i32, r_c2: &MyCell<&'b i32>) { r_c2.set(r_val); }

// static X: i32 = 10;


// 13-19: `fn(T)` 的逆变示例
// trait A {
//     fn foo(&self, s: &'static str);
// }

// struct B;

// impl A for B {
//     // 逆变
//     fn foo(&self, s: &str) {
//         println!("{:?}", s);
//     }
// }

// impl B {
//     // 不变
//     fn foo2(&self, s: &'static str) {
//         println!("{:?}", s);
//     }
// }


// 13-20: 另一个 `fn(T)` 逆变示例
// fn foo(input: &str) {
//     println!("{:?}", input);
// }

// // &'static str 是 &str 的子类型
// // &'static str <: &str
// // fn(&str) <: fn(&'static str) 逆变
// fn bar(f: fn(&'static str), v: &'static str) {
//     (f)(v);
// }


// 13-21 从原生指针得到引用
fn foo<'a>(input: *const u32) -> &'a u32 { unsafe { return &*input } }

fn main() {
    // 13-11 创建空指针并判断是否为空指针
    // let p: *const u8 = std::ptr::null();
    // assert!(p.is_null());

    // let s: &str = "hello";
    // // 因字符串是以字节为单位存储的, 所以指针的类型为 `*const u8`.
    // let ptr: *const u8 = s.as_ptr();
    // assert!(!ptr.is_null());

    // let mut s = [1, 2, 3];
    // let ptr: *mut u32 = s.as_mut_ptr();
    // assert!(!ptr.is_null());

    // 13-12 使用 offset 方法
    // let s: &str = "Rust";
    // let ptr: *const u8 = s.as_ptr();
    // unsafe {
    //     // `offset` 是 unsafe 方法, 需要在 unsafe 块中使用
    //     // 优先级: *(ptr.offset(1)) as char
    //     println!("{:?}", *ptr.offset(1) as char); // u
    //     println!("{:?}", *ptr.offset(3) as char); // t
    //     println!("{:?}", *ptr.offset(255) as char); // 不可预料
    // }

    // 13-13 使用 read/wrote 方法
    // let x = "hello".to_string();
    // let y: *const u8 = x.as_ptr();
    // unsafe {
    //     assert_eq!(y.read() as char, 'h');
    // }

    // let x = [0, 1, 2, 3];
    // // 注意: 此处的原生指针类型是带长度的
    // let y = x[0..].as_ptr() as *const [u32; 4];
    // unsafe {
    //     assert_eq!(y.read(), [0, 1, 2, 3]);
    // }

    // let mut x = "";
    // let y = &mut x as *mut &str;
    // let z = "hello";
    // unsafe {
    //     y.write(z);
    //     assert_eq!(y.read(), "hello");
    // }

    // 13-14 使用 replace/swap 方法
    // let mut v: Vec<i32> = vec![1, 2];
    // // 指向第一个元素(i32 类型)
    // let v_ptr: *mut i32 = v.as_mut_ptr();

    // unsafe {
    //     // 替换第一个值
    //     let old_v = v_ptr.replace(5);
    //     assert_eq!(1, old_v);
    //     assert_eq!([5, 2], &v[..]);
    // }

    // let mut v: Vec<i32> = vec![1, 2];
    // // 指向全部元素 Vec<i32>
    // let v_ptr = &mut v as *mut Vec<i32>;

    // unsafe {
    //     // 替换全部元素
    //     let old_v = v_ptr.replace(vec![3, 4, 5]);
    //     assert_eq!([1, 2], &old_v[..]);
    //     assert_eq!([3, 4, 5], &v[..]);
    // }

    // let mut array = [0, 1, 2, 3];
    // let x = array[0..].as_mut_ptr() as *mut [u32; 2];
    // let y = array[1..].as_mut_ptr() as *mut [u32; 2];

    // unsafe {
    //     assert_eq!([0, 1], x.read());
    //     assert_eq!([1, 2], y.read());
    //     // 操作内存区域重叠, 可能引起内部数据混乱
    //     x.swap(y);
    //     assert_eq!([1, 0, 1, 3], array);
    // }


    // 13-17 使用 `MyCell<T>` 示例
    // let cell = MyCell::new(&X);
    // step1(&cell);
    // println!(" end value: {}", cell.value);


    // 13-19
    // B.foo("hello");
    // let s = "hello".to_string();
    // B.foo2(&s);


    // 13-20
    // let v: &'static str = "hello";
    // bar(foo, v);


    // 13-21
    // let x;
    // {
    //     let y = 42;
    //     // 悬垂指针
    //     // foo 函数产生了一个未绑定生命周期的借用, 所以跳过了借用检查
    //     x = foo(&y);
    // }
    // println!("hello: {}", x);


    // 13-22 使用 `transmute` 函数得到引用
    let x: &i32;
    {
        let a = 12;
        let ptr = &a as *const i32;
        // 该函数可以将类型 `T` 转为类型 `U`
        // 这是一个 unsafe 函数, 使用不当会产生未定义行为
        x = unsafe { transmute::<*const i32, &i32>(ptr) };
    } // 离开作用域后, x 依旧产生悬垂指针
    println!("hello {}", x);
}
