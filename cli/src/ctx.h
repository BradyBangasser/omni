#ifndef OMNI_CLI_CTX_H
#define OMNI_CLI_CTX_H

typedef struct omni_cli_ctx {
  struct {
    const int argc;
    const char **argv;
  } args;

} cmni_cli_ctx_t;

#endif
