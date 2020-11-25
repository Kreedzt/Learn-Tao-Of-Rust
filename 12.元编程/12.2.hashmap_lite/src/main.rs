#![feature(decl_macro)]
use hashmap_lite::hashmap;
use hashmap_lite::inc as mc;

// 12-41 使用 `#[macro_use]` 导出 mod 模块中的宏
#[macro_use]
mod macros {
    macro_rules! X {
	    () => {
		    Y!();
	    };
    }
    macro_rules! Y {
	    () => {
		    
	    };
    }
}

macro unless($arg: expr, $branch: expr) {
    ( if !$arg { $branch });
}

fn cmp(a: i32, b: i32) {
    unless!(a > b, {
        println!("{} < {}", a, b)
    });
}

fn main() {
    // 12-40 导入宏
    // let map = hashmap! {
    //     "a" => 1,
    //     "b" => 2
    // };

    // assert_eq!(map["a"], 1);


    // 12-41 使用 `#[macro_use]` 导出 mod 模块中的宏
    X!();

    let res = mc!{
        1
    };
    assert_eq!(res, 2);

    let (a, b) = (1, 2);
    cmp(a, b);
}
