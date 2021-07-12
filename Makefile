all: build bind

build:
	cargo build

bind:
	rust-objcopy --binary-architecture=riscv64 target/riscv64gc-unknown-none-elf/debug/os --strip-all -O binary target/riscv64gc-unknown-none-elf/debug/os.bin