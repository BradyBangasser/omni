# CLI

The `omni` command is the entry point to the compiler. It is a C/C++ front end
(`cli/`) with subcommands; the `compile` command drives `omnicom` over a routes
directory.

Today the compiler is most easily exercised directly: `omnicom` compiles the
example routes under `compiler/test/helloworld/src/routes` and writes the
generated library to `./build/lib/mod.rs`.

> Status: the CLI is scaffolded (`main.cpp`, a `compile` command) and is being
> built out. See Status.
