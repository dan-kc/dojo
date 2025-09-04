# Lifetime Generics - Solution

## Solution

```rust
pub struct DataProcessor<'a, T, F, R> {
    data: &'a T,
    func: F,
    _phantom: std::marker::PhantomData<R>,
}

impl<'a, T, F, R> DataProcessor<'a, T, F, R> {
    pub fn new(data: &'a T, func: F) -> DataProcessor<'a, T, F, R>
    where
        F: Fn(&T) -> R,
    {
        DataProcessor {
            data,
            func,
            _phantom: std::marker::PhantomData,
        }
    }
    
    pub fn process(&self) -> R
    where
        F: Fn(&T) -> R,
    {
        (self.func)(self.data)
    }
    
    pub fn get_data(&self) -> &'a T {
        self.data
    }
}

pub fn filter_and_format<T, F>(slice: &[T], predicate: F) -> impl Iterator<Item = String> + '_
where
    T: Debug,
    F: Fn(&T) -> bool,
{
    slice
        .iter()
        .filter(move |item| predicate(item))
        .map(|item| format!("{:?}", item))
}

pub struct GenericRefHolder<'a, T> {
    item: &'a T,
}

impl<'a, T> GenericRefHolder<'a, T> {
    pub fn new(item: &'a T) -> GenericRefHolder<'a, T> {
        GenericRefHolder { item }
    }
    
    pub fn map_item<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        func(self.item)
    }
    
    pub fn equals(&self, other: &T) -> bool
    where
        T: PartialEq,
    {
        self.item == other
    }
    
    pub fn debug_string(&self) -> String
    where
        T: Debug,
    {
        format!("{:?}", self.item)
    }
}

pub trait BorrowWithLifetime<'a> {
    type Target: ?Sized;
    
    fn borrow_for_lifetime(&'a self) -> &'a Self::Target;
}

pub fn transform_borrowed<'a, T, R, F>(borrower: &'a T, transformer: F) -> R
where
    T: BorrowWithLifetime<'a>,
    F: FnOnce(&'a T::Target) -> R,
{
    let borrowed = borrower.borrow_for_lifetime();
    transformer(borrowed)
}

pub struct ComplexContainer<'a, T, P> {
    items: Vec<&'a T>,
    processor: P,
}

impl<'a, T, P> ComplexContainer<'a, T, P>
where
    P: Fn(&T) -> String,
    T: Clone + Debug,
{
    pub fn new(processor: P) -> ComplexContainer<'a, T, P> {
        ComplexContainer {
            items: Vec::new(),
            processor,
        }
    }
    
    pub fn add_item(&mut self, item: &'a T) {
        self.items.push(item);
    }
    
    pub fn process_all(&self) -> Vec<String> {
        self.items
            .iter()
            .map(|item| (self.processor)(item))
            .collect()
    }
    
    pub fn find_item<F>(&self, predicate: F) -> Option<&'a T>
    where
        F: Fn(&T) -> bool,
    {
        self.items
            .iter()
            .find(|item| predicate(item))
            .copied()
    }
}

pub trait AsyncProcessor<T> {
    fn process_with_callback<F>(&self, item: &T, callback: F)
    where
        F: for<'a> Fn(&'a T) -> String;
}

pub struct SimpleProcessor;

impl<T> AsyncProcessor<T> for SimpleProcessor
where
    T: Debug,
{
    fn process_with_callback<F>(&self, item: &T, callback: F)
    where
        F: for<'a> Fn(&'a T) -> String,
    {
        let result = callback(item);
        println!("Callback result: {}", result);
    }
}

// Additional implementation for BorrowWithLifetime test
struct StringWrapper(String);

impl<'a> BorrowWithLifetime<'a> for StringWrapper {
    type Target = str;
    
    fn borrow_for_lifetime(&'a self) -> &'a str {
        &self.0
    }
}
```

## Explanation

### Combining Lifetimes with Generics

**Complex Parameter Lists:**
```rust
pub struct DataProcessor<'a, T, F, R> {
    data: &'a T,
    func: F,
    _phantom: std::marker::PhantomData<R>,
}
```

**Key Concepts:**
- Lifetime parameters come first: `<'a, T, F, R>`
- `PhantomData` helps the compiler track unused generic parameters
- All parameters must be properly constrained in impl blocks

**Parameter Ordering:**
1. Lifetime parameters (`'a`, `'b`, etc.)
2. Type parameters (`T`, `U`, etc.) 
3. Const parameters (if any)

### Higher-Ranked Trait Bounds (HRTB)

**For-All Quantification:**
```rust
F: for<'a> Fn(&'a T) -> String
```

**What This Means:**
- The function F must work for *any* lifetime `'a`
- The function can handle references with any valid lifetime
- Enables very flexible callback patterns

**Common Use Cases:**
- Callback functions that work with borrowed data
- Generic functions that don't know the specific lifetime
- Trait implementations that must work universally

### Advanced Trait Patterns

**Associated Types with Lifetimes:**
```rust
pub trait BorrowWithLifetime<'a> {
    type Target: ?Sized;
    fn borrow_for_lifetime(&'a self) -> &'a Self::Target;
}
```

**Design Benefits:**
- `?Sized` allows associated type to be unsized (like `str`)
- Lifetime parameter on trait controls returned reference lifetime
- Enables generic borrowing patterns

### Iterator Return Types

**Impl Trait with Lifetimes:**
```rust
pub fn filter_and_format<T, F>(slice: &[T], predicate: F) -> impl Iterator<Item = String> + '_
```

**Key Points:**
- `+ '_` ties iterator lifetime to function parameters
- Closure moves into returned iterator with `move`
- Iterator is lazy and processes items on-demand

### Generic Collections with Lifetimes

**Multiple Constraints:**
```rust
impl<'a, T, P> ComplexContainer<'a, T, P>
where
    P: Fn(&T) -> String,
    T: Clone + Debug,
```

**Pattern Benefits:**
- Where clauses make complex bounds more readable
- Separate concerns: lifetime `'a` for references, traits for behavior
- Generic processor function allows flexible item transformation

### Error-Prone Patterns and Solutions

**PhantomData Usage:**
```rust
_phantom: std::marker::PhantomData<R>
```
Needed when generic parameter R isn't directly stored but affects the type signature.

**Lifetime Variance Issues:**
- Use `for<'a>` when lifetime must be flexible
- Explicit lifetime parameters when lifetime is tied to specific data
- Associated types when lifetime depends on implementor

**Common Compilation Errors:**
1. **Missing lifetime bounds** - Add explicit bounds in where clauses
2. **Lifetime parameter order** - Lifetimes must come before types
3. **Unused generic parameters** - Use PhantomData or remove parameter
4. **HRTB confusion** - Use `for<'a>` for universal quantification

### Best Practices

**Design Guidelines:**
1. Start with simple constraints and add complexity as needed
2. Use where clauses for readability with multiple bounds
3. Prefer associated types over generic parameters when possible
4. Use HRTB sparingly and only when truly needed

**Performance Considerations:**
- Generic code monomorphizes (creates copies for each type)
- Iterator combinators with `impl Trait` are zero-cost
- Closure captures should be minimized in generic contexts

**Testing Strategy:**
- Test with different concrete types to verify generic bounds
- Test lifetime edge cases with short-lived data
- Verify HRTB functions work with various reference lifetimes