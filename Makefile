example: build libwidmann libwidmannserver
	rustc -L rust-http/build/ -L rust-pcre/build/ -L build examples/example.rs --out-dir build

test: build rust-http/build rust-pcre/build example
	rustc --test -L rust-http/build/ -L rust-pcre/build/ -L build src/libwidmann/lib.rs --out-dir build
	build/lib

libwidmann: build rust-http/build rust-pcre/build
	rustc -L rust-http/build/ -L rust-pcre/build/ -L build src/libwidmann/lib.rs --out-dir build

libwidmannserver: build rust-http/build
	rustc -L rust-http/build/ -L rust-pcre/build/ -L build src/libwidmannserver/lib.rs --out-dir build

rust-http/build:
	cd rust-http && make

rust-pcre/build:
	cd rust-pcre && make

build:
	mkdir -p build

knob:
	rustpkg install github.com/skade/knob

clean:
	rm -rf build
	rm -rf rust-http/build
	rm -rf rust-pcre/build

.PHONY: test
