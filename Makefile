.PHONY: build
build: 
	cargo build --release

debug:
	cargo build

.PHONY: install
install:
	cargo install --locked --path .
	install -m 755 configs/log4rs.yaml /etc/hmir/

.PHONY: clean
clean:
	cargo clean
