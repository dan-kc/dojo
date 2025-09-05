// Config Manager
//
// Learning objectives:
// - Static lifetimes and their use cases
// - Combining static and runtime lifetimes
// - Optional references with different lifetimes
//
// cargo test --bin config_manager

/// Implement a configuration manager that holds references to static configuration data
/// and can also hold references to runtime data with different lifetimes.
pub struct ConfigManager<'a> {
    static_config: &'static str,
    runtime_config: Option<&'a str>,
}

impl<'a> ConfigManager<'a> {
    /// Create a new ConfigManager with static configuration
    pub fn new(static_config: &'static str) -> ConfigManager<'a> {
        ConfigManager {
            static_config,
            runtime_config: None,
        }
    }
    
    /// Add runtime configuration that may have a shorter lifetime
    pub fn add_runtime_config(&mut self, runtime_config: &'a str) {
        self.runtime_config = Some(runtime_config);
    }
    
    /// Get the effective configuration (runtime overrides static if present)
    pub fn get_effective_config(&self) -> &str {
        self.runtime_config.unwrap_or(self.static_config)
    }
    
    /// Get only the static configuration
    pub fn get_static_config(&self) -> &'static str {
        self.static_config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    static STATIC_CONFIG: &str = "production_config";

    #[test]
    fn test_config_manager_static_only() {
        let config = ConfigManager::new(STATIC_CONFIG);
        assert_eq!(config.get_effective_config(), "production_config");
        assert_eq!(config.get_static_config(), "production_config");
    }

    #[test]
    fn test_config_manager_with_runtime() {
        let mut config = ConfigManager::new(STATIC_CONFIG);
        let runtime = "debug_config".to_string();
        config.add_runtime_config(&runtime);
        
        assert_eq!(config.get_effective_config(), "debug_config");
        assert_eq!(config.get_static_config(), "production_config");
    }

    #[test]
    fn test_config_manager_runtime_scope() {
        let mut config = ConfigManager::new(STATIC_CONFIG);
        
        {
            let runtime = "temporary_config".to_string();
            config.add_runtime_config(&runtime);
            assert_eq!(config.get_effective_config(), "temporary_config");
        } // runtime goes out of scope here
        
        // After runtime data is dropped, this test verifies the design
        // In practice, this would be a compilation error if not handled correctly
        assert_eq!(config.get_static_config(), "production_config");
    }

    #[test]
    fn test_config_manager_override_runtime() {
        let mut config = ConfigManager::new(STATIC_CONFIG);
        let runtime1 = "config1".to_string();
        let runtime2 = "config2".to_string();
        
        config.add_runtime_config(&runtime1);
        assert_eq!(config.get_effective_config(), "config1");
        
        config.add_runtime_config(&runtime2);
        assert_eq!(config.get_effective_config(), "config2");
    }
}

fn main() {
    println!("Run tests with: cargo test --bin config_manager");
}