SHELL := bash
.ONESHELL:
.SHELLFLAGS := -eu -o pipefail -c
.DELETE_ON_ERROR:
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

BIN = target/arm-unknown-linux-gnueabihf/release/plato
KVOL = /Volumes/KOBOeReader

all: $(BIN)

image:
	docker build . -f Dockerfile.local-dev -t plato:local-dev
.PHONY: image

setup:
	docker run --rm -it -v $$(pwd):/plato plato:local-dev ./setup.sh
.PHONY: setup 

$(BIN):
	docker run --rm -it -v $$(pwd):/plato plato:local-dev make inner-build

install: $(BIN)
	@if [ -d "$(KVOL)" ]; then \
		echo "Copying binary to Kobo" ; \
		cp $(BIN) $(KVOL)/.adds/plato_dev/plato; \
		echo "Unmounting $(KVOL)" ; \
		diskutil unmountDisk $(KVOL); \
	else \
		echo "Kobo not connected"; \
	fi

# RULES /!\ Run within container /!\

inner-build:
	cargo build --release --target=arm-unknown-linux-gnueabihf --features="importer"
.PHONY: inner-build
