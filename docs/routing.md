# Authoring routes

Routes are a directory tree of handler files, similar to file-based routing on
the frontend, but for backend handlers.

## Directory is the path

The directory structure under `src/routes/` is the URL path. A handler at
`routes/test/test2/get.go` serves `/test/test2`. Dynamic segments come from
directory names and are represented internally as `:param`.

## Handlers

A handler is a function whose **first parameter is `OmniContext`**. Functions
that do not take an `OmniContext` are ignored with a warning.

```go
package routes

func GET(ctx OmniContext) string {
    return "Hello World"
}
```

## Method resolution

The HTTP method is resolved in this order:

1. **Function name.** A function named after a method (`GET`, `POST`, `PUT`,
   `PATCH`, `DELETE`, `HEAD`, `OPTIONS`, `CONNECT`, `TRACE`) is an endpoint for
   that method.
2. **File name.** Otherwise, a method-named file (`get.go`) makes its handlers
   endpoints for that method.
3. **Middleware.** A handler that matches neither (for example a `MIDDLEWARE`
   function, or a `middleware.go` file) is middleware for its subtree.

## Route flags

During discovery each node is tagged with `RouteFlags`: `DB`, `Ctx`, `Hdr`,
`Bdy`, `Dyn`, `Hot`. These describe what the handler touches and let the compiler
specialize the generated code (for example, only wiring body parsing where a
handler reads a body, or treating a `Hot` route differently).

## Language support

Handlers are parsed with tree-sitter. Go is supported today (`tree-sitter-go`),
and the compiler has a Rust adapter for output; see Code generation.
