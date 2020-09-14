fn main() {
    let orig = Box::new(5);
    println!("{}", *orig);

    // 所有权转移
    let stolen = orig;
    // error[E0382]: borrow of moved value: `orig`
    // println!("{}", *orig);
}
