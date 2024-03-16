all: device.h

SHELL = /bin/bash
DUMMY = .$(ARCH)_$(DEVICE).mark
TOML_PATH = $(CURDIR)/$(ARCH)/$(DEVICE).toml

device.h: $(DUMMY)
	START=false; \
	while read line; do \
		if [ "$$START" = true ] ; then \
			if [[ $${line:0:1} == "[" ]] ; then \
				break;	\
			fi; \
			echo $$line | awk '{print "#define " $$1 " " $$3}'; \
		elif [[ "$$line" == "[parameters]" ]] ; then \
			START=true;\
		fi; \
	done < $(TOML_PATH) > device.h

$(DUMMY): $(TOML_PATH)
	-rm .*.mark; \
	echo "Makefile stub file" > $(DUMMY)

.PHONY: clean
clean:
	-rm .*.mark device.h;
	