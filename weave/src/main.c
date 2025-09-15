#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <dirent.h>
#include <sys/stat.h>
#include "weave.h"  // FFI declarations

// Extern FFI z Rust
extern char* parse_velvet_ffi(const char* input);
extern char* scan_project_ffi(const char* dir);
extern void compile_to_rust_ffi(const char* ast, const char* output, const char* deps);
extern void run_velvet_ffi(const char* input);
extern char* clone_git_repo_ffi(const char* repo, const char* dest);

// Helper: Czytaj plik
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

// Helper: Pisz do library.txt
void add_to_library(const char* dep, const char* repo) {
    FILE* f = fopen("/weave/library.txt", "a");
    if (f) {
        fprintf(f, "%s - %s\n", dep, repo);
        fclose(f);
    }
}

// Helper: Znajdź repo dla dep z library.txt
char* find_repo_for_dep(const char* dep) {
    FILE* f = fopen("/weave/library.txt", "r");
    if (!f) return NULL;
    char line[512];
    while (fgets(line, sizeof(line), f)) {
        char* name = strtok(line, " - ");
        char* repo = strtok(NULL, "\n");
        if (strcmp(name, dep) == 0) {
            fclose(f);
            return strdup(repo);
        }
    }
    fclose(f);
    return NULL;
}

int main(int argc, char** argv) {
    if (argc < 2) {
        printf("Usage: weave <command> [args]\n");
        return 1;
    }

    char* cmd = argv[1];
    if (strcmp(cmd, "init") == 0) {
        mkdir("src", 0755);
        FILE* f = fopen("src/main.vel", "w");
        if (f) {
            fprintf(f, "# Velvet project initialized\n");
            fclose(f);
        }
        printf("Project initialized.\n");
        return 0;
    } else if (strcmp(cmd, "run") == 0 && argc > 2) {
        char* input = read_file(argv[2]);
        if (input) {
            run_velvet_ffi(input);
            free(input);
        }
        return 0;
    } else if (strcmp(cmd, "install") == 0 && argc > 3) {  // weave install dep repo
        char* dep = argv[2];
        char* repo = argv[3];
        add_to_library(dep, repo);
        char dest[256];
        snprintf(dest, sizeof(dest), "/weave/%s", dep);
        clone_git_repo_ffi(repo, dest);
        printf("Installed %s from %s\n", dep, repo);
        return 0;
    } else if (strcmp(cmd, "build") == 0) {
        // Skanuj projekt (bieżący dir)
        char* scan_result = scan_project_ffi(".");
        // Parse JSON: [deps, errors]
        // Uproszczone: zakładamy brak JSON parser w C, użyj string split (real: use jansson or similar)
        if (strstr(scan_result, "Error") != NULL) {
            printf("%s\n", scan_result);
            free(scan_result);
            return 1;
        }
        // Extract deps (uproszczone: assume format "deps:[\"dep1\",\"dep2\"],errors:[]")
        char* deps_start = strstr(scan_result, "deps:[") + 6;
        char* deps_end = strstr(deps_start, "\"],");
        char deps_str[1024];
        strncpy(deps_str, deps_start, deps_end - deps_start);
        deps_str[deps_end - deps_start] = '\0';

        // Dla każdej dep: znajdź repo, klonuj do /build/lib/
        mkdir("build", 0755);
        mkdir("build/lib", 0755);
        char* dep = strtok(deps_str, ",\"");
        while (dep) {
            char* repo = find_repo_for_dep(dep);
            if (repo) {
                char lib_dest[256];
                snprintf(lib_dest, sizeof(lib_dest), "build/lib/%s", dep);
                clone_git_repo_ffi(repo, lib_dest);
                free(repo);
            }
            dep = strtok(NULL, ",\"");
        }

        // Zbierz wszystkie .vel do jednego AST (uproszczone)
        char* main_input = read_file("src/main.vel");  // Zakładamy main.vel
        char* ast = parse_velvet_ffi(main_input);
        free(main_input);

        // Kompiluj do Rust
        mkdir("build/build-files", 0755);
        compile_to_rust_ffi(ast, "build/build-files/main.rs", deps_str);

        // Buduj binarki cross (użyj zig dla cross-compile)
        mkdir("build/linux", 0755);
        mkdir("build/windows", 0755);
        mkdir("build/macos", 0755);
        system("zig build-exe build/build-files/main.rs -O ReleaseFast -target x86_64-linux-gnu -o build/linux/velvet");  // Przykład
        system("zig build-exe build/build-files/main.rs -O ReleaseFast -target x86_64-windows-gnu -o build/windows/velvet.exe");
        system("zig build-exe build/build-files/main.rs -O ReleaseFast -target x86_64-macos -o build/macos/velvet");

        // Dla appimage/deb/rpm: jeśli arg
        if (argc > 2) {
            char* target = argv[2];
            if (strcmp(target, "appimage") == 0) {
                system("appimagetool build/linux velvet.appimage");
            } // Podobnie dla deb/rpm
        }

        free(ast);
        free(scan_result);
        printf("Built in /build\n");
        return 0;
    } else if (strcmp(cmd, "install-x") == 0 && argc > 2) {
        // Jak wcześniej
        char cmd_buf[512];
        snprintf(cmd_buf, sizeof(cmd_buf), "sudo chroot /weave apt update && apt install -y %s", argv[2]);
        system(cmd_buf);
        return 0;
    } else if (strcmp(cmd, "help") == 0) {
        printf("Weave help:\n- init\n- run {file}\n- install {dep} {repo}\n- build [appimage/deb/rpm]\n- install-x {dep}\n- help\n");
        return 0;
    }
    printf("Unknown command\n");
    return 1;
}
