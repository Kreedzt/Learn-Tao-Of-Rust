#![feature(test)]
extern crate test;
use test::Bencher;
use std::path::PathBuf;
use csv_challenge::{
    Opt,
    {load_csv, write_csv},
    replace_column
};

// 标注为基准测试
#[bench]
fn bench_read_100times(b: &mut Bencher) {
    // 提供了 `iter` 方法, 接收闭包作为参数
    b.iter(|| {
        let n = test::black_box(100);
        (0..n).fold(0, |_,_| {
            test_load_csv();
            0
        });
    })
}

fn test_load_csv() {
    let filename = PathBuf::from("./input/challenge.csv");
    load_csv(filename);
}
