// String Manipulation Practice Exercises Module
//
// This module contains comprehensive string manipulation exercises for learning Rust's
// string handling capabilities. Each submodule focuses on a specific aspect of string
// processing and demonstrates important Rust concepts like String vs &str, UTF-8 handling,
// iterator patterns, memory-efficient operations, and error handling.

pub mod pattern_matcher;
pub mod string_builder;
pub mod string_formatter;
pub mod text_parser;
pub mod unicode_processor;
pub mod word_counter;

// Re-export the main functions for convenience
pub use pattern_matcher::extract_urls;
pub use string_builder::build_query_string;
pub use string_formatter::format_person_data;
pub use text_parser::parse_key_value_pairs;
pub use unicode_processor::normalize_text;
pub use word_counter::{count_words, WordStats};
