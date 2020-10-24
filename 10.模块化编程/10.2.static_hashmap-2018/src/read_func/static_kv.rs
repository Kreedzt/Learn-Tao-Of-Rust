use lazy_static::lazy_static;
use std::collections::HashMap; // 仅在当前作用域有效
use std::sync::RwLock; // 仅在当前作用域有效
pub const NF: &'static str = "not found";

lazy_static! {
    // 固定格式: [pub] ref NAME_1: TYPE_! = EXPR_1;
    pub static ref MAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m
    };
    
    // 读写锁包装 `HashMap`, 因为 HashMap 不是线程安全的类型(没有实现 `Sync`)
    pub static ref MAP_MUT: RwLock<HashMap<u32, &'static str>> = {
        let mut m = HashMap::new();
        m.insert(0, "bar");
        RwLock::new(m)
    };
}
