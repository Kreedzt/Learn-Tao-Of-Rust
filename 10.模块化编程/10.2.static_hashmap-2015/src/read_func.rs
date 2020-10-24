// 顶级模块 `main.rs` 中已经使用 `mod` 关键字引入, 只需要 use 打开模块即可
// Rust 2015:
use static_kv;
// Rust 2018:
// use crate::static_kv;

// 增加 `pub` 使得可以外部调用
pub fn read_kv() {
    // mod 名称(命名空间)::内部成员
    // 使用 ref 模式匹配来获取
    let ref m = static_kv::MAP;
    // 与以下代码可互换:
    // let m = &static_kv::MAP;
    
    assert_eq!("foo", *m.get(&0).unwrap_or(&static_kv::NF));
    assert_eq!(static_kv::NF, *m.get(&1).unwrap_or(&static_kv::NF));
}

pub fn rw_mut_kv() -> Result<(), String> {
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
