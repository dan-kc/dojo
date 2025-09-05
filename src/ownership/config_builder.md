# Builder Pattern with Ownership - Solution

## Solution

```rust
impl ConfigBuilder {
    fn new() -> Self {
        Self {
            name: None,
            timeout: None,
            retries: None,
        }
    }

    fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    fn timeout(mut self, timeout: u64) -> Self {
        self.timeout = Some(timeout);
        self
    }

    fn retries(mut self, retries: u32) -> Self {
        self.retries = Some(retries);
        self
    }

    fn build(self) -> Result<Config, &'static str> {
        let name = self.name.ok_or("Name is required")?;
        let timeout = self.timeout.ok_or("Timeout is required")?;
        let retries = self.retries.ok_or("Retries is required")?;

        Ok(Config { name, timeout, retries })
    }
}
```

## Explanation

This implementation demonstrates the builder pattern with move semantics:

1. **Consuming methods**: Each builder method takes `self` by value (not `&mut self`), consumes the builder, and returns a new builder instance.

2. **Fluent interface**: The methods can be chained together since each returns `Self`.

3. **Ownership transfer**: After calling any builder method, the original builder is moved and cannot be used again.

4. **Mutable self**: Each method takes `mut self` to allow modifying the internal state before returning.

5. **Validation in build()**: The `build()` method consumes the builder and validates that all required fields are present.

6. **Error handling**: Uses `Result` to handle missing required fields with descriptive error messages.

**Key Rust concepts demonstrated:**
- **Move semantics**: Builder is consumed by each method call
- **Method chaining**: Fluent interface pattern
- **Option handling**: Using `ok_or()` to convert `Option` to `Result`
- **Pattern matching**: Implicit in `ok_or()` usage
- **Ownership**: Builder cannot be reused after calling methods

**Advantages of this pattern:**
- **Compile-time validation**: Cannot accidentally reuse builder
- **Clear ownership**: Explicit about when builder is consumed
- **Immutable final object**: Config is immutable after creation
- **Type safety**: Prevents partial or invalid configurations

**Common variations:**
- Some builders use `&mut self` for performance
- Some implement `Clone` to allow reuse
- Some use typestate pattern for compile-time validation
- Some provide default values instead of requiring all fields

This pattern is commonly used in Rust libraries like `clap`, `reqwest`, and `serde_json`.