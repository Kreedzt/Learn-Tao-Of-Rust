#[link(name = "c_fn", kind = "static")]
extern "C" {
    fn c_fn(num: i32);
    fn c_add(a: i32, b: i32) -> i32;
}

pub fn call_printf_from_c(n: i32) {
    println!("inner call printf_from_c...");
    unsafe {
        c_fn(n);
    }
    println!("ready to leave call printf_from_c...");
}

pub fn call_c_add_from_c(a: i32, b: i32) -> i32 {
    unsafe {
        return c_add(a, b);
    }
}

fn main() {
    println!("Rust call c start...");
    call_printf_from_c(9);
    println!("res: {}", call_c_add_from_c(2, 3));
    println!("Rust call c end...");
}
