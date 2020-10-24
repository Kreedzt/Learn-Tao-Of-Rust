#[macro_use]
extern crate lazy_static;
mod static_kv;
mod read_func;
use read_func::{read_kv, rw_mut_kv};

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
