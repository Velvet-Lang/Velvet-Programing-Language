#ifndef WEAVE_H
#define WEAVE_H

void add_to_library(const char* dep, const char* repo);
char* find_repo_for_dep(const char* dep);
void build_project(const char* ast, const char* deps);
void build_specific(const char* target);
void setup_isolated_env();
void install_external(const char* dep);

#endif
