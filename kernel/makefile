targets = riscv64
.PHONY : $(targets) config clean

OUTPUT_DIR = $(CURDIR)/lib

$(targets) :
	mkdir -p $(OUTPUT_DIR)/$@
	$(MAKE) -C ./src/arch/$@/asm OUTPUT_DIR=$(OUTPUT_DIR)/$@

clean :
	for target in $(targets); do \
		rm -rf $(OUTPUT_DIR)/$$target; \
	done
