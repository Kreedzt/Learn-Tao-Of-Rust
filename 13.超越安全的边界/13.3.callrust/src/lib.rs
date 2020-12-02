use libc::{c_char, c_uint};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::iter;
use std::slice;

// 13-63
// 告知编译器禁止修改函数名
// pub extern 关键字声明表明该函数为外部调用接口, extern 默认是兼容 C-ABI
#[no_mangle]
pub extern "C" fn print_hello_from_rust() {
    println!("Hello from Rust");
}


// 13-68
// 原书描述有误: https://github.com/ZhangHanDong/tao-of-rust-codes/issues/312
// C 语言的字符串是以 `\0` 字符为结尾的字符数组, 由 `char* str` 定义
#[no_mangle]
pub extern "C" fn hm_chars(s: *const c_char) -> c_uint {
    let c_str = unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s)
    };

    // 转成 Rust 的 String类型操作
    let r_str = c_str.to_str().unwrap();
    // 统计数目后转成 C 语言可用的 `c_unit` 类型
    r_str.chars().count() as c_uint
}


// 13-72
#[no_mangle]
pub extern "C" fn batman_song(length: c_uint) -> *mut c_char {
    let mut song = String::from("boom ");
    song.extend(iter::repeat("nana ").take(length as usize));
    song.push_str("Batman! boom");
    // 因为 `String` 拥有所有权, 所以此处使用 `CString`
    let c_str_strong = CString::new(song).unwrap();
    // 转为 C 兼容字符串
    c_str_strong.into_raw()
}


// 13-73
#[no_mangle]
pub extern "C" fn free_song(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        // C字符串 指针转为 `CString`, 离开作用域后, Rust 自动释放
        CString::from_raw(s)
    };
}


// 13-77
// C 的数组就是指针 + 数组长度, 对应于 Rust 中就是切片类型
#[no_mangle]
pub extern "C" fn sum_of_even(n: *const c_uint, len: c_uint) -> c_uint {
    let numbers = unsafe {
        assert!(!n.is_null());
        // 转为切片类型
        slice::from_raw_parts(n, len as usize)
    };
    // 累计计算出偶数之和
    let sum = numbers
        .iter()
        .filter(|&v| v % 2 == 0)
        .fold(0, |acc, v| acc + v);
    sum as c_uint
}


// 13-80 处理元组
// 标注内存布局兼容 C-ABI
#[repr(C)]
pub struct Tuple {
    x: c_uint,
    y: c_uint,
}

impl From<(u32, u32)> for Tuple {
    fn from(tup: (u32, u32)) -> Tuple { Tuple { x: tup.0, y: tup.1 } }
}

impl From<Tuple> for (u32, u32) {
    fn from(tup: Tuple) -> (u32, u32) { (tup.x, tup.y) }
}


fn compute_tuple(tup: (u32, u32)) -> (u32, u32) {
    let (a, b) = tup;
    (b + 1, a - 1)
}

#[no_mangle]
pub extern "C" fn flip_things_around(tup: Tuple) -> Tuple { compute_tuple(tup.into()).into() }


// 13-83
// 注意: 此处的结构体需要传递给 C 代码使用, 但没有使用 `#[repr(C)]` 来保证其内存布局
// 因为在 C 代码中, 要使用抽象的结构体类型与其相对应, 并非一个具体的结构体类型
// 这种抽象的结构体类型叫做不透明数据类型
pub struct Database {
    data: HashMap<String, u32>,
}

impl Database {
    fn new() -> Database {
        Database {
            data: HashMap::new(),
        }
    }

    fn insert(&mut self) {
        for i in 0..100000 {
            let zip = format!("{:05}", i);
            // 字符串为键, 数字类型为值
            self.data.insert(zip, i);
        }
    }

    // 取值
    fn get(&self, zip: &str) -> u32 { self.data.get(zip).cloned().unwrap_or(0) }
}

#[no_mangle]
pub extern "C" fn database_new() -> *mut Database { Box::into_raw(Box::new(Database::new())) }

#[no_mangle]
pub extern "C" fn database_insert(ptr: *mut Database) {
    let database = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    database.insert()
}

#[no_mangle]
pub extern "C" fn database_query(ptr: *const Database, zip: *const c_char) -> c_uint {
    let database = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let zip = unsafe {
        assert!(!zip.is_null());
        CStr::from_ptr(zip)
    };
    let zip_str = zip.to_str().unwrap();
    database.get(zip_str)
}

#[no_mangle]
pub extern "C" fn database_free(ptr: *mut Database) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        // 堆内存在 Rust 中分配, 所以必须由 Rust 来释放
        Box::from_raw(ptr);
    }
}
