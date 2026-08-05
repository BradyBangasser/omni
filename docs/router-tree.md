# The router tree

Omni does not match routes with a linear scan. It compiles them into a
`ConditionTree`: a decision tree whose interior nodes are cheap conditions and
whose leaves are route stacks. This is what makes dispatch cost a handful of
cycles.

## Conditions

A `ConditionType` partitions a set of routes:

| Condition | Splits routes by | Status |
| --- | --- | --- |
| `SegmentCount(n)` | number of path segments | implemented |
| `Length(n)` | path string length | implemented |
| `Prefix(s)` | a shared path prefix | planned |
| `Custom(..)` | a user-provided condition | supported via trait |

Each is a constant-time check (an integer compare, or a prefix test), so walking
the tree to a leaf is a short sequence of comparisons rather than a scan over
every route.

## Passes

A **pass** inserts a level of conditions into the tree by partitioning the routes
at that level. Passes are composable and run in sequence; ordering them well is
what shapes an efficient tree.

- `Length` partitions by path length.
- `Segcount` partitions by segment count.
- `Prefix` (planned) partitions by shared prefix.

Custom conditions implement the `CustomCondition` trait, which also emits the
decision code for that condition, so the tree is extensible without changing the
core.

## Leaves: route stacks

Each leaf is a `GenRoute`: a path plus, per HTTP method, a stack of the
middleware and the handler to run. The generated router walks the condition tree
to a leaf, selects the stack for the method, and executes it.
