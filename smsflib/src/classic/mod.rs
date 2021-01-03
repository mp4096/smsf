//! Classic HP-like stack with a fixed size of four registers: X, Y, Z, T

/// Implementation of the [BasicStackOperations](crate::traits::BasicStackOperations) trait
mod basic_ops;
/// Data type definitions
mod types;

pub use types::ClassicStack;

impl<T: num_traits::Float> crate::traits::FloatMathOperations for ClassicStack<T> {}
impl<T: Clone + num_traits::NumAssignRef + num_traits::Signed> crate::traits::BasicMathOperations
    for ClassicStack<T>
{
}
