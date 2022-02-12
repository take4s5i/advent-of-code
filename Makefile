.PHONY: all aoc-rust

PREFIX := bin

all: aoc-rust

$(PREFIX):
	mkdir -p $(PREFIX)

aoc-rust: $(PREFIX)
	cd aoc-rust && cargo test && cargo build --release
	install -p aoc-rust/target/release/aoc-rust $(PREFIX)/aoc-rust
