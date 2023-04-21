.PHONY: run
run:
	cargo run

.PHONY: check
check:
	cargo check

.PHONY: expand
expand:
	RUSTC_BOOTSTRAP=1 cargo expand

.PHONY: test
test:
	cargo test
