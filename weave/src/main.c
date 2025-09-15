#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <dirent.h>
#include <sys/stat.h>
#include "weave.h"

extern char* parse_velvet_ffi(const char* input);
extern char* scan_project_ffi(const char* dir);
extern void compile_to_rust_ffi(const char* ast, const char* output, const char* deps);
extern void run_velvet_ffi(const char* input);
extern char* clone_git_repo_ffi(const char* repo, const char* dest);

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

int read_library_velvet(char* deps_str, size_t size) {
    FILE* f = fopen("library.velvet", "r");
    if (!f) return -1;
    char line[512];
    while (fgets(line, sizeof(line), f)) {
        char* dep = strtok(line, " -> ");
        char* repo = strtok(NULL, "\n");
        if (dep && repo) {
            strncat(deps_str, dep, size - strlen(deps_str) - 1);
            strncat(deps_str, ",", size - strlen(deps_str) - 1);
            char lib_dest[256];
            snprintf(lib_dest, sizeof(lib_dest), "build/lib/%s", dep);
            clone_git_repo_ffi(repo, lib_dest);
        }
    }
    fclose(f);
    return 0;
}

int main(int argc, char** argv) {
    if (argc < 2) {
        printf("Usage: weave <command> [args]\nCommands: init, run, install, build, install-x, help\n");
        return 1;
    }

    char* cmd = argv[1];
    if (strcmp(cmd, "init") == 0) {
        mkdir("src", 0755);
        FILE* f = fopen("src/main.vel", "w");
        if (f) {
            fprintf(f, "[zaleznosc]\n@komentarz\n#velvet {\n5 < A\n}\n");
            fclose(f);
        }
        printf("Project initialized with src/main.vel\n");
        return 0;
    } else if (strcmp(cmd, "run") == 0 && argc > 2) {
        char* input = read_file(argv[2]);
        if (input) {
            run_velvet_ffi(input);
            free(input);
        } else {
            printf("Error: Cannot read file %s\n", argv[2]);
        }
        return 0;
    } else if (strcmp(cmd, "install") == 0 && argc > 3) {
        char* dep = argv[2];
        char* repo = argv[3];
        add_to_library(dep, repo);
        char dest[256];
        snprintf(dest, sizeof(dest), "/weave/%s", dep);
        char* result = clone_git_repo_ffi(repo, dest);
        printf("Installed %s from %s: %s\n", dep, repo, result);
        free(result);
        return 0;
    } else if (strcmp(cmd, "build") == 0) {
        char* scan_result = scan_project_ffi(".");
        if (strstr(scan_result, "Error") != NULL) {
            printf("%s\n", scan_result);
            free(scan_result);
            return 1;
        }

        char deps_str[1024] = "";
        read_library_velvet(deps_str, sizeof(deps_str));
        mkdir("build", 0755);
        mkdir("build/lib", 0755);
        mkdir("build/build-files", 0755);
        mkdir("build/linux", 0755);
        mkdir("build/windows", 0755);
        mkdir("build/macos", 0755);

        char* dep = strtok(deps_str, ",");
        while (dep && strlen(dep) > 0) {
            char* repo = find_repo_for_dep(dep);
            if (repo) {
                char lib_dest[256];
                snprintf(lib_dest, sizeof(lib_dest), "build/lib/%s", dep);
                clone_git_repo_ffi(repo, lib_dest);
                free(repo);
            }
            dep = strtok(NULL, ",");
        }

        char* main_input = read_file("src/main.vel");
        if (!main_input) {
            printf("Error: Cannot read src/main.vel\n");
            free(scan_result);
            return 1;
        }
        char* ast = parse_velvet_ffi(main_input);
        free(main_input);

        compile_to_rust_ffi(ast, "build/build-files/main.rs", deps_str);

        system("zig build-exe build/build-files/main.rs -O ReleaseFast -target x86_64-linux-gnu -o build/linux/velvet");
        system("zig build-exe build/build-files/main.rs -O ReleaseFast -target x86_64-windows-gnu -o build/windows/velvet.exe");
        system("zig build-exe build/build-files/main.rs -O ReleaseFast -target x86_64-macos -o build/macos/velvet");

        if (argc > 2) {
            char* target = argv[2];
            if (strcmp(target, "appimage") == 0) {
                system("appimagetool build/linux velvet.appimage");
            } else if (strcmp(target, "deb") == 0) {
                system("checkinstall -D make install");
            } else if (strcmp(target, "rpm") == 0) {
                system("checkinstall -R make install");
            }
        }

        free(ast);
        free(scan_result);
        printf("Built binaries in /build\n");
        return 0;
    } else if (strcmp(cmd, "install-x") == 0 && argc > 2) {
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
