#![feature(allocator_api, dropck_eyepatch)]
use std::alloc::{GlobalAlloc, Layout, System};
use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::mem::transmute;
use std::mem::ManuallyDrop;
use std::ptr;
use std::ptr::{null, NonNull};
// use std::vec::Vec;

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
// fn foo<'a>(input: *const u32) -> &'a u32 { unsafe { return &*input } }


// 13-23 声明元组变量测试 dropck
// #[derive(Copy, Clone, Debug)]
// enum State {
//     InValid,
//     Valid,
// }

// #[derive(Debug)]
// struct Hello<T: fmt::Debug>(&'static str, T, State);

// impl<T: fmt::Debug> Hello<T> {
//     fn new(name: &'static str, t: T) -> Self { Hello(name, t, State::Valid) }
// }

// impl<T: fmt::Debug> Drop for Hello<T> {
//     fn drop(&mut self) {
//         println!("drop Hello({}, {:?}, {:?})", self.0, self.1, self.2);
//         self.2 = State::InValid;
//     }
// }

// struct WrapBox<T> {
//     v: Box<T>,
// }

// impl<T> WrapBox<T> {
//     fn new(t: T) -> Self { WrapBox { v: Box::new(t) } }
// }

// fn f1() {
//     let (x, y);
//     // 交换顺序报错
//     // let (y, x);
//     x = Hello::new("x", 13);
//     // Rust 1.50 Nightly 下没有报错
//     y = WrapBox::new(Hello::new("y", &x));
// }


// 13-26 使用原生指针的结构体
// struct MyBox<T> {
//     v: *const T,
// }

// impl<T> MyBox<T> {
//     fn new(t: T) -> Self {
//         unsafe {
//             let p = System.alloc(Layout::array::<T>(1).unwrap());
//             let p = p as *mut T;
//             ptr::write(p, t);
//             MyBox { v: p }
//         }
//     }
// }

// impl<T> Drop for MyBox<T> {
//     fn drop(&mut self) {
//         unsafe {
//             println!("MyBox drop");
//             let p = self.v as *mut _;
//             System.dealloc(p, Layout::array::<T>(mem::align_of::<T>()).unwrap());
//         }
//     }
// }


// 13-28 修改 drop 方法
// unsafe impl<#[may_dangle] T> Drop for MyBox<T> {
//     fn drop(&mut self) {
//         unsafe {
//             ptr::read(self.v); // 此处新增
//             println!("MyBox drop");
//             let p = self.v as *mut _;
//             System.dealloc(p, Layout::array::<T>(mem::align_of::<T>()).unwrap());
//         }
//     }
// }

// fn f2() {
//     {
//         let (x1, y1);
//         x1 = Hello::new("x1", 13);
//         y1 = MyBox::new(Hello::new("y1", &x1));
//     }
//     {
//         // let (x2, y2);
//         // 13-29 修改顺序: 出现问题: 产生悬垂指针
//         let (y2, x2);
//         x2 = Hello::new("x2", 13);
//         y2 = MyBox::new(Hello::new("y2", &x2));
//     }
// }


// 13-31 新增 `MyBox2<T>`
// struct MyBox2<T> {
//     v: *const T,
//     // 表明: `Mybox2<T>` 拥有 `T`
//     _pd: PhantomData<T>,
// }

// impl<T> MyBox2<T> {
//     fn new(t: T) -> Self {
//         unsafe {
//             let p = System.alloc(Layout::array::<T>(1).unwrap());
//             let p = p as *mut T;
//             ptr::write(p, t);
//             MyBox2 {
//                 v: p,
//                 _pd: Default::default(),
//             }
//         }
//     }
// }

// unsafe impl<#[may_dangle] T> Drop for MyBox2<T> {
//     fn drop(&mut self) {
//         unsafe {
//             ptr::read(self.v);
//             let p = self.v as *mut _;
//             System.dealloc(p, Layout::array::<T>(mem::align_of::<T>()).unwrap());
//         }
//     }
// }

// fn f3() {
//     // let (y, x); // 正常触发编译报错
//     let (x, y);
//     // let x;
//     // let y;
//     x = Hello::new("x", 13);
//     y = MyBox2::new(Hello::new("y", &x));
// }


// 13-33 转移结构体中字段所有权示例
// struct A;
// struct B;
// struct Foo {
//     a: A,
//     b: B,
// }

// impl Foo {
//     fn take(self) -> (A, B) {
//         // 转移所有权但不实现 `drop`: 允许的操作
//         // error[E0509]: cannot move out of type `Foo`, which implements the `Drop` trait
//         (self.a, self.b)
//     }
// }

// impl Drop for Foo {
//     fn drop(&mut self) {
//         // ..do something
//     }
// }

