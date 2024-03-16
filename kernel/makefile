.PHONY: asm device clean

LIB_DIR = $(CURDIR)/lib
DEVICE_DIR = $(CURDIR)/device

CARGO_CLEAN_MARKER = $(OUT_DIR)/.cargo_clean.marker

all: asm

asm: $(CARGO_CLEAN_MARKER) device
	mkdir -p $(LIB_DIR)/$(ARCH)
	$(MAKE) -C $(LIB_DIR) ARCH=$(ARCH) DEVICE_DIR=$(DEVICE_DIR)

device:
	$(MAKE) -C $(DEVICE_DIR) ARCH=$(ARCH) DEVICE=$(DEVICE)

clean:
	$(MAKE) clean -C $(LIB_DIR)
	$(MAKE) clean -C $(DEVICE_DIR)

$(CARGO_CLEAN_MARKER):
	$(MAKE) clean;
	echo "Makefile stub file" > $(CARGO_CLEAN_MARKER)
