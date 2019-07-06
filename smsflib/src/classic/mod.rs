//! Classic HP-like stack with a fixed size of four registers: X, Y, Z, T

/// Implementation of the [BasicStackOperations](crate::traits::BasicStackOperations) trait
mod basic_ops;
/// Implementation of the [MathOperations](crate::traits::MathOperations) trait
mod math_ops;
/// Implementation of the [TrigOperations](crate::traits::TrigOperations) trait
mod trig_ops;
/// Data type definitions
mod types;

pub use types::ClassicStack;
