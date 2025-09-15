// weave/src/main.c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "weave.h"  // Zawiera FFI declarations
// #include <ffi.h> lub direct link do Rust lib (via dlopen lub static)

extern char* parse_velvet_ffi(const char* input);
extern void compile_to_rust_ffi(const char* ast, const char* output);

int main(int argc, char** argv) {
    if (argc < 2) {
        printf("Usage: weave <command> [args]\nCommands: install, build, install-x, help\n");
        return 1;
    }

    char* cmd = argv[1];
    if (strcmp(cmd, "install") == 0 && argc > 2) {
        // Instaluj zależność w /weave (izolowane env, np. via mkdir/chroot)
        char path[256];
        snprintf(path, sizeof(path), "/weave/%s", argv[2]);
        mkdir(path, 0755);
        printf("Installed %s in isolated /weave\n", argv[2]);
        return 0;
    } else if (strcmp(cmd, "build") == 0 && argc > 2) {
        char* target = argv[2];
        // Buduj projekt (zakładamy w bieżącym dir)
        // Użyj FFI do parsowania/kompilacji
        char* input = read_file("main.vel");  // Uproszczone: czytaj plik
        char* ast = parse_velvet_ffi(input);
        char output[256] = "build/main.rs";
        compile_to_rust_ffi(ast, output);
        // Potem: cargo build --release dla target (appimage/deb/rpm via alien lub flatpak-builder)
        // Cross: użyj zig cc dla Windows/Mac
        mkdir("build", 0755);
        if (strcmp(target, "appimage") == 0) {
            system("cargo build --release && appimagetool build/ velvet.appimage");
        } else if (strcmp(target, "deb") == 0) {
            system("cargo build --release && checkinstall -D make install");
        } else if (strcmp(target, "rpm") == 0) {
            // Podobnie
        } else {
            // Default binarki
            system("cargo build --target x86_64-unknown-linux-gnu --release");
            system("cargo build --target x86_64-apple-darwin --release");  // Mac
            system("cargo build --target x86_64-pc-windows-gnu --release"); // Win
            printf("Built binaries in /build\n");
        }
        free(ast);
        free(input);
        return 0;
    } else if (strcmp(cmd, "install-x") == 0 && argc > 2) {
        // Instaluj zewn. lib w /weave (np. apt install w chroot)
        char cmd_buf[512];
        snprintf(cmd_buf, sizeof(cmd_buf), "sudo chroot /weave apt update && apt install -y %s", argv[2]);
        system(cmd_buf);
        return 0;
    } else if (strcmp(cmd, "help") == 0) {
        printf("Weave help:\n- install {dep}\n- build {target}\n- install-x {dep}\n");
        return 0;
    }
    printf("Unknown command\n");
    return 1;
}

// Helper: read_file (uproszczony)
char* read_file(const char* filename) {
    FILE* f = fopen(filename, "r");
    if (!f) return NULL;
    fseek(f, 0, SEEK_END);
    long len = ftell(f);
    fseek(f, 0, SEEK_SET);
    char* buf = malloc(len + 1);
    fread(buf, 1, len, f);
    buf[len] = '\0';
    fclose(f);
    return buf;
}
