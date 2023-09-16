targets = riscv64
.PHONY : $(targets) clean

$(targets) :
	$(MAKE) -C ./src/arch/$@/asm

clean :
	for target in $(targets); do \
		$(MAKE) -C ./src/arch/$$target/asm clean; \
	done
