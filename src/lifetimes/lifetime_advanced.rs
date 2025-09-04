// Advanced Lifetime Practice  
//
// Learning objectives:
// - Static lifetimes and their use cases
// - Lifetime subtyping and variance
// - Complex lifetime relationships in real-world patterns
// - Self-referential data structures
//
// cargo test --lib lifetimes::lifetime_advanced

/// Implement a configuration manager that holds references to static configuration data
/// and can also hold references to runtime data with different lifetimes.
pub struct ConfigManager {
    todo!("Add fields: static_config: &'static str, runtime_config: Option<&str>")
}

impl ConfigManager {
    /// Create a new ConfigManager with static configuration
    pub fn new(static_config: &'static str) -> ConfigManager {
        todo!("Initialize with static config and None for runtime config")
    }
    
    /// Add runtime configuration that may have a shorter lifetime
    pub fn add_runtime_config(&mut self, runtime_config: &str) {
        todo!("Set the runtime_config field")
    }
    
    /// Get the effective configuration (runtime overrides static if present)
    pub fn get_effective_config(&self) -> &str {
        todo!("Return runtime config if present, otherwise static config")
    }
    
    /// Get only the static configuration
    pub fn get_static_config(&self) -> &'static str {
        todo!("Return static config")
    }
}

/// Create a function that demonstrates lifetime subtyping.
/// This function should accept a reference with any lifetime and return
/// a reference with a potentially shorter lifetime.
pub fn get_first_line(text: &str) -> &str {
    todo!("Return the first line of text (before first newline or whole text)")
}

/// Implement a caching structure that demonstrates complex lifetime relationships.
/// The cache holds references to data with different lifetimes and provides
/// methods to query the cached data safely.
pub struct LifetimeCache {
    todo!("Add lifetime parameters and fields for entries: Vec<(&str, &str)>")
}

impl LifetimeCache {
    /// Create a new empty cache
    pub fn new() -> LifetimeCache {
        todo!("Create empty cache")
    }
    
    /// Add an entry to the cache with key and value having potentially different lifetimes
    pub fn insert(&mut self, key: &str, value: &str) {
        todo!("Add (key, value) tuple to entries")
    }
    
    /// Find a value by key, returning a reference if found
    pub fn get(&self, key: &str) -> Option<&str> {
        todo!("Search for key and return corresponding value")
    }
    
    /// Get all keys in the cache
    pub fn keys(&self) -> impl Iterator<Item = &str> {
        todo!("Return iterator over all keys")
    }
    
    /// Clear all entries with keys matching a predicate
    pub fn clear_matching<F>(&mut self, predicate: F)
    where
        F: Fn(&str) -> bool,
    {
        todo!("Remove entries where predicate returns true for the key")
    }
}

/// Implement a trait for types that can provide string data with static lifetime
pub trait StaticStringProvider {
    todo!("Define method: fn get_static_string() -> &'static str;")
}

/// Create a struct that implements StaticStringProvider
pub struct ConstantProvider;

impl StaticStringProvider for ConstantProvider {
    todo!("Return a string literal (has static lifetime)")
}

/// Implement a function that works with any type implementing StaticStringProvider
/// and combines it with runtime data to create a new owned string.
pub fn combine_static_and_runtime<T>(provider: T, runtime_data: &str) -> String
where
    T: StaticStringProvider,
{
    todo!("Get static string from provider and combine with runtime_data")
}

/// Create a structure that demonstrates self-referential patterns safely.
/// This struct holds owned data and a reference to a part of that data.
/// Note: This is challenging and may require careful design to avoid self-reference issues.
pub struct SafeSelfRef {
    todo!("Design a struct that can safely hold both owned and borrowed data")
}

impl SafeSelfRef {
    /// Create a new SafeSelfRef that stores text and creates a reference to part of it
    pub fn new(text: String, start: usize, end: usize) -> SafeSelfRef {
        todo!("Safely create structure with owned data and reference to part of it")
    }
    
    /// Get the owned text
    pub fn get_full_text(&self) -> &str {
        todo!("Return reference to full owned text")
    }
    
    /// Get the referenced part
    pub fn get_part(&self) -> &str {
        todo!("Return the part of text that was referenced")
    }
}

