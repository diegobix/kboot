CPPFLAGS= -m32 -fno-pic -Wall -fno-exceptions -Wextra -ffreestanding

all: linked

run: linked
	qemu-system-i386 -drive format=raw,file=boot.bin 

linked: boot.o main.o
	ld -m elf_i386 -T linker.ld -o boot.bin boot.o main.o --oformat binary

boot.o: boot.s
	nasm -f elf32 boot.s -o boot.o

main.o: src/main.cpp
	g++ $(CPPFLAGS) -c src/main.cpp -o main.o

clean:
	rm -rf *.o *.bin 