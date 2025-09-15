CC = gcc
RUST_LIB = target/debug/libvelvet_core.so  # Link do Rust

weave: src/main.c
	$(CC) -o build/weave src/main.c -L../core/target/debug -lvelvet_core -ldl

install: weave
	cp build/weave /usr/local/bin/

clean:
	rm -rf build/*

.PHONY: install clean