/// Demonstrate variance with a covariant function.
/// This function should work with references of any lifetime,
/// showing how longer lifetimes can be used where shorter ones are expected.
pub fn demonstrate_covariance(long_lived: &'static str, short_lived: &str) -> (&str, &str) {
    // Helper function that expects a specific lifetime
    fn process_string<'a>(s: &'a str) -> &'a str {
        s.trim()
    }
    
    todo!("Call process_string with both parameters, showing covariance works")
}

/// Create a function that demonstrates working with multiple lifetime bounds
/// where one lifetime must outlive another.
pub fn lifetime_relationship_demo<'long, 'short>(
    long_ref: &'long str,
    short_ref: &'short str,
) -> &'long str 
where
    'long: 'short,  // 'long outlives 'short
{
    todo!("Return the longer-lived reference, demonstrating lifetime bounds")
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
    fn test_get_first_line() {
        assert_eq!(get_first_line("hello\nworld"), "hello");
        assert_eq!(get_first_line("single line"), "single line");
        assert_eq!(get_first_line(""), "");
        assert_eq!(get_first_line("line1\nline2\nline3"), "line1");
    }

    #[test]
    fn test_lifetime_cache_basic() {
        let mut cache = LifetimeCache::new();
        
        let key1 = "key1";
        let value1 = "value1";
        let key2 = "key2";
        let value2 = "value2";
        
        cache.insert(key1, value1);
        cache.insert(key2, value2);
        
        assert_eq!(cache.get("key1"), Some("value1"));
        assert_eq!(cache.get("key2"), Some("value2"));
        assert_eq!(cache.get("key3"), None);
    }

    #[test]
    fn test_lifetime_cache_keys_iteration() {
        let mut cache = LifetimeCache::new();
        cache.insert("alpha", "1");
        cache.insert("beta", "2");
        cache.insert("gamma", "3");
        
        let mut keys: Vec<&str> = cache.keys().collect();
        keys.sort();
        assert_eq!(keys, vec!["alpha", "beta", "gamma"]);
    }

    #[test]
    fn test_lifetime_cache_clear_matching() {
        let mut cache = LifetimeCache::new();
        cache.insert("test_1", "value1");
        cache.insert("prod_1", "value2");
        cache.insert("test_2", "value3");
        
        cache.clear_matching(|key| key.starts_with("test_"));
        
        assert_eq!(cache.get("test_1"), None);
        assert_eq!(cache.get("test_2"), None);
        assert_eq!(cache.get("prod_1"), Some("value2"));
    }

    #[test]
    fn test_constant_provider() {
        let provider = ConstantProvider;
        let static_str = ConstantProvider::get_static_string();
        assert!(!static_str.is_empty());
        
        let combined = combine_static_and_runtime(provider, "_runtime");
        assert!(combined.contains("_runtime"));
    }

    #[test]
    fn test_safe_self_ref() {
        let text = "Hello, World!".to_string();
        let safe_ref = SafeSelfRef::new(text, 0, 5);
        
        assert_eq!(safe_ref.get_full_text(), "Hello, World!");
        assert_eq!(safe_ref.get_part(), "Hello");
    }

    #[test]
    fn test_safe_self_ref_different_ranges() {
        let text = "Rust Programming".to_string();
        let safe_ref = SafeSelfRef::new(text, 5, 16);
        
        assert_eq!(safe_ref.get_full_text(), "Rust Programming");
        assert_eq!(safe_ref.get_part(), "Programming");
    }

    #[test]
    fn test_demonstrate_covariance() {
        static LONG_LIVED: &str = "I live for the entire program";
        let short_lived = "I have a shorter lifetime".to_string();
        
        let (processed_long, processed_short) = demonstrate_covariance(LONG_LIVED, &short_lived);
        
        assert_eq!(processed_long, "I live for the entire program");
        assert_eq!(processed_short, "I have a shorter lifetime");
    }

    #[test]
    fn test_lifetime_relationship_demo() {
        static LONG_DATA: &str = "long lived data";
        let short_data = "short lived".to_string();
        
        let result = lifetime_relationship_demo(LONG_DATA, &short_data);
        assert_eq!(result, "long lived data");
    }
}