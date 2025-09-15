CC = gcc
RUST_LIB = ../core/target/debug/libvelvet_core.so

weave: src/main.c
	$(CC) -o build/weave src/main.c -L../core/target/debug -lvelvet_core -ldl -lgit2  # Dodaj libgit2 jeśli potrzeba

install: weave
	cp build/weave /usr/local/bin/

clean:
	rm -rf build/*
