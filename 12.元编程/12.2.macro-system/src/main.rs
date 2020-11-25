#![feature(trace_macros)]

// 12-12/12-22 定义 `unless!` 宏
macro_rules! unless {
    ($arg: expr, $branch: expr) => {
        if !$arg {
            $branch
        };
    };
}

// fn cmp(a: i32, b: i32) {
//     unless!(a > b, {
//         println!("{} < {}", a, b);
//     });
// }


// 12-13 使用自定义派生属性示例
// #[derive(new)]
// pub struct Foo;


// 12-15 普通函数示例
// fn t(i: i32) -> i32 {
//     i + 2
// }


// 12-26 hashmap! 宏的实现
// macro_rules! hashmap {
//     // 该模式在处理最后一行键值对的时候, 只能匹配没有逗号结尾的情况
//     // 注意: 此匹配有逗号
//     ($($key: expr => $value: expr), *) => {
//         {
//             // 绝对路径的 `HashMap`, 避免冲突
//             let mut _map = ::std::collections::HashMap::new();
//         $(
//             _map.insert($key, $value);
//         )*
//                 _map
//         }
//     }
// }


// 12-28 `hashmap!` 递归调用消去最后键值对的结尾逗号
// macro_rules! hashmap {
//     // 递归替换 ("a" => 1, "b" => 2,) 为 ("a" => 1, "b" => 2)
//     ($($key: expr => $value: expr,) *) =>
//     { hashmap!($($key => $value), *)};
//     // 该模式在处理最后一行键值对的时候, 只能匹配没有逗号结尾的情况
//     // 注意: 此匹配有逗号
//     ($($key: expr => $value: expr), *) => {
//         {
//             // 绝对路径的 `HashMap`, 避免冲突
//             let mut _map = ::std::collections::HashMap::new();
//             $(
//                 _map.insert($key, $value);
//             )*
//                 _map
//         }
//     };
// }


// 12-29 利用重复匹配技巧来匹配结尾逗号
// macro_rules! hashmap {
//     ($($key: expr => $value:expr), * $(,)*) => {
//         {
//             let mut _map = ::std::collections::HashMap::new();
//             $(
//                 _map.insert($key, $value);
//             )*
//                 _map
//         }
//     }
// }


// 12-31 可根据键值对个数预分配的 `hashmap!` 宏
// macro_rules! unit {
//     ($($x:tt)*) => (());
// }

// macro_rules! count {
//     ($($key:expr), *) => (<[()]>::len(&[$(unit!($key)), *]));
// }

// macro_rules! hashmap {
//     ($($key:expr => $value:expr), * $(,)*) => {
//         {
//             let _cap = count!($($key), *);
//             let mut _map = ::std::collections::HashMap::with_capacity(_cap);
//             $(
//                 _map.insert($key, $value);
//             )*
//                 _map
//         }
//     }
// }


// 12-32 在 `hashmap!` 宏内部定义依赖宏
macro_rules! hashmap {
    // `@unit` 和 `@count` 相当于是内部宏规则的宏名, 暂且称为 *内部宏*
    (@unit $($x:tt)*) => (());
    (@count $($rest:expr), *) =>
        (<[()]>::len(&[$(hashmap!(@unit $rest)), *]));
    ($($key:expr => $value:expr), * $(,)*) => {
        {
            let _cap = hashmap!(@count $($key), *);
            let mut _map = ::std::collections::HashMap::with_capacity(_cap);
            $(
                _map.insert($key, $value);
            )*
                _map
        }
    }
}


// 12-35 展示声明宏的卫生性
macro_rules! sum {
    ($e: expr) => ({
        let a = 2;
        $e + a
    })
}

fn main() {
    // 12-12 定义 `unless!` 宏
    // let (a, b) = (1, 2);
    // cmp(a, b);


    // let x = Foo::new();
    // assert_eq!(x, Foo);

    // t(1);


    // 12-17 另外一个较复杂的示例
    // let (a, b, c, d, e) = (1, 2, 3, [4, 5], 6);
    // a + b + (c + d[0]) + e;


    // 12-21 unless 宏定义示例
    // let (a, b) = (1, 2);
    // unless!(a > b, {
    //     b - a;
    // });


    // 12-25 `hashmap!` 宏用法示意
    // let map = hashmap! {
    //     "a" => 1,
    //     "b" => 2
    //     // error: unexpected end of macro invocation
    //     // "b" => 2,
    // };


    // 12-28 `hashmap!` 递归调用消去最后键值对的结尾逗号
    // let map = hashmap! {
    //     "a" => 1,
    //     "b" => 2,
    // };

    // let map = hashmap! {
    //     "a" => 1,
    //     "b" => 2
    // };

    // assert_eq!(map["a"], 1);


    // 12-31 可根据键值对个数预分配的 `hashmap!` 宏
    // 调试宏
    // trace_macros!(true);
    // let map = hashmap!{
    //     "a" => 1,
    //     "b" => 2,
    // };

    // assert_eq!(map["a"], 1);


    // 12-35 展示声明宏的卫生性
    // error[E0425]: cannot find value `a` in this scope
    // let four = sum!(a);
}
