# lifetimes/config_manager

Reference implementation. Read after attempting the exercise.

```rust
pub struct ConfigManager<'a> {
    static_config: &'static str,
    runtime_config: Option<&'a str>,
}
impl<'a> ConfigManager<'a> {
    pub fn new(static_config: &'static str) -> Self {
        Self {
            static_config,
            runtime_config: None,
        }
    }
    pub fn add_runtime_config(&mut self, runtime_config: &'a str) {
        self.runtime_config = Some(runtime_config);
    }
    pub fn clear_runtime_config(&mut self) {
        self.runtime_config = None;
    }
    pub fn get_effective_config(&self) -> &'a str {
        self.runtime_config.unwrap_or(self.static_config)
    }
    pub fn get_static_config(&self) -> &'static str {
        self.static_config
    }
}
```
