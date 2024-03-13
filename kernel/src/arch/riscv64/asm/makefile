all: entry.a

CC = riscv64-unknown-elf-gcc
CFLAGS = -mabi=lp64d -march=rv64gc

entry.a: entry.S trap.S
	$(CC) $(CFLAGS) ./entry.S -c -o $(OUTPUT_DIR)/entry.o && \
	$(CC) $(CFLAGS) ./trap.S -c -o $(OUTPUT_DIR)/trap.o && \
	ar crs $(OUTPUT_DIR)/libentry.a $(OUTPUT_DIR)/entry.o $(OUTPUT_DIR)/trap.o
