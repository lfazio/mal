EXEC_DIR := target/release

STEP0 := $(EXEC_DIR)/step0_repl
STEP1 := $(EXEC_DIR)/step1_read_print
STEP2 := $(EXEC_DIR)/step2_eval
STEP3 := $(EXEC_DIR)/step3_env
STEPS := $(STEP0) $(STEP1) $(STEP2) $(STEP3)

all: $(STEPS)

$(STEPS): $(EXEC_DIR)/%: src/%.rs
	cargo build --release --bin $*

$(STEP1): src/error.rs src/printer.rs src/reader.rs src/types.rs
$(STEP2) $(STEP3): src/builtins/mod.rs src/environment.rs src/evaluation.rs

lint:
	rustfmt src/*.rs
	cargo clippy

.PHONY: clean

clean:
	cargo clean