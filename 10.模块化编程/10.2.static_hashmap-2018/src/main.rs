// 引入模块
mod read_func;

// use crate::read_func::{read_kv, rw_mut_kv};
// 此处也可以用 `self` 代替 `crate`, 表明会以当前相对路径寻找
use self::read_func::{read_kv, rw_mut_kv};

fn main() {
    read_kv();
    match rw_mut_kv() {
        Ok(()) => {
            // 层级关系
            let m = read_func::static_kv::MAP_MUT
                .read()
                .map_err(|e| e.to_string())
                .unwrap();
            assert_eq!("baz", *m.get(&1).unwrap_or(&read_func::static_kv::NF));
        }
        Err(e) => println!("Error {}", e),
    }
}
