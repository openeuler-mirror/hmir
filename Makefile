.PHONY: build
build: 
	cargo build --release

.PHONY: install
install:
	cargo install --locked --path .

.PHONY: clean
clean:
	cargo clean
