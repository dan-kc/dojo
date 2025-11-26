// Concurrency practice questions module
// Topics covered:
// - Basic thread spawning and joining
// - Channel communication (mpsc)
// - Shared state with Arc<Mutex<T>>
// - Thread pools and parallel processing
// - Deadlock prevention
// - Send and Sync traits

// Basic threading patterns
pub mod parallel_counter;
pub mod threaded_factorial;
pub mod parallel_vector_processing;
pub mod thread_identification;

// Channel communication patterns
pub mod producer_consumer_pattern;
pub mod work_distribution_system;
pub mod processing_pipeline;
pub mod timed_message_collector;
pub mod broadcast_system;

// Shared state patterns
pub mod concurrent_counter;
pub mod thread_safe_cache;
pub mod bank_transfer_simulation;
pub mod bounded_queue;
pub mod parallel_sum;

// Thread pool and parallel processing patterns
pub mod thread_pool_implementation;
pub mod parallel_map;
pub mod parallel_reduce;
pub mod parallel_prime_sieve;

// Deadlock prevention patterns
pub mod dining_philosophers;
pub mod timeout_resource_manager;
pub mod hierarchical_locking;

// Send and Sync trait patterns
pub mod thread_safe_counter;
pub mod send_wrapper;
pub mod non_send_container;
pub mod thread_local_storage;
pub mod send_not_sync;
pub mod send_to_thread_utils;
