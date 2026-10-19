# string_manipulation/string_formatter

Reference implementation. Read after attempting the exercise.

```rust
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
    let mut name = [person.first_name.trim(), person.last_name.trim()]
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    if let Some(age) = person.age {
        name.push_str(&format!(" (age: {age})"));
    }
    let mut lines = vec![name];
    for (label, value) in [
        ("Email", &person.email),
        ("Phone", &person.phone),
        ("Address", &person.address),
    ] {
        if let Some(value) = value.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
            lines.push(format!("{label}: {value}"));
        }
    }
    if lines.len() == 1 {
        lines.push("No contact information available".into());
    }
    lines.join("\n")
}
```
