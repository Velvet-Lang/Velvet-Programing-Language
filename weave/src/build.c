#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include "weave.h"

extern char* scan_project_ffi(const char* dir);
extern void compile_to_rust_ffi(const char* ast, const char* output, const char* deps);
extern char* clone_git_repo_ffi(const char* repo, const char* dest);

void build_project(const char* ast, const char* deps) {
    mkdir("build", 0755);
    mkdir("build/build-files", 0755);
    mkdir("build/linux", 0755);
    mkdir("build/windows", 0755);
    mkdir("build/macos", 0755);
    mkdir("build/lib", 0755);

    compile_to_rust_ffi(ast, "build/build-files/main.rs", deps);

    system("zig build-exe build/build-files/main.rs -O ReleaseFast -target x86_64-linux-gnu -o build/linux/velvet");
    system("zig build-exe build/build-files/main.rs -O ReleaseFast -target x86_64-windows-gnu -o build/windows/velvet.exe");
    system("zig build-exe build/build-files/main.rs -O ReleaseFast -target x86_64-macos -o build/macos/velvet");
}

void build_specific(const char* target) {
    if (strcmp(target, "appimage") == 0) {
        system("appimagetool build/linux velvet.appimage");
    } else if (strcmp(target, "deb") == 0) {
        system("checkinstall -D make install");
    } else if (strcmp(target, "rpm") == 0) {
        system("checkinstall -R make install");
    }
}
