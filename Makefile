init:
	cargo build --release
	rm -rf build
	mkdir build

example_arithmetic:
	target/release/voz 2 0 65 0  2 1 1 0  2 2 1 0  6 1 2 0  12 6 1 0  11 3 0 0  24 1 1 1  0 0 0 0
	nasm -f elf64 build/out.asm -o build/out.o
	ld -o build/out build/out.o