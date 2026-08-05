# Architecture

Omni is a Rust workspace of focused crates plus a CLI.

| Crate | Role |
| --- | --- |
| `base` | Shared types: the HTTP `Method` set, the Omni ABI types (`OmniContext`, `OmniRequest`, `OmniResponse`, `OmniHeader`, `OmniBody`), and ABI helpers. |
| `compiler` | `omnicom`, the compiler. Discovers routes, builds the routing tree, and generates code. |
| `router` | The runtime server that executes a compiled router. |
| `cli` | The `omni` command-line entry point. |

## The compile pipeline

1. **Discover.** `walk_routes` walks the routes directory. Each handler file is
   parsed with tree-sitter, and functions are discovered and validated (a handler
   takes an `OmniContext` as its first parameter). Each becomes a `Node`, either
   an `Endpoint(method)` or `Middleware`, carrying `RouteFlags` describing what it
   uses (DB, context, header, body, dynamic, hot path).
2. **Build the routing tree.** The routes are loaded into a `ConditionTree`, and
   optimization passes partition them into a decision tree keyed on cheap
   conditions (segment count, path length, prefix). See The router tree.
3. **Lower to a generation tree.** `GenTree::from_cond_tree` turns the condition
   tree into a `GenTree` of `GenRoute`s, each a per-method stack of
   middleware and handler references.
4. **Generate.** A `StackGenerator` drives language adapters (Go, Rust) that emit
   the router and wiring into the build directory.

The compiler context (`OmnicomCtx`) fixes the output layout: `./build` for
output, `./build/bin` for binaries, `./build/lib` for generated libraries.
