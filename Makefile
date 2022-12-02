ARGS =

run:
	cargo run -p bin -- $(ARGS) test.png

build:
	sed -i.bak 's/^.*image.*/image = { git = "https:\/\/github.com\/ntBre\/image" , branch = "master" }/' Cargo.toml
	cargo build -p bin --release
	mv Cargo.toml.bak Cargo.toml

install:
	cp target/release/bin /usr/bin/safespace
