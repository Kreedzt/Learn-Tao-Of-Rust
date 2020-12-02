GCC_BIN ?= $(shell which gcc)
CARGO_BIN ?= $(shell which cargo)

run: clean build
	./c_src/main

clean:
	$(CARGO_BIN) clean
	rm -f ./c_src/main

build:
	$(CARGO_BIN) build 	#生成已指定的 C-ABI 兼容的静态库和动态库
	$(GCC_BIN) -o ./c_src/main ./c_src/main.c -Isrc -L ./target/debug -lcallrust # 编译 C 代码, 链接 Rust 库
