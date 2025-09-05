# Config Manager

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
```

## Explanation

This solution demonstrates mixing static and dynamic lifetimes:

1. **Static Lifetime**: `&'static str` for configuration that lives for the entire program
2. **Generic Lifetime**: `<'a>` for runtime configuration with shorter lifetimes
3. **Optional Runtime Config**: Uses `Option<&'a str>` to allow optional overrides
4. **Flexible Returns**: `get_effective_config` returns the shorter lifetime using lifetime subtyping

Key concepts:
- **Static vs Dynamic**: Static config lives forever, runtime config has limited scope
- **Lifetime Subtyping**: `&'static str` can be used where `&str` is expected
- **Optional References**: Combining `Option` with lifetimes for conditional data
- **Priority Logic**: Runtime configuration overrides static when present

Important design decisions:
- The struct lifetime `'a` applies only to runtime configuration
- Static configuration doesn't need the generic lifetime parameter
- `get_effective_config` returns `&str` (the shorter lifetime) for flexibility
- This pattern is common in configuration systems and caching layers

Lifetime relationships:
- `'static` outlives any lifetime `'a`
- The struct cannot outlive the runtime configuration it holds
- Methods can return references with different lifetimes as appropriate