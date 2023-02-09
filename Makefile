init:
	cargo build --release
	rm -rf build
	mkdir build

compile:
	nasm -f elf64 build/out.asm -o build/out.o
	ld -o build/out build/out.o

example_arithmetic:
	target/release/voz 2 0 65 0  2 1 1 0  2 2 1 0  6 1 2 0  12 6 1 0  11 3 0 0  24 1 1 1  0 0 0 0
example_string:
	target/release/voz "Hello World!\n" 18 0 0 0  24 1 0 13
example_img:
	target/release/voz "img.ppm" "P1 2 2\n" "0 " "1 "  18 0 0 0  20 0 0 0 18 1 1 0  24 3 1 7  18 2 2 0  18 3 3 0  24 3 3 2  24 3 2 2  24 3 2 2  24 3 3 2
