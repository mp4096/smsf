//! Dynamic-sized RPL-like stack

/// Implementation of the [BasicStackOperations](crate::stack::BasicStackOperations) trait
mod basic_stack_operations_impl;
/// Implementation of the [InPlaceFnApplication](crate::stack::InPlaceFnApplication) trait
mod in_place_fn_application_impl;
/// Data type definitions
mod types;

pub use types::DynamicSizedStack;

impl<T: num_traits::Float> crate::stack::FloatMathOperations for DynamicSizedStack<T> {}
impl<T: Clone + num_traits::NumAssignRef + num_traits::Signed> crate::stack::BasicMathOperations
    for DynamicSizedStack<T>
{
}
