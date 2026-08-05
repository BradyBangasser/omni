# Code generation

The compiler lowers the routing tree to a `GenTree` and hands it to a
`StackGenerator`, which drives language **adapters** to emit code.

## Adapters

An adapter turns generation nodes into source in a target language. Two exist:

- **Go** (`GoAdapter`). Emits a Go module (runs `go mod init`; module name from
  `GO_MODULE_NAME`, default `omni`) wrapping the handlers.
- **Rust** (`RustAdapter`). Emits Rust into `./build/lib/mod.rs`.

Generated handler references are **content-addressed**: each emitted stack entry
carries a version and a SHA3/Shake hash of the source, so identical handlers
dedupe and changes are detectable across builds.

## Build targets

The same compiled routes are meant to produce different deployable shapes:

- **Share-nothing monolith** - one large binary serving every route (the runtime
  today; see Runtime).
- **Per-route functions** - many small binaries, one per route, for serverless
  (Lambda). *Planned.*

## Endpoints

REST and gRPC endpoints are both intended to be generated from the same handler
set. *Planned.*

## Infrastructure

Omni is intended to emit the **Terraform** to deploy the chosen target. *Planned;
the build orchestrator (`OmniBuilder`) is currently a stub.*
