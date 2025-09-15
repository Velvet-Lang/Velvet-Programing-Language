#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "weave.h"

void add_to_library(const char* dep, const char* repo) {
    FILE* f = fopen("/weave/library.txt", "a");
    if (f) {
        fprintf(f, "%s - %s\n", dep, repo);
        fclose(f);
    }
}

char* find_repo_for_dep(const char* dep) {
    FILE* f = fopen("/weave/library.txt", "r");
    if (!f) return NULL;
    char line[512];
    while (fgets(line, sizeof(line), f)) {
        char* name = strtok(line, " - ");
        char* repo = strtok(NULL, "\n");
        if (name && repo && strcmp(name, dep) == 0) {
            fclose(f);
            return strdup(repo);
        }
    }
    fclose(f);
    return NULL;
}
