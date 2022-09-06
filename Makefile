run:
	RUSTFLAGS="-C target-cpu=native" cargo run -q --release

test:
	cargo test

bench:
	rustup run nightly rustc --test -O src/bench.rs
	./bench --bench
	rm -f bench

clean: # rm leftover binary if make run is interrupted
	rm -f src/main
	rm -f bench

lint:
	cargo fmt
	cargo clippy

.PHONY: run test bench clean lint
