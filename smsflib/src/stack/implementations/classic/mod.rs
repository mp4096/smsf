//! Classic HP-like stack with a fixed size of four registers: X, Y, Z, T

/// Implementation of the [BasicStackOperations](crate::stack::BasicStackOperations) trait
mod basic_ops;
/// Data type definitions
mod types;

pub use types::ClassicStack;

impl<T: num_traits::Float> crate::stack::FloatMathOperations for ClassicStack<T> {}
impl<T: Clone + num_traits::NumAssignRef + num_traits::Signed> crate::stack::BasicMathOperations
    for ClassicStack<T>
{
}
