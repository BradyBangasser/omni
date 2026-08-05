# Conditions and passes

## ConditionType

The routing tree partitions routes on a `ConditionType`:

| Variant | Check | Status |
| --- | --- | --- |
| `SegmentCount(usize)` | number of path segments | implemented |
| `Length(usize)` | path string length | implemented |
| `Prefix(String)` | shared path prefix | planned |
| `Custom(Arc<dyn CustomCondition>)` | user-defined | supported |

Equality is structural for the built-ins and pointer-identity for `Custom`.

## The Pass trait

A `Pass` inserts a condition level into the tree by partitioning the current
routes:

```rust
impl Pass for Length {
    fn run(&mut self, tree: &mut ConditionTree) {
        tree.insert_root_condition(self, Self::partition);
    }
}
```

Built-in passes: `Length`, `Segcount`, and `Prefix` (planned). Passes are run in
sequence on the tree; their order determines the tree's shape.

## CustomCondition

A custom condition implements:

```rust
trait CustomCondition {
    fn get_type(&self) -> &'static str;
    fn gen_code(&self, writer, indent, nodes) -> Result<usize, _>;
}
```

`gen_code` emits the decision code for that condition, so new routing strategies
can be added without touching the core generator.
