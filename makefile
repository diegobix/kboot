all: linked

run: linked
	qemu-system-i386 -drive format=raw,file=boot.bin 

linked: boot.o
	ld -m elf_i386 -T linker.ld -o boot.bin boot.o --oformat binary

boot.o: boot.s
	nasm -f elf32 boot.s -o boot.o

clean:
	rm -rf *.o *.bin 