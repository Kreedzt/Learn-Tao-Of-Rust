// TODO: 未编译通过
use faster::*;

fn main() {
    let two_hundred = (&[2.0f32; 100][..])
        .simd_iter()
        .simd_reduce(f32s(0.0), f32s(0.0), |acc, v| acc + v)
        .sum();

    assert_eq!(two_hundred, 200.0f32)
}
