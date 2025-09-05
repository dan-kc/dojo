// Ownership and Borrowing practice questions module
// Topics covered:
// - Move semantics and ownership transfer
// - Borrowing rules and lifetime analysis
// - Mutable vs immutable references
// - Reference counting with Rc<T>
// - Interior mutability with RefCell<T>
// - Clone vs Copy traits
// - Drop trait and RAII

// Move semantics patterns
pub mod process_owned_string;
pub mod config_builder;

// Original multi-question files (to be split)
pub mod move_semantics;

// Future modules (to be implemented)
// pub mod borrowing_rules;
// pub mod reference_counting;
// pub mod interior_mutability;
// pub mod clone_copy_traits;
// pub mod drop_raii;