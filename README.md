---
title: "Omni"
summary: "A file-based route compiler that turns handlers into an optimized router and deployable artifacts, from a share-nothing monolith to per-route serverless functions."
tags: [compiler, rust, distributed-systems, sre, infrastructure]
featured: true
---

# Omni

Omni compiles a directory of HTTP handlers into a service. You write route
handlers as files in a tree; Omni discovers them, compiles the routing into an
optimized decision tree, generates the wiring in your target language, and
produces a deployable artifact.

The design goals:

- **One source, many shapes.** Build the same routes as a single large binary (a
  share-nothing monolith) or as many small binaries, one per route, for
  serverless (Lambda).
- **REST and gRPC** endpoints from the same handlers.
- **Fast routing.** The router is a compiled decision tree, targeting a routing
  cost on the order of tens of CPU cycles rather than a linear match.
- **Infrastructure included.** Generate the Terraform to deploy the result.
- **Built for HA distributed systems.**

Omni is early (compiler version 0.0.1). The docs mark what is implemented today
versus what is planned; see Status for the roadmap.

Full documentation is under `docs/`.
