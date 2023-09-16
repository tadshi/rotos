all: entry.a

entry.a: entry.S
	riscv64-unknown-elf-gcc -mabi=lp64d -march=rv64gc ./entry.S -c -o entry.o
	ar crs libentry.a entry.o

.PHONY clean :
	-rm entry.o libentry.a
