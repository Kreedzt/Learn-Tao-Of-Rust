fn main() {
    // 13-1 unsafe 块中使用引用依旧会进行借用检查
    unsafe {
        let mut a = "hello";
        let b = &a;
        let c = &mut a;
    }


    // 13-4 unsafe 块示意
    let hello = vec![104, 101, 108, 108, 111];
    let hello = unsafe { String::from_utf8_unchecked(hello) };
    // error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
    // let hello = String::from_utf8_unchecked(hello);
    assert_eq!("hello", hello);
}
