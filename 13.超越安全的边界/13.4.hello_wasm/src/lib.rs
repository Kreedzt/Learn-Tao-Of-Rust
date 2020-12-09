

// 指定 extern 块的 wasm 模块名称为 env
#[link(wasm_import_module = "env")]
extern "C" {
    // 打算调用 js 中的 console.log()
    pub fn logit();
    // 调用 js 中定义的 hello
    pub fn hello(ptr: *const u8, len: u32);
}

// 导出 C-ABI 兼容的接口
// 经过 LLVM WebAssembly Backend 的编译和 lld 的链接
// 最终输出为 wasm 二进制
#[no_mangle]
pub extern "C" fn add_one(x: i32) {
    unsafe {
        logit();
        let msg = format!("Hello world: {}", x + 1);
        hello(msg.as_ptr(), msg.len() as u32);
    }
}
