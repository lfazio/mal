EXEC_DIR := target/release

STEP0 := $(EXEC_DIR)/step0_repl
STEP1 := $(EXEC_DIR)/step1_read_print
STEPS := $(STEP0) $(STEP1)

all: $(STEPS)

$(STEPS): $(EXEC_DIR)/%: src/%.rs
	cargo build --release --bin $*

$(STEP1): src/error.rs src/printer.rs src/reader.rs src/types.rs

lint:
	rustfmt src/*.rs
	cargo clippy

.PHONY: clean

clean:
	cargo clean