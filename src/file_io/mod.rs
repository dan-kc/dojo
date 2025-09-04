// File I/O Practice Exercises Module
// 
// This module contains comprehensive file I/O exercises for learning Rust's
// standard library file operations. Each submodule focuses on a specific
// aspect of file I/O operations.

pub mod text_processor;
pub mod binary_handler;
pub mod metadata_analyzer;
pub mod atomic_operations;
pub mod directory_sync;
pub mod csv_validator;

// Re-export the main functions for convenience
pub use text_processor::process_text_file;
pub use binary_handler::xor_encrypt_file;
pub use metadata_analyzer::{analyze_directory, FileStats};
pub use atomic_operations::safe_file_replace;
pub use directory_sync::sync_directories;
pub use csv_validator::process_csv_with_validation;