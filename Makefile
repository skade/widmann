example: lib
	rust build -L rust-http/build/ -L build src/examples/example.rs --out-dir build

lib: rust-http/build
	rust build -L rust-http/build/ -L build src/libwidmann/lib.rs --out-dir build

rust-http/build:
	cd rust-http && make