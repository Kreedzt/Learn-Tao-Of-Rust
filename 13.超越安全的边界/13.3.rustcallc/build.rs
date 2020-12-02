extern crate cc;

fn main() {
    cc::Build::new()
        .warnings(true)
        .file("c_src/fn.c")
        .compile("c_fn");
}
