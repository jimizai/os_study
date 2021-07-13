TARGET := target/riscv64gc-unknown-none-elf/debug/os
OUTPUT := target/riscv64gc-unknown-none-elf/debug/os.bin
BOOTLOADER := ../bootloader/rustsbi-qemu.bin

all: build bind

dev: build bind run

build:
	@cargo build

bind:
	@rust-objcopy \
		--binary-architecture=riscv64 $(TARGET) \
		--strip-all \
		-O binary $(OUTPUT)

run:
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(OUTPUT),addr=0x80200000