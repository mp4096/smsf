mod implementations;
mod traits;

pub use crate::stack::implementations::{ClassicStack, DynamicSizedStack};

pub use crate::stack::traits::{
    BasicMathOperations, BasicStackOperations, FloatMathOperations, InPlaceFnApplication,
};
