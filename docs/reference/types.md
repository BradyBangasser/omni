# Types

## HTTP methods

`base::types::http::Method` is a bitflag set, so a route can carry several methods
in one value:

```
GET POST PATCH PUT DELETE HEAD OPTIONS CONNECT TRACE
```

Parsed case-insensitively from a string via `FromStr`.

## ABI types

The Omni ABI (`base::types::Types`) is the vocabulary handlers speak:

| Type | Meaning |
| --- | --- |
| `OmniContext` | The per-request context; a handler's required first parameter. |
| `OmniRequest` | The incoming request. |
| `OmniResponse` | The response. |
| `OmniHeader` | A header value. |
| `OmniBody` | A request or response body. |

`base::abi::is_ctx` recognizes the context type (`"Omni.Context"`) during
discovery.

## Route model

- **`RouteSeg`** - a path segment, either `Static(name)` or `Dynamic(name)` (a
  `:param`).
- **`RouteFlags`** - `DB`, `Ctx`, `Hdr`, `Bdy`, `Dyn`, `Hot`: what a handler
  touches, used to specialize generated code.
- **`Node`** - a discovered route node: `Endpoint(datum, Method)` or
  `Middleware(datum)`. `NodeDatum` holds the source file, function name, params,
  returns, flags, and the parsed AST.
