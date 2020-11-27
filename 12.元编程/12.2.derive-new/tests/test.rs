use derive_new::New;

// 12-56 tests/test.rs 中编写测试用例
// 无字段结构体
#[derive(New, PartialEq, Debug)]
pub struct Foo {}

// 包含字段的结构体
#[derive(New, PartialEq, Debug)]
pub struct Bar {
    pub x: i32,
    pub y: String,
}

// 单元结构体
#[derive(New, PartialEq, Debug)]
pub struct Baz;

// 元组结构体
#[derive(New, PartialEq, Debug)]
pub struct Tuple(pub i32, pub i32);


// 12-57 tests/test.rs 中编写调用 new 方法的测试用例
#[test]
fn test_empty_struct() {
    let x = Foo::new();
    assert_eq!(x, Foo {});
}

#[test]
fn test_simple_struct() {
    let x = Bar::new(42, "Hello".to_owned());
    assert_eq!(
        x,
        Bar {
            x: 42,
            y: "Hello".to_owned()
        }
    );
}

#[test]
fn test_unit_struct() {
    let x = Baz::new();
    assert_eq!(x, Baz);
}

#[test]
fn test_simple_tuple_struct() {
    let x = Tuple::new(5, 6);
    assert_eq!(x, Tuple(5, 6));
}


// 12-68 TODO
// #[derive(New)]
// pub struct Fred {
//     #[new(value = "1 + 2")]
//     pub x: i32,
//     pub y: String,
//     #[new(value = "vec![-42, 42]")]
//     pub z: Vec<i8>,
// }

// #[test]
// fn test_struct_with_values() {
//     let x = Fred::new("Fred".to_owned());
//     assert_eq!(
//         x,
//         Fred {
//             x: 3,
//             y: "Fred".to_owned(),
//             z: vec![-42, 42]
//         }
//     )
// }
