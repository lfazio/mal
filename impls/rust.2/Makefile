EXEC_DIR := target/release

STEP0 := $(EXEC_DIR)/step0_repl
STEP1 := $(EXEC_DIR)/step1_read_print
STEP2 := $(EXEC_DIR)/step2_eval
STEP3 := $(EXEC_DIR)/step3_env
STEP4 := $(EXEC_DIR)/step4_if_fn_do
STEPS := $(STEP0) $(STEP1) $(STEP2) $(STEP3) $(STEP4)

all: $(STEPS)

$(STEPS): $(EXEC_DIR)/%: src/%.rs
	cargo build --release --bin $*

$(STEP1) $(STEP2) $(STEP3) $(STEP4): \
	src/builtins/mod.rs \
	src/builtins/core.rs \
	src/builtins/math.rs \
	src/builtins/string.rs \
	src/types/environment.rs \
	src/types/error.rs \
	src/types/lambda.rs \
	src/types/mod.rs \
	src/evaluation.rs \
	src/printer.rs \
	src/reader.rs

lint:
	rustfmt src/*.rs src/types/*.rs src/builtins/*.rs
	cargo clippy

.PHONY: clean

clean:
	cargo clean