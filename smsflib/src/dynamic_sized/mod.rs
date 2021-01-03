//! Dynamic-sized RPL-like stack

/// Implementation of the [BasicStackOperations](crate::traits::BasicStackOperations) trait
mod basic_stack_operations_impl;
/// Implementation of the [InPlaceFnApplication](crate::traits::InPlaceFnApplication) trait
mod in_place_fn_application_impl;
/// Data type definitions
mod types;

pub use types::DynamicSizedStack;
