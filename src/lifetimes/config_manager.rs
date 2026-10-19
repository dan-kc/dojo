// cargo test --lib lifetimes::config_manager::

//! Hold a static default configuration and an optional borrowed runtime override.
//! An override, including an empty string, takes precedence. Clearing it restores the default.
//! get_static_config must return a static reference; get_effective_config returns data valid for the input lifetime.
//! Do not copy or allocate configuration strings.

pub struct ConfigManager<'a> {
    static_config: &'static str,
    runtime_config: Option<&'a str>,
}
impl<'a> ConfigManager<'a> {
    pub fn new(static_config: &'static str) -> Self {
        todo!()
    }
    pub fn add_runtime_config(&mut self, runtime_config: &'a str) {
        todo!()
    }
    pub fn clear_runtime_config(&mut self) {
        todo!()
    }
    pub fn get_effective_config(&self) -> &'a str {
        todo!()
    }
    pub fn get_static_config(&self) -> &'static str {
        todo!()
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

    // #[test]
    // fn test_config_manager_runtime_scope() {
    //     // This test demonstrates what would NOT compile due to lifetime errors
    //     let mut config = ConfigManager::new(STATIC_CONFIG);
    //
    //     {
    //         let runtime = "temporary_config".to_string();
    //         config.add_runtime_config(&runtime);
    //         assert_eq!(config.get_effective_config(), "temporary_config");
    //     } // runtime goes out of scope here
    //
    //     // After runtime data is dropped, this test verifies the design
    //     // In practice, this would be a compilation error if not handled correctly
    //     assert_eq!(config.get_static_config(), "production_config");
    // }

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

#[cfg(test)]
mod override_tests {
    use super::*;
    #[test]
    fn empty_override_and_reset() {
        let mut c = ConfigManager::new("default");
        c.add_runtime_config("");
        assert_eq!(c.get_effective_config(), "");
        c.clear_runtime_config();
        assert_eq!(c.get_effective_config(), "default");
    }
}
