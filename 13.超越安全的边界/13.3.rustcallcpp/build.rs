extern crate cc;

fn main() {
    cc::Build::new()
        .cpp(true)
        .warnings(true)
        .flag("-Wall")
        .flag("-std=c++14")
        .flag("-c")
        .file("cpp_src/sorting.cpp")
        .compile("sorting");
    // 相当于以下操作:
    // g++ -Wall -std=c++14 -c spp_src/sorting.cpp
    // ar rc libsorting.a sorting.o
}
