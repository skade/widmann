example: build libwidmann libwidmannserver
	rust build -L rust-http/build/ -L rust-pcre/build/ -L build examples/example.rs --out-dir build

test: build knob rust-http/build rust-pcre/build example
	rust build --test -L rust-http/build/ -L rust-pcre/build/ -L build src/libwidmann/lib.rs --out-dir build
	build/lib

libwidmann: build knob rust-http/build rust-pcre/build
	rust build -L rust-http/build/ -L rust-pcre/build/ -L build src/libwidmann/lib.rs --out-dir build

libwidmannserver: build rust-http/build
	rust build -L rust-http/build/ -L rust-pcre/build/ -L build src/libwidmannserver/lib.rs --out-dir build

rust-http/build:
	cd rust-http && make

rust-pcre/build:
	cd rust-pcre && make

build:
	mkdir -p build

knob:
	rustpkg install github.com/skade/knob

.PHONY: test