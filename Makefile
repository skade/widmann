example: libwidmann libwidmannserver
	rust build -L rust-http/build/ -L .rust/build/x86_64-apple-darwin/github.com/skade/rust-pcre/ -L build src/examples/example.rs --out-dir build

libwidmann: rust-http/build
	rust build -L rust-http/build/ -L .rust/build/x86_64-apple-darwin/github.com/skade/rust-pcre/ -L build src/libwidmann/lib.rs --out-dir build


libwidmannserver: rust-http/build
	rust build -L rust-http/build/ -L .rust/build/x86_64-apple-darwin/github.com/skade/rust-pcre/ -L build src/libwidmannserver/lib.rs --out-dir build

rust-http/build:
	cd rust-http && make