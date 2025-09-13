// Builder Pattern with Ownership Practice
//
// Learning Objectives:
// - Implement builder pattern that transfers ownership through method chaining
// - Practice consuming self in builder methods
// - Handle validation and error conditions in build process
// - Work with Option types and pattern matching
//
// Run with: cargo test config_builder

/// Implement a builder pattern that transfers ownership through method chaining.
struct ConfigBuilder {
    name: Option<String>,
    timeout: Option<u64>,
    retries: Option<u32>,
}

impl ConfigBuilder {
    fn new() -> Self {
        todo!()
    }

    /// Each method should take self by value and return Self
    fn name(self, name: String) -> Self {
        todo!()
    }

    fn timeout(self, timeout: u64) -> Self {
        todo!()
    }

    fn retries(self, retries: u32) -> Self {
        todo!()
    }

    /// Consume the builder and produce the final config
    fn build(self) -> Result<Config, &'static str> {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
struct Config {
    name: String,
    timeout: u64,
    retries: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = ConfigBuilder::new()
            .name("test-config".to_string())
            .timeout(5000)
            .retries(3)
            .build()
            .unwrap();

        assert_eq!(
            config,
            Config {
                name: "test-config".to_string(),
                timeout: 5000,
                retries: 3,
            }
        );
    }

    #[test]
    fn test_builder_failure() {
        let result = ConfigBuilder::new().timeout(1000).build();
        assert!(result.is_err());
    }

    #[test]
    fn test_partial_config() {
        let result = ConfigBuilder::new().name("partial".to_string()).build();
        assert!(result.is_err());
    }

    #[test]
    fn test_builder_order_independence() {
        let config1 = ConfigBuilder::new()
            .retries(2)
            .name("test".to_string())
            .timeout(3000)
            .build()
            .unwrap();

        let config2 = ConfigBuilder::new()
            .timeout(3000)
            .name("test".to_string())
            .retries(2)
            .build()
            .unwrap();

        assert_eq!(config1, config2);
    }

    #[test]
    fn test_builder_replacement() {
        let config = ConfigBuilder::new()
            .name("first".to_string())
            .name("second".to_string()) // Should replace first
            .timeout(1000)
            .retries(1)
            .build()
            .unwrap();

        assert_eq!(config.name, "second");
    }
}
