all: entry.a

entry.a: entry.S
	riscv64-unknown-elf-gcc -mabi=lp64d -march=rv64gc ./entry.S -c -o $(OUTPUT_DIR)/entry.o && \
	ar crs $(OUTPUT_DIR)/libentry.a $(OUTPUT_DIR)/entry.o
