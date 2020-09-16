use std::fmt::Debug;

// 无输入参数且返回引用的函数
// fn return_str<'a>() -> &'a str {
//     // 离开当前函数会被析构
//     let mut s = "Rust".to_string();

//     for i in 0..3 {
//         s.push_str("Good ");
//     }

//     // 违反了规则一, 制造了悬垂指针
//     // error[E0515]: cannot return value referencing local variable `s`
//     &s[..]
// }

// 函数的引用参数和返回的引用生命周期毫无关联
// 此时编译器拥有对函数的全部信息, 生命周期标准完全没有派上用场
// fn foo<'a>(x: &'a str, y: &'a str) -> &'a str {
//     let result = String::from("really long string");
//     // error[E0515]: cannot return value referencing local variable `result`
//     result.as_str()
// }

// 需要进行生命周期标注的示例
// error[E0106]: missing lifetime specifier
// 函数声明中的 `'a` 可以看做一个生命周期泛型参数, 输入引用和输出引用都标记为 `'a`
// 意味着输出引用(借用方)的生命周期不长于输入引用(出借方)的生命周期
// fn the_longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
//     if s1.len() > s2.len() { s1 } else { s2 }
// }

// 标注多个生命周期参数
// error[E0623]: lifetime mismatch
// fn the_longest<'a, 'b>(s1: &'a str, s2: &'b str) -> &'a str {
//     if s1.len() > s2.len() { s1 } else { s2 }
// }

// 'b:'a, 借用方的生命周期参数不长于出借方的生命周期参数
// 此处返回生命周期若改为 `'b` 必定报错
fn the_longest<'a, 'b:'a>(s1: &'a str, s2: &'b str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

// 结构体定义中的生命周期参数
#[derive(Debug)]
struct Foo<'a> {
    part: &'a str
}

// 为结构体 `Foo` 实现方法
// `'a` 在整个 `impl` 块中适用
impl<'a> Foo<'a> {
    fn split_first(s: &str) -> &str {
        s.split(',').next().expect("Could not find a ','")
    }

    // fn split_first(s: &'a str) -> &'a str {
    //     s.split(',').next().expect("Could not find a ','")
    // }

    // error[E0621]: explicit lifetime required in the type of `s`
    // fn new(s: &str) -> Self {
    //     Foo {
    //         part: Foo::split_first(s)
    //     }
    // }

    fn new(s: &'a str) -> Self {
        Foo {
            part: Foo::split_first(s)
        }
    }
}

// 省略生命周期参数的示例
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

type Command = u32;
type ToCStr = u64;

// 各类函数签名生命周期省略或非法示例
trait Lifetime<T> {
    // 只有一个引用类型, 满足第二条
    fn print(s: &str);
    // fn print<'a>(s: &'a str); // 展开
    fn debug(lvl: u64, s: &str);
    // fn debug<'a>(lvl: u64, s: &str); // 展开
    fn substr(s: &str, until: u64) -> &str;
    // fn substr<'a>(s: &'a str, until: u64) -> &'a str; // 展开
    
    // 没有任何参数, 不满足任何一条规则
    // fn get_str() -> &str; // 非法
    
    // 两个引用参数, 也就是拥有 2 个生命周期参数的位置, 分别补齐两个不同的生命周期参数, 满足第一条, 但是不满足其他
    // 也不存在其他规则来帮助编译器推断生命周期
    // fn frob(s: &str, t: &str) -> &str; // 非法
    
    fn get_mut(&mut self) -> &mut T;
    // fn get_mut<'a>(&'a mut self) -> &'a mut T; // 展开

    // 两个引用参数代表两个生命周期参数的位置
    // 但是其中之一是 `&mut self`, 满足第三条, 返回引用的生命周期会指派为 `self` 的生命周期
    fn args(&mut self, args: &[T]) -> &u64;
    // fn args<'a, 'b>(&'a mut self, args: &'b [T]) -> &'a u64; // 展开
    
    fn new(buf: &mut [u8]) -> &u8;
    // fn new<'a>(buf: &'a mut [u8]) -> &'a u8; // 展开
}


