ARGS =

run:
	cargo run -p bin -- $(ARGS)

build:
	cargo build -p bin --release

install:
	cp target/release/bin /usr/bin/safespace
