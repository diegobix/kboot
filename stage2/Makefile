all: os-image

os-image: stage2.bin bootsector.bin
	dd if=/dev/zero of=drive.img bs=512 count=2048
	dd conv=notrunc if=bootsector.bin of=drive.img bs=512 count=1 seek=0
	dd conv=notrunc if=stage2.bin of=drive.img bs=512 count=15 seek=1

bootsector.bin: stage1/bootsector.s
	nasm -f bin -o $@ $< -I stage1

stage2.bin: stage2.elf
	objcopy -O binary stage2.elf stage2.bin

stage2.elf: stage2/src/*.rs
	cd stage2 && cargo build --release
	cp stage2/target/i386-target/release/stage2 ./stage2.elf

run: all
	qemu-system-i386 -drive file=drive.img,format=raw,if=floppy

clean:
	rm -f ./*.bin ./*.elf ./*.img stage1/*.bin
	cd stage2 && cargo clean

.PHONY: all os-image bootsector.bin stage2.bin stage2 run clean