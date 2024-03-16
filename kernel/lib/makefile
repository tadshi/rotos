all: libasm.a

riscv64_cc = riscv64-unknown-elf-gcc
riscv64_cflags = -mabi=lp64d -march=rv64gc

SRCDIR = $(CURDIR)/../src/arch/$(ARCH)/asm
OBJ = $(subst $(SRCDIR), $(ARCH), \
		 $(patsubst %.S, %.o, $(wildcard $(SRCDIR)/*.S)))

libasm.a: $(OBJ)
	ar -crs $@ $?

$(ARCH)/%.o: $(SRCDIR)/%.S
	$($(ARCH)_cc) $($(ARCH)_cflags) $^ -I$(DEVICE_DIR) -c -o $@

.PHONY: clean
clean:
	-find . -type f -name '*.o' -exec rm {} \;;\
	rm libasm.a