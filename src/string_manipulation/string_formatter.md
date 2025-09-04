# String Formatting and Transformation - Solution

## Complete Implementation

```rust
use std::fmt::Write;

#[derive(Debug)]
pub struct PersonData {
    pub first_name: String,
    pub last_name: String,
    pub age: Option<u32>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

pub fn format_person_data(person: &PersonData) -> String {
    let mut result = String::new();
    
    // Format name and age
    let first_name = person.first_name.trim();
    let last_name = person.last_name.trim();
    
    if let Some(age) = person.age {
        write!(result, "{} {} (age: {})", first_name, last_name, age).unwrap();
    } else {
        write!(result, "{} {}", first_name, last_name).unwrap();
    }
    
    // Collect valid contact information
    let mut contact_info = Vec::new();
    
    if let Some(email) = &person.email {
        let email = email.trim();
        if !email.is_empty() {
            contact_info.push(format!("Email: {}", email));
        }
    }
    
    if let Some(phone) = &person.phone {
        let phone = phone.trim();
        if !phone.is_empty() {
            contact_info.push(format!("Phone: {}", phone));
        }
    }
    
    if let Some(address) = &person.address {
        let address = address.trim();
        if !address.is_empty() {
            contact_info.push(format!("Address: {}", address));
        }
    }
    
    // Add contact information or no-contact message
    if contact_info.is_empty() {
        result.push_str("\nNo contact information available");
    } else {
        for info in contact_info {
            result.push('\n');
            result.push_str(&info);
        }
    }
    
    result
}
```

## Alternative Implementation Using Display Trait

```rust
impl std::fmt::Display for PersonData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Format name and age
        let first_name = self.first_name.trim();
        let last_name = self.last_name.trim();
        
        if let Some(age) = self.age {
            write!(f, "{} {} (age: {})", first_name, last_name, age)?;
        } else {
            write!(f, "{} {}", first_name, last_name)?;
        }
        
        // Helper closure to check and format contact info
        let add_contact_info = |f: &mut std::fmt::Formatter, label: &str, value: &Option<String>| -> std::fmt::Result {
            if let Some(val) = value {
                let trimmed = val.trim();
                if !trimmed.is_empty() {
                    write!(f, "\n{}: {}", label, trimmed)?;
                }
            }
            Ok(())
        };
        
        let mut has_contact = false;
        
        // Check if we have any contact information
        if self.email.as_ref().map_or(false, |e| !e.trim().is_empty()) ||
           self.phone.as_ref().map_or(false, |p| !p.trim().is_empty()) ||
           self.address.as_ref().map_or(false, |a| !a.trim().is_empty()) {
            has_contact = true;
        }
        
        if has_contact {
            add_contact_info(f, "Email", &self.email)?;
            add_contact_info(f, "Phone", &self.phone)?;
            add_contact_info(f, "Address", &self.address)?;
        } else {
            write!(f, "\nNo contact information available")?;
        }
        
        Ok(())
    }
}

// Then format_person_data becomes:
pub fn format_person_data(person: &PersonData) -> String {
    person.to_string()
}
```

## Key Concepts Explained

### 1. String Building Strategies

**Direct String Manipulation:**
```rust
let mut result = String::new();
write!(result, "{} {} (age: {})", first_name, last_name, age).unwrap();
result.push_str("\nEmail: example@test.com");
```

**Using Format Macros:**
```rust
// format! - creates new String
let header = format!("{} {}", first_name, last_name);

// write! - appends to existing String
write!(result, "{} {}", first_name, last_name).unwrap();

// writeln! - appends with newline
writeln!(result, "Email: {}", email).unwrap();
```

**Performance Comparison:**
- `write!` is more efficient for building strings incrementally
- `format!` creates new allocations for each call  
- `push_str` is fastest for literal strings
- Pre-allocating capacity can improve performance for large strings

### 2. Optional Data Handling

**Robust Option Processing:**
```rust
if let Some(email) = &person.email {
    let email = email.trim();
    if !email.is_empty() {
        contact_info.push(format!("Email: {}", email));
    }
}
```

