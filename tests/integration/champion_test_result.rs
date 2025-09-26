```rust
#[test]
fn test_address_parse() {
    let result = "127.0.0.1:8080".parse();
    match result {
        Ok(address) => println!("Parsed address: {:?}", address),
        Err(e) => panic!("Error parsing address: {}", e),
    }
}
```