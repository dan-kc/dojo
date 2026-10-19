# iterators/format_adults

Reference implementation. Read after attempting the exercise.

```rust
pub fn format_adults(mut people: Vec<Person>) -> Vec<String> {
    people.sort_by(|a, b| b.age.cmp(&a.age));

    people
        .into_iter()
        .filter(|person| person.age > 18)
        .map(|person| format!("Name: {}, Age: {}", person.name, person.age))
        .collect()
}

#[derive(Debug, PartialEq, Clone)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub email: String,
}
```
