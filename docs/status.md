# Status

Omni is at compiler version 0.0.1. This page is the honest line between what runs
today and what is designed but not yet built.

## Implemented

- File-based route discovery from Go handlers via tree-sitter, with method
  resolution (function name, then file name) and middleware detection.
- Route flags (`DB`, `Ctx`, `Hdr`, `Bdy`, `Dyn`, `Hot`).
- The condition-tree router with `Length` and `Segcount` passes and an extensible
  `CustomCondition` trait.
- Lowering to a generation tree of per-method middleware and handler stacks.
- Go and Rust output adapters, with content-addressed (SHA3) handler references.
- A share-nothing hyper runtime (SO_REUSEPORT, thread-per-core, local executor).

## Planned

- The `Prefix` routing pass.
- The build orchestrator (`OmniBuilder`) and the CLI `compile` command end to end.
- Deployable targets: per-route serverless (Lambda) binaries alongside the
  monolith.
- REST and gRPC endpoint generation from the same handlers.
- Terraform generation for the chosen target.
- Wiring the compiled router into the runtime's request path (it currently serves
  a fixed response).

## Goals

The end state: point Omni at a routes directory and get a fast, HA service in the
shape you want (monolith or serverless), speaking REST and gRPC, with the
Terraform to deploy it.
