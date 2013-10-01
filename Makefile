example: libwidmann libwidmannserver
	rust build -L rust-http/build/ -L rust-pcre/build/ -L build examples/example.rs --out-dir build

test: rust-http/build rust-pcre/build
	rust build --test -L rust-http/build/ -L rust-pcre/build/ -L build src/libwidmann/lib.rs --out-dir build
	build/lib

libwidmann: rust-http/build rust-pcre/build
	rust build -L rust-http/build/ -L rust-pcre/build/ -L build src/libwidmann/lib.rs --out-dir build

libwidmannserver: rust-http/build
	rust build -L rust-http/build/ -L rust-pcre/build/ -L build src/libwidmannserver/lib.rs --out-dir build

rust-http/build:
	cd rust-http && make

rust-pcre/build:
	cd rust-pcre && make

.PHONY: test