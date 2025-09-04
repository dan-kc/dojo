// File I/O Practice Exercises Module
//
// This module contains comprehensive file I/O exercises for learning Rust's
// standard library file operations. Each submodule focuses on a specific
// aspect of file I/O operations.

pub mod atomic_operations;
pub mod binary_handler;
pub mod csv_validator;
pub mod directory_sync;
pub mod metadata_analyzer;
pub mod text_processor;

// Re-export the main functions for convenience
pub use atomic_operations::safe_file_replace;
pub use binary_handler::xor_encrypt_file;
pub use csv_validator::process_csv_with_validation;
pub use directory_sync::sync_directories;
pub use metadata_analyzer::{analyze_directory, FileStats};
pub use text_processor::process_text_file;
