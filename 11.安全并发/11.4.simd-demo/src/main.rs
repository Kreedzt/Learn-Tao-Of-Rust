#![feature(stdsimd)]
// TODO: 未编译通过

use ::std as read_std;
use stdsimd as std;
#[cfg(target_arch = "x86")]
use ::std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use ::std::arch::x86_64::*;

fn main() {
    // 11-79 SIMD 使用示例
    if is_x86_feature_detected!("sse4.2") {
        #[target_feature(enable = "sse4.2")]
        unsafe fn worker() {
            let needle = b"\r\n\t ignore this ";
            let haystack = b"Split a \r\n\t line  ";
            // 从内存中将长度为 128 位的整数数据加载到向量寄存器中.
            // 实际调用的是 Intel 的 _mm_loadu_si128 指令.
            // 这里是将 needle 字符串加载到向量寄存器中.
            let a = _mm_loadu_si128(needle.as_ptr() as *const _);
            // 同理, 将 haystack 字符串加载到向量寄存器中, 此过程也称为打包字符串
            let b = _mm_loadu_si128(haystack.as_ptr() as *const _);
            // 参数 1: 打包好的 needle 字符串
            // 参数 2: 想要检索的长度
            // 参数 3: 打包好的子串
            // 参数 4: 长度
            // 参数 5: 比较模式说明符, 此处使用的是: 代表字符串相等检测模式
            // 整个函数要做的就是在 haystack 字符串中查找匹配 needle 前三位的索引的位置
            let idx = _mm_cmpestri(a, 3, b, 20, _SIDD_CMP_EQUAL_ORDERED);

            assert_eq!(idx, 8);
        }

        unsafe {
            worker();
        }
    }
}