// 13-36 重新为 `Foo` 实现 `take` 方法
// impl Foo {
//     fn take(mut self) -> (A, B) {
//         let a = mem::replace(&mut self.a, unsafe { mem::uninitialized() });
//         let b = mem::replace(&mut self.b, unsafe { mem::uninitialized() });
//         mem::forget(self);
//         (a, b)
//     }
// }

// impl Drop for Foo {
//     fn drop(&mut self) {
//         // ..do something
//     }
// }


// 13-37 `ManualDrop` 使用示例
// struct Peach;
// struct Banana;
// struct Melon;
// struct FruitBox {
//     peach: ManuallyDrop<Peach>,
//     melon: Melon,
//     banana: ManuallyDrop<Banana>,
// }

// impl Drop for FruitBox {
//     fn drop(&mut self) {
//         unsafe {
//             // 显式指定析构顺序
//             ManuallyDrop::drop(&mut self.peach);
//             ManuallyDrop::drop(&mut self.banana);
//         }
//     }
// }


// 13-42 空指针优化展示
// struct Foo {
//     a: *mut u64,
//     b: *mut u64,
// }

// struct FooUsingNonNull {
//     a: *mut u64,
//     b: NonNull<*mut u64>,
// }


// 13-44 Unsafe Rust 中需要注意恐慌安全问题
// error[E0116]: cannot define inherent `impl` for a type outside of the crate where the type is defined
// impl<T: Clone> Vec<T> {
//     fn push_all(&mut self, to_push: &[T]) {
//         self.reserve(to_push.len());

//         unsafe {
//             self.set_len(self.len() + to_push.len());

//             for (i, x) in to_push.iter().enumerate() {
//                 // `clone()` 方法存在恐慌的可能. 所以整个函数就不是恐慌的安全函数, 它也不保证内存安全
//                 self.ptr().offset(i as isize).write(x.clone());
//             }
//         }
//     }
// }


// 13-46 自定义全局分配器示例
struct MyAllocator;
unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 { System.alloc(layout) }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) { System.dealloc(ptr, layout) }
}

#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;

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
    // let x: &i32;
    // {
    //     let a = 12;
    //     let ptr = &a as *const i32;
    //     // 该函数可以将类型 `T` 转为类型 `U`
    //     // 这是一个 unsafe 函数, 使用不当会产生未定义行为
    //     x = unsafe { transmute::<*const i32, &i32>(ptr) };
    // } // 离开作用域后, x 依旧产生悬垂指针
    // println!("hello {}", x);


    // 13-23
    // f1();


    // 13-26
    // f2();


    // 13-31
    // f3();


    // 13-33 (补充)
    // let a = Foo { a: A, b: B };
    // let b = a.take();


    // 13-41 `NonNull<T>` 内置方法示例
    // 创建悬垂指针
    // let ptr: NonNull<i32> = NonNull::dangling();
    // println!("{:p}", ptr); // 0x4

    // let mut v = 42;
    // // 将可变的原生指针生成 `Option<NonNull<i32>>` 类型
    // let ptr: Option<NonNull<i32>> = NonNull::new(&mut v);
    // println!("{:?}", ptr); // Some(0x7ffee085d834)
    // // `*mut T` 指针
    // println!("{:?}", ptr.unwrap().as_ptr()); // 0x7ffee085d834
    // // `&mut T` 引用
    // // 注意: 此处的 `as_mut()~ 方法得到的引用是有正常生命周期的引用, 而非未绑定生命周期的引用
    // println!("{}", unsafe { ptr.unwrap().as_mut() }); // 42

    // let mut v = 42;
    // let ptr = NonNull::from(&mut v);
    // println!("{:?}", ptr); // 0x7ffee085d934
    // let null_p: *const i32 = null();
    // let ptr = NonNull::new(null_p as *mut i32);
    // println!("{:?}", ptr); // None


    // 13-42 空指针优化展示
    // println!("*mut u64: {} bytes", mem::size_of::<*mut u64>()); // 8
    // println!(
    //     "NonNull<*mut u64>: {} bytes",
    //     mem::size_of::<NonNull<&mut u64>>()
    // ); // 8
    // println!(
    //     "Option<*mut u64>: {} bytes",
    //     mem::size_of::<Option<*mut u64>>()
    // ); // 16
    // println!(
    //     "Option<NonNull<*mut u64>>: {} bytes",
    //     mem::size_of::<Option<NonNull<*mut u64>>>()
    // ); // 8
    // println!("Option<Foo>: {} bytes", mem::size_of::<Option<Foo>>()); // 24
    // println!(
    //     "Option<FooUsingNonNull>: {} bytes",
    //     mem::size_of::<Option<FooUsingNonNull>>()
    // ); // 16


    // 13-46
    // 此处 Vec 的内存会由 GLOBAL 全局分配器来分配
    let mut v = Vec::new();
    v.push(1);
}
