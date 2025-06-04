all: os-image

fat-drive: stage2.bin bootsector.bin
	dd if=/dev/zero of=drive.img bs=512 conv=notrunc count=20480
	mkfs.fat -F 16 drive.img

os-image: stage2.bin bootsector.bin fat-drive
	dd conv=notrunc if=bootsector.bin of=drive.img bs=1 seek=62
	mcopy -i drive.img stage2.bin ::STAGE2.BIN
	mcopy -i drive.img kernel.bin ::KERNEL.BIN
	
bootsector.bin: stage1/bootsector.s
	nasm -f bin -o $@ $< -I stage1

stage2.bin: stage2.elf
	objcopy -O binary stage2.elf stage2.bin

stage2.elf: stage2/src/*.rs
	cd stage2 && cargo build
	cp stage2/target/i386-target/debug/stage2 ./stage2.elf

run: os-image
	qemu-system-i386 -drive file=drive.img,format=raw

debug: os-image
	qemu-system-i386 -drive file=drive.img,format=raw -S -s &
	gdb stage2.elf

clean:
	rm -f ./*.bin ./*.elf ./*.img stage1/*.bin
	cd stage2 && cargo clean

.PHONY: all os-image bootsector.bin stage2.bin stage2 run clean