// use lazy_static::lazy_static; // Not work

// #[macro_use] extern crate lazy_static; // export Global

mod static_kv {
    use lazy_static::lazy_static; // OK, // 仅在当前作用域有效
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
}
fn read_kv() {
    // mod 名称(命名空间)::内部成员
    // 使用 ref 模式匹配来获取
    let ref m = static_kv::MAP;
    // 与以下代码可互换:
    // let m = &static_kv::MAP;
    
    assert_eq!("foo", *m.get(&0).unwrap_or(&static_kv::NF));
    assert_eq!(static_kv::NF, *m.get(&1).unwrap_or(&static_kv::NF));
}
fn rw_mut_kv() -> Result<(), String> {
    // 不允许同时读写, 此处读写隔离, 出作用域自动释放
    // 不隔离会产生死锁的情况
    {
        let m = static_kv::MAP_MUT.read().map_err(|e| e.to_string())?;
        assert_eq!("bar", *m.get(&0).unwrap_or(&static_kv::NF));
    }
    {
        let mut m = static_kv::MAP_MUT.write().map_err(|e| e.to_string())?;
        m.insert(1, "baz");
    }
    Ok(())
}
fn main() {
    read_kv();
    match rw_mut_kv() {
        Ok(()) => {
            let m = static_kv::MAP_MUT
                .read()
                .map_err(|e| e.to_string())
                .unwrap();
            assert_eq!("baz", *m.get(&1).unwrap_or(&static_kv::NF));
        }
        Err(e) => println!("Error {}", e),
    }
}