// 添加新方法
impl<'a> Foo<'a> {
    fn get_part(&self) -> &str {
        self.part
    }
}

// 生命周期限定示例
#[derive(Debug)]
// 定义一个元组结构体, 用于保存泛型类型 T 的引用, 但是却不知道该引用类型的生命周期
// 使用 `T: 'a` 来对类型 T 进行生命周期限定
// struct Ref<'a, T: 'a>(&'a T);
// 在 Rust 2018 中, 可省略 `:'a`
struct Ref<'a, T>(&'a T);

// 泛型参数
fn print<T>(t: T)
where
    T: Debug,
{
    println!("`print`: t is {:?}", t);
}

// 泛型参数
fn print_ref<'a, T>(t: &'a T)
where
    T: Debug + 'a,
{
    println!("`print_ref`: t is {:?}", t);
}

// trait 对象中实现 trait 的类型带有生命周期参数
trait FooT {}
struct Bar<'a> {
    x: &'a i32,
}

impl <'a> FooT for Bar<'a> {}

// 需要明确指定 trait 对象生命周期的示例
trait FooC<'a> {}
struct FooImpl<'a> {
    s: &'a [u32],
}
impl<'a> FooC<'a> for FooImpl<'a> {
    
}

// `Box<FooC<'a>>` 是一个 trait 对象, 默认生命周期是 `'static`
// 但 s 的类型为 `&'a [u32]` 所以此时的 trait 生命周期对象应该是 `'a`
// fn foo<'a>(s: &'a [u32]) -> Box<FooC<'a>> {
//     // error[E0759]: `s` has lifetime `'a` but it needs to satisfy a `'static` lifetime requirement
//     Box::new(FooImpl { s })
// }

// 修复后
// `'a` 覆盖了默认的 `'static` 生命周期
fn foo<'a>(s: &'a [u32]) -> Box<FooC<'a> + 'a> {
    // error[E0759]: `s` has lifetime `'a` but it needs to satisfy a `'static` lifetime requirement
    Box::new(FooImpl { s })
}

fn main() {
    // 借用检查示例
    // let r;
    // {
    //     let x = 5;
    //     // 生命周期冲突
    //     // 离开作用域后, x 会被销毁, 产生悬垂指针
    //     r = &x;
    // }
    // println!("r: {}", r);

    // 调用: 无输入参数且返回引用的函数
    // let x = return_str();

    // 调用: 函数的引用参数和返回的引用生命周期毫无关联
    // let x = "hello";
    // let y = "rust";
    // foo(x, y);

    let s1 = String::from("Rust");
    let s1_r = &s1;
    {
        let s2 = String::from("C");
        //                 't
        // 'r                's1   's2
        let res = the_longest(s1_r, &s2);
        // 参数顺序并不影响
        let res = the_longest(&s2, s1_r);
        println!("{} is the longest", res);
    }

    // 标注生命周期参数的结构体
    let words = String::from("Sometimes think, the greatest sorrow than older");
    let first = words.split(',').next().expect("Could not find a ','");
    let f = Foo {
        part: first
    };
    assert_eq!("Sometimes think", f.part);

    // 为结构体 `Foo` 实现方法后
    let words = String::from("Sometimes think, the greatest sorrow than older");
    println!("{:?}", Foo::new(words.as_str()));

    // 字符串字面量生命周期
    let x = "hello Rust";
    let y = x;
    assert_eq!(x, y);

    // 省略生命周期参数的示例
    println!("{:?}", first_word("hello Rust"));

    // 调用: 添加 `get_part()`
    let words = String::from("Sometimes think, the greatest sorrow than older");
    let foo = Foo::new(words.as_str());
    println!("{:?}", foo.get_part());

    // 生命周期限定示例
    let x = 7;
    let ref_x = Ref(&x);
    print_ref(&ref_x);
    print(ref_x);

    // trait 对象中实现 trait 的类型带有生命周期参数
    let num = 5;
    let box_bar = Box::new(Bar { x: & num });
    let obj = box_bar as Box<FooT>;
}
