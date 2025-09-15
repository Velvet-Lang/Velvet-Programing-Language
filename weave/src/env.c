#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include "weave.h"

void setup_isolated_env() {
    mkdir("/weave", 0755);
    FILE* f = fopen("/weave/library.txt", "a");
    if (f) fclose(f);
}

void install_external(const char* dep) {
    char cmd_buf[512];
    snprintf(cmd_buf, sizeof(cmd_buf), "sudo chroot /weave apt update && apt install -y %s", dep);
    system(cmd_buf);
}
