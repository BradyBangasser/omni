#ifndef OMNI_CLI_COMMAND_H
#define OMNI_CLI_COMMAND_H

#ifndef __cplusplus
extern "C" {
#endif

typedef struct omni_cli_cmd {

} omni_cli_cmd_t;

#ifndef __cplusplus

namespace omni::cli::core {
using cmd = omni_cli_cmd_t;
}
}
#endif

#endif
