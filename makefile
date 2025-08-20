_DB_PATH ?= ${PWD}/data/
INSTALL_ROOT ?= ./dist

.PHONY: all build migrate install clean



build:
	cargo build

migrate:
	DB_FILE_PATH=$(_DB_PATH) cargo run  --bin init_db


install:
	_DB_PATH=./$(INSTALL_ROOT)/ $(MAKE) migrate
	CARGO_INSTALL_ROOT=$(INSTALL_ROOT) cargo install --path cli --profile release-performance

clean:
	cargo clean
