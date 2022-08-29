SHELL := bash
.ONESHELL:
.SHELLFLAGS := -eu -o pipefail -c
.DELETE_ON_ERROR:
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

ifeq ($(origin .RECIPEPREFIX), undefined)
  $(error This Make does not support .RECIPEPREFIX. Please use GNU Make 4.0 or later)
endif
.RECIPEPREFIX = >

all: target/arm-unknown-linux-gnueabihf/release/plato

image:
> docker build . -f Dockerfile.local-dev -t plato:local-dev
.PHONY: image

setup:
> docker run --rm -it -v $$(pwd):/plato plato:local-dev ./setup.sh
.PHONY: setup 

target/arm-unknown-linux-gnueabihf/release/plato: $(shell find src -type f)
> docker run --rm -it -v $$(pwd):/plato plato:local-dev make inner-build

# RULES /!\ Run within container /!\

inner-build:
> cargo build --release --target=arm-unknown-linux-gnueabihf
.PHONY: inner-build
