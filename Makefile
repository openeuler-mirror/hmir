.PHONY: build
build: 
	cargo build --release

debug:
	cargo build

.PHONY: install
install:
	cargo install --locked --path .
	install -m 755 configs/log4rs.yaml /etc/hmir/


.PHONY: package
package:
	#cargo deb
	cd hmir-webterm
	cargo deb
	cd -
	cd hmir-frontend
	npm run tauri build
	cd -


.PHONY: clean
clean:
	cargo clean