**Key Techniques:**
- **Reference Borrowing**: `&person.email` avoids moving the Option
- **Nested Validation**: Check `Some` then validate content
- **Whitespace Handling**: Trim and check for empty strings
- **Graceful Degradation**: Handle missing data without errors

### 3. String Memory Management

```rust
let mut contact_info = Vec::new();
// ... populate vector
for info in contact_info {
    result.push('\n');
    result.push_str(&info);
}
```

**Memory Efficiency:**
- Collect contact info in `Vec` before formatting
- Single allocation for main result string
- Reuse references where possible (`&info` instead of cloning)

### 4. Advanced Formatting Patterns

**Conditional Formatting:**
```rust
match (person.age, has_any_contact) {
    (Some(age), true) => format!("{} {} (age: {})\n{}", first, last, age, contact),
    (Some(age), false) => format!("{} {} (age: {})\nNo contact information available", first, last, age),
    (None, true) => format!("{} {}\n{}", first, last, contact),
    (None, false) => format!("{} {}\nNo contact information available", first, last),
}
```

**Builder Pattern for Complex Formatting:**
```rust
struct PersonFormatter<'a> {
    person: &'a PersonData,
    include_age: bool,
    contact_prefix: String,
}

impl<'a> PersonFormatter<'a> {
    fn new(person: &'a PersonData) -> Self {
        Self {
            person,
            include_age: true,
            contact_prefix: String::new(),
        }
    }
    
    fn with_age(mut self, include: bool) -> Self {
        self.include_age = include;
        self
    }
    
    fn build(self) -> String {
        // Formatting logic here
        todo!()
    }
}
```

## Best Practices Demonstrated

### 1. Safe String Operations

```rust
// Safe character operations
result.push('\n');  // Add single character
result.push_str(&info);  // Add string slice

// Safe trimming with UTF-8
let trimmed = email.trim();  // Handles Unicode whitespace
```

### 2. Error Handling in Formatting

```rust
// write! can fail, but String formatting typically doesn't
write!(result, "{}", data).unwrap();

// More defensive approach
if write!(result, "{}", data).is_err() {
    result.push_str("Error formatting data");
}

// Using Display trait with proper error propagation
impl Display for PersonData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", data)?;  // Proper error propagation
        Ok(())
    }
}
```

### 3. Unicode and Localization Considerations

```rust
// Proper Unicode handling
let first_name = person.first_name.trim();  // Unicode-aware trimming

// Locale-aware formatting (conceptual)
fn format_person_localized(person: &PersonData, locale: &str) -> String {
    match locale {
        "en-US" => format!("{} {}", person.first_name, person.last_name),
        "ja-JP" => format!("{} {}", person.last_name, person.first_name),
        _ => format!("{} {}", person.first_name, person.last_name),
    }
}
```

### 4. Performance Optimizations

```rust
// Pre-calculate capacity for large strings
fn format_person_optimized(person: &PersonData) -> String {
    let estimated_capacity = person.first_name.len() + 
                           person.last_name.len() + 
                           person.email.as_ref().map_or(0, |e| e.len()) + 
                           100; // Buffer for formatting
    
    let mut result = String::with_capacity(estimated_capacity);
    // ... formatting logic
    result
}

// Use references to avoid cloning
fn collect_contact_info(person: &PersonData) -> Vec<String> {
    let mut info = Vec::new();
    
    // Process each field only once
    [
        ("Email", &person.email),
        ("Phone", &person.phone), 
        ("Address", &person.address)
    ]
    .iter()
    .filter_map(|(label, opt_value)| {
        opt_value.as_ref()
            .map(|v| v.trim())
            .filter(|v| !v.is_empty())
            .map(|v| format!("{}: {}", label, v))
    })
    .for_each(|formatted| info.push(formatted));
    
    info
}
```

This implementation demonstrates comprehensive string formatting techniques in Rust, emphasizing safety, performance, and maintainability. The solution handles optional data gracefully while providing clean, readable output formatting.