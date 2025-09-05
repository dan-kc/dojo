# Move Semantics and Ownership Transfer

## Solution

```rust
fn process_owned_string(s: String) -> usize {
    let upper = s.to_uppercase();
    upper.len() + s.len()
}

fn transform_and_return(mut vec: Vec<i32>, multiplier: i32) -> (Vec<i32>, i32) {
    let sum: i32 = vec.iter().sum();
    vec.iter_mut().for_each(|x| *x *= multiplier);
    (vec, sum)
}

#[derive(Debug)]
struct PersonData {
    name: String,
    age: u32,
    hobbies: Vec<String>,
}

impl PersonData {
    fn new(name: String, age: u32, hobbies: Vec<String>) -> Self {
        Self { name, age, hobbies }
    }

    fn extract_name(self) -> (String, u32, Vec<String>) {
        (self.name, self.age, self.hobbies)
    }

    fn take_hobbies(&mut self) -> Vec<String> {
        std::mem::take(&mut self.hobbies)
    }
}

fn swap_ownership<T>(a: T, b: T) -> (T, T) {
    (b, a)
}

fn conditional_move<T, P>(value: T, predicate: P) -> Option<T>
where
    P: FnOnce(&T) -> bool,
{
    if predicate(&value) {
        Some(value)
    } else {
        None
    }
}

struct ConfigBuilder {
    name: Option<String>,
    timeout: Option<u64>,
    retries: Option<u32>,
}

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
        let timeout = self.timeout.unwrap_or(5000);
        let retries = self.retries.unwrap_or(3);

        Ok(Config {
            name,
            timeout,
            retries,
        })
    }
}

#[derive(Debug, PartialEq)]
struct Config {
    name: String,
    timeout: u64,
    retries: u32,
}

fn process_multiple_owned(data: (String, Vec<i32>, Option<u64>)) -> String {
    let (prefix, numbers, maybe_suffix) = data;
    let sum: i32 = numbers.iter().sum();
    let suffix = maybe_suffix.unwrap_or(42);
    format!("{}: sum={}, extra={}", prefix, sum, suffix)
}

fn create_move_closure(prefix: String, suffix: String) -> impl Fn(&str) -> String {
    move |middle: &str| {
        format!("{} {} {}", prefix, middle, suffix)
    }
}

fn iterator_with_moves(strings: Vec<String>, transform: fn(String) -> String) -> Vec<String> {
    strings.into_iter().map(transform).collect()
}

fn try_process_or_return(value: String, should_succeed: bool) -> Result<String, String> {
    if should_succeed {
        Ok(value.to_uppercase())
    } else {
        Err(value)
    }
}
```

## Explanation

This solution demonstrates **move semantics** and **ownership transfer** patterns in Rust:

### Key Concepts Demonstrated:

1. **Move Semantics**:
   - When a value is passed to a function by value (`String` not `&String`), ownership is transferred
   - The original binding can no longer be used after the move
   - This prevents use-after-move bugs at compile time

2. **Partial Moves**:
   - `extract_name(self)` consumes the entire struct and moves out all fields
   - `take_hobbies(&mut self)` uses `std::mem::take` to move out one field while keeping the struct usable
   - After partial move, only non-moved fields remain accessible

3. **Builder Pattern with Moves**:
   - Each method takes `self` by value and returns `Self`
   - Enables method chaining while transferring ownership through the chain
   - `build()` consumes the builder to produce the final result

4. **Conditional Ownership Transfer**:
   - `conditional_move` demonstrates how ownership can be conditionally transferred
   - Uses `FnOnce` to allow the predicate to take ownership if needed
   - Returns `Option<T>` to represent successful/failed transfers

### How Move Semantics Work:

```rust
// Move occurs here:
let s = String::from("hello");
let len = process_owned_string(s);  // s is moved into the function
// println!("{}", s);  // ERROR: s is no longer valid

// Builder pattern moves:
let config = ConfigBuilder::new()    // ConfigBuilder created
    .name("test".to_string())        // ConfigBuilder moved, new one returned
    .timeout(1000)                   // Previous ConfigBuilder consumed, new one returned
    .build()                         // Final ConfigBuilder consumed, Config returned
    .unwrap();
```

### Memory Management Patterns:

1. **Zero-Copy Operations**: Moving doesn't copy data, just transfers ownership
2. **RAII**: Resources are automatically cleaned up when ownership ends
3. **Compile-time Safety**: Use-after-move detected at compile time
4. **Optimization**: Moves often optimized away by the compiler

### Partial Move with `std::mem::take`:

```rust
fn take_hobbies(&mut self) -> Vec<String> {
    std::mem::take(&mut self.hobbies)  // Replaces with Vec::default()
}

// After this call:
// - self.name and self.age are still accessible
// - self.hobbies is now an empty Vec (not moved/invalid)
// - Returned Vec contains the original hobbies
```

### Builder Pattern Benefits:

1. **Fluent API**: Method chaining creates readable code
2. **Immutable Steps**: Each step produces a new builder state
3. **Validation**: `build()` can validate required fields
4. **Type Safety**: Impossible to use builder after calling `build()`

### Move Closures:

```rust
let closure = create_move_closure(prefix, suffix);
// prefix and suffix are moved into the closure
// They're no longer accessible in the outer scope
// The closure owns its captured environment
```

### Iterator Ownership Patterns:

```rust
// into_iter() takes ownership of the Vec and its elements
strings.into_iter().map(transform).collect()

// vs iter() which only borrows:
// strings.iter().map(|s| transform(s.clone())).collect()  // Would require cloning
```

### Error Handling with Moves:

The `try_process_or_return` function demonstrates how to handle ownership in error scenarios:
- On success: Value is processed and moved into `Ok`
- On failure: Original value is moved into `Err` for recovery

### Memory Layout Changes:

```
Before move:
Stack Frame 1: [String ptr, len, cap] → Heap: "hello"
                     ↓ move
Stack Frame 2: [String ptr, len, cap] → Heap: "hello"
Stack Frame 1: [INVALID]  // Original binding invalidated
```

### Performance Implications:

1. **Zero-cost**: Moves are typically zero-cost (just transferring ownership)
2. **No Deep Copies**: Large data structures moved efficiently
3. **Stack Allocation**: Small types may be copied instead of moved
4. **Compiler Optimization**: Many moves optimized away entirely

### When Moves Occur:

- **Function Arguments**: Passing by value
- **Assignment**: `let b = a;` for non-Copy types  
- **Return Values**: Returning owned values
- **Method Calls**: Methods taking `self` by value
- **Closures**: `move` closures capture by value

### Copy vs Move Types:

```rust
// Copy types (implement Copy trait):
let a = 42;
let b = a;  // a is copied, still usable
println!("{}", a);  // OK

// Move types (don't implement Copy):
let a = String::from("hello");
let b = a;  // a is moved, no longer usable
// println!("{}", a);  // ERROR
```

### Real-World Applications:

- **Resource Management**: Transferring ownership of files, network connections
- **Data Processing**: Moving large datasets between processing stages
- **Configuration**: Builder patterns for complex object construction
- **Error Recovery**: Returning original values in error cases

This solution demonstrates how Rust's ownership system enables safe, efficient resource management while preventing common bugs like use-after-free, double-free, and data races at compile time.