#[macro_export]
macro_rules! hashmap {
    // `@unit` 和 `@count` 相当于是内部宏规则的宏名, 暂且称为 *内部宏*
    (@unit $($x:tt)*) => (());
    (@count $($rest:expr), *) =>
        (<[()]>::len(&[$(hashmap!(@unit $rest)), *]));
    ($($key:expr => $value:expr), * $(,)*) => {
        {
            let _cap = hashmap!(@count $($key), *);
            let mut _map = ::std::collections::HashMap::with_capacity(_cap);
            $(
                _map.insert($key, $value);
            )*
                _map
        }
    }
}


// 12-42 宏依赖于函数
pub fn incr(x: u32) -> u32 {
    x + 1
}

#[macro_export]
macro_rules! inc {
    // 12-44 使用 `$crate` 变量
    ($x: expr) => ( $crate::incr($x) )
}
