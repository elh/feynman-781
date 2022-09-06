run:
	RUSTFLAGS="-C target-cpu=native" cargo run -q --release

clean: # rm leftover binary if make run is interrupted
	rm src/main

lint:
	cargo fmt
	cargo clippy

.PHONY: run clean lint
