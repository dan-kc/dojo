# Advanced Lifetimes - Solution

## Solution

```rust
pub struct ConfigManager<'a> {
    static_config: &'static str,
    runtime_config: Option<&'a str>,
}

impl<'a> ConfigManager<'a> {
    pub fn new(static_config: &'static str) -> ConfigManager<'a> {
        ConfigManager {
            static_config,
            runtime_config: None,
        }
    }
    
    pub fn add_runtime_config(&mut self, runtime_config: &'a str) {
        self.runtime_config = Some(runtime_config);
    }
    
    pub fn get_effective_config(&self) -> &str {
        self.runtime_config.unwrap_or(self.static_config)
    }
    
    pub fn get_static_config(&self) -> &'static str {
        self.static_config
    }
}

pub fn get_first_line(text: &str) -> &str {
    match text.find('\n') {
        Some(pos) => &text[..pos],
        None => text,
    }
}

pub struct LifetimeCache<'a> {
    entries: Vec<(&'a str, &'a str)>,
}

impl<'a> LifetimeCache<'a> {
    pub fn new() -> LifetimeCache<'a> {
        LifetimeCache {
            entries: Vec::new(),
        }
    }
    
    pub fn insert(&mut self, key: &'a str, value: &'a str) {
        self.entries.push((key, value));
    }
    
    pub fn get(&self, key: &str) -> Option<&'a str> {
        self.entries
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| *v)
    }
    
    pub fn keys(&self) -> impl Iterator<Item = &'a str> + '_ {
        self.entries.iter().map(|(k, _)| *k)
    }
    
    pub fn clear_matching<F>(&mut self, predicate: F)
    where
        F: Fn(&str) -> bool,
    {
        self.entries.retain(|(key, _)| !predicate(key));
    }
}

pub trait StaticStringProvider {
    fn get_static_string() -> &'static str;
}

pub struct ConstantProvider;

impl StaticStringProvider for ConstantProvider {
    fn get_static_string() -> &'static str {
        "CONSTANT_VALUE"
    }
}

pub fn combine_static_and_runtime<T>(provider: T, runtime_data: &str) -> String
where
    T: StaticStringProvider,
{
    let static_str = T::get_static_string();
    format!("{}{}", static_str, runtime_data)
}

pub struct SafeSelfRef {
    full_text: String,
    part_start: usize,
    part_end: usize,
}

impl SafeSelfRef {
    pub fn new(text: String, start: usize, end: usize) -> SafeSelfRef {
        let end = end.min(text.len());
        let start = start.min(end);
        
        SafeSelfRef {
            full_text: text,
            part_start: start,
            part_end: end,
        }
    }
    
    pub fn get_full_text(&self) -> &str {
        &self.full_text
    }
    
    pub fn get_part(&self) -> &str {
        &self.full_text[self.part_start..self.part_end]
    }
}

pub fn demonstrate_covariance(long_lived: &'static str, short_lived: &str) -> (&str, &str) {
    fn process_string<'a>(s: &'a str) -> &'a str {
        s.trim()
    }
    
    // This works because 'static can be coerced to any shorter lifetime
    let processed_long = process_string(long_lived);
    let processed_short = process_string(short_lived);
    
    (processed_long, processed_short)
}

pub fn lifetime_relationship_demo<'long, 'short>(
    long_ref: &'long str,
    _short_ref: &'short str,
) -> &'long str 
where
    'long: 'short,
{
    // We can return the longer-lived reference safely
    // The bound ensures 'long outlives 'short
    long_ref
}
```

## Explanation

### Static Lifetimes

**Understanding `'static`:**
```rust
static_config: &'static str,
runtime_config: Option<&'a str>,
```

**Key Properties:**
- `'static` references live for the entire program duration
- String literals have `'static` lifetime automatically
- `'static` can be coerced to any shorter lifetime (covariance)
- Not all long-lived data needs to be `'static`

**When to Use Static:**
- Configuration data from string literals
- Error messages and constants
- Data that truly needs to live for the entire program

### Lifetime Subtyping and Variance

**Covariance Example:**
```rust
pub fn demonstrate_covariance(long_lived: &'static str, short_lived: &str) -> (&str, &str)
```

**How Covariance Works:**
- `&'static str` can be used where `&'a str` is expected for any `'a`
- Longer lifetimes can substitute for shorter lifetime requirements
- This is safe because longer-lived data is always valid when shorter-lived data would be

**Lifetime Bounds:**
```rust
where 'long: 'short,  // 'long outlives 'short
```

**Practical Applications:**
- API design where you want flexibility in lifetime relationships
- Generic functions that work with various lifetime combinations
- Ensuring safety when one reference must outlive another

### Complex Lifetime Relationships

**Multiple Lifetime Parameters:**
```rust
pub struct LifetimeCache<'a> {
    entries: Vec<(&'a str, &'a str)>,
}
```

**Design Decisions:**
- Single lifetime parameter `'a` for simplicity
- Both keys and values must live for the same minimum duration
- Alternative: `LifetimeCache<'k, 'v>` for independent key/value lifetimes

### Self-Referential Patterns

**Safe Self-Reference:**
```rust
pub struct SafeSelfRef {
    full_text: String,
    part_start: usize,
    part_end: usize,
}
```

**Why This Works:**
- Stores indices instead of direct references
- No actual self-reference in the struct
- Methods create references on-demand from owned data
- Avoids borrowing issues entirely

**Alternative Approaches:**
- `Pin<Box<T>>` for truly self-referential structs
- `Rc<RefCell<T>>` for shared mutable access
- Arena allocation for managing related lifetimes

### Advanced Patterns

**Iterator Return Types:**
```rust
pub fn keys(&self) -> impl Iterator<Item = &'a str> + '_
```

**Breakdown:**
- `Item = &'a str` - iterator yields references with lifetime `'a`
- `+ '_` - iterator itself tied to `&self` lifetime
- Compiler infers the relationship between these lifetimes

**Trait Methods with Static Lifetimes:**
```rust
pub trait StaticStringProvider {
    fn get_static_string() -> &'static str;
}
```

**Benefits:**
- Guarantees returned strings live for entire program
- Enables optimization opportunities
- Clear contract about data lifetime expectations

### Common Anti-Patterns and Solutions

**Fighting the Borrow Checker:**
```rust
// Don't do this - trying to store references to owned data
// pub struct BadSelfRef<'a> {
//     text: String,
//     part: &'a str,  // Can't reference text field
// }

// Do this instead:
pub struct GoodSelfRef {
    text: String,
    start: usize,
    end: usize,
}
```

**Overusing Static Lifetimes:**
- Not all long-lived data needs `'static`
- Use specific lifetime parameters when possible
- `'static` should be for truly global data

### Performance and Memory Considerations

**Lifetime Impact on Performance:**
- Lifetime checking happens at compile time (zero runtime cost)
- Static lifetimes enable compiler optimizations
- Complex lifetime relationships don't affect runtime performance

**Memory Management:**
- Lifetimes prevent memory leaks and use-after-free
- Enable zero-copy programming patterns
- Allow fine-grained control over data ownership

**Best Practices:**
1. Start with simple lifetime relationships
2. Use owned types (`String`, `Vec`) when lifetimes become complex
3. Prefer composition over self-referential patterns
4. Use `'static` sparingly and appropriately
5. Design APIs to minimize lifetime constraints on users