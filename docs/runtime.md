# Runtime

The `router` crate is the server that runs a compiled router. It is built for
high-availability, multi-core serving.

## Share-nothing model

The runtime is a **share-nothing monolith**: it opens its listener with
`SO_REUSEPORT` and runs a thread-per-core, current-thread Tokio runtime with
`spawn_local`, so connections are handled without cross-thread sharing or
contention. Each core owns its work end to end.

## Stack

- `hyper` and `hyper-util` for HTTP, with an auto (HTTP/1 and HTTP/2) connection
  builder.
- A custom `LocalExecuter` that spawns onto the local set, keeping tasks on their
  origin thread.
- A raw `socket2` listener bound to `0.0.0.0:8080` with `SO_REUSEPORT` and a
  1024-deep backlog.

## Status

The runtime currently serves a fixed response ("Hello from the Share-Nothing
Monolith!") while the generated router is wired in. The serving model, socket
setup, and executor are in place; dispatch through the compiled tree is the piece
being connected.
