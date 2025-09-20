Here is an example of a single test function for the `Registry::with_discovery()` method in the `main.rs` file:

```rust
#[test]
fn test_registry_with_discovery() {
    // Create a new registry with discovery enabled
    let mut registry = Registry::with_discovery();

    // Verify that the registry has at least two nodes (in-memory)
    assert!(registry.nodes().is_some());
}
```

This test function:

1. Creates a new `Registry` instance with `with_discovery()` enabled.
2. Asserts that the `nodes()` method