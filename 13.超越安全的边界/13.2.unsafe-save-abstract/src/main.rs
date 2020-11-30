fn main() {
    // 13-11 创建空指针并判断是否为空指针
    // let p: *const u8 = std::ptr::null();
    // assert!(p.is_null());

    // let s: &str = "hello";
    // // 因字符串是以字节为单位存储的, 所以指针的类型为 `*const u8`.
    // let ptr: *const u8 = s.as_ptr();
    // assert!(!ptr.is_null());

    // let mut s = [1, 2, 3];
    // let ptr: *mut u32 = s.as_mut_ptr();
    // assert!(!ptr.is_null());


    // 13-12 使用 offset 方法
    // let s: &str = "Rust";
    // let ptr: *const u8 = s.as_ptr();
    // unsafe {
    //     // `offset` 是 unsafe 方法, 需要在 unsafe 块中使用
    //     // 优先级: *(ptr.offset(1)) as char
    //     println!("{:?}", *ptr.offset(1) as char); // u
    //     println!("{:?}", *ptr.offset(3) as char); // t
    //     println!("{:?}", *ptr.offset(255) as char); // 不可预料
    // }


    // 13-13 使用 read/wrote 方法
    // let x = "hello".to_string();
    // let y: *const u8 = x.as_ptr();
    // unsafe {
    //     assert_eq!(y.read() as char, 'h');
    // }
    
    // let x = [0, 1, 2, 3];
    // // 注意: 此处的原生指针类型是带长度的
    // let y = x[0..].as_ptr() as *const [u32; 4];
    // unsafe {
    //     assert_eq!(y.read(), [0, 1, 2, 3]);
    // }
    
    // let mut x = "";
    // let y = &mut x as *mut &str;
    // let z = "hello";
    // unsafe {
    //     y.write(z);
    //     assert_eq!(y.read(), "hello");
    // }


    // 13-14 使用 replace/swap 方法
    let mut v: Vec<i32> = vec![1, 2];
    // 指向第一个元素(i32 类型)
    let v_ptr: *mut i32 = v.as_mut_ptr();

    unsafe {
        // 替换第一个值
        let old_v = v_ptr.replace(5);
        assert_eq!(1, old_v);
        assert_eq!([5, 2], &v[..]);
    }
    
    let mut v: Vec<i32> = vec![1, 2];
    // 指向全部元素 Vec<i32>
    let v_ptr = &mut v as *mut Vec<i32>;

    unsafe {
        // 替换全部元素
        let old_v = v_ptr.replace(vec![3, 4, 5]);
        assert_eq!([1, 2], &old_v[..]);
        assert_eq!([3, 4, 5], &v[..]);
    }

    let mut array = [0, 1, 2, 3];
    let x = array[0..].as_mut_ptr() as *mut [u32; 2];
    let y = array[1..].as_mut_ptr() as *mut [u32; 2];

    unsafe {
        assert_eq!([0, 1], x.read());
        assert_eq!([1, 2], y.read());
        // 操作内存区域重叠, 可能引起内部数据混乱
        x.swap(y);
        assert_eq!([1, 0, 1, 3], array);
    }

    v.insert(0, 1);
}
