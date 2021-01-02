use super::ClassicStack;
use crate::traits::BasicMathOperations;
use crate::traits::BasicStackOperations;

impl<T: Copy + num_traits::NumAssignOps + num_traits::Signed> BasicMathOperations
    for ClassicStack<T>
{
    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<i32>::new(1, 2, 0, 1);
    /// stack.add();
    ///
    /// assert_eq!(stack.x(), 3);
    /// assert_eq!(stack.y(), 0);
    /// assert_eq!(stack.z(), 1);
    /// assert_eq!(stack.t(), 1);
    /// ```
    fn add(&mut self) {
        self.binary_op_inplace(|x: &mut T, y: &T| {
            *x += *y;
        });
    }

    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<i32>::new(1, 2, 0, 1);
    /// stack.subtract();
    ///
    /// assert_eq!(stack.x(), -1);
    /// assert_eq!(stack.y(), 0);
    /// assert_eq!(stack.z(), 1);
    /// assert_eq!(stack.t(), 1);
    /// ```
    fn subtract(&mut self) {
        self.binary_op_inplace(|x: &mut T, y: &T| {
            *x -= *y;
        });
    }

    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<i32>::new(6, 7, 0, 1);
    /// stack.multiply();
    ///
    /// assert_eq!(stack.x(), 42);
    /// assert_eq!(stack.y(), 0);
    /// assert_eq!(stack.z(), 1);
    /// assert_eq!(stack.t(), 1);
    /// ```
    fn multiply(&mut self) {
        self.binary_op_inplace(|x: &mut T, y: &T| {
            *x *= *y;
        });
    }

    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<i32>::new(5, 10, 0, 1);
    /// stack.divide();
    ///
    /// assert_eq!(stack.x(), 2);
    /// assert_eq!(stack.y(), 0);
    /// assert_eq!(stack.z(), 1);
    /// assert_eq!(stack.t(), 1);
    /// ```
    fn divide(&mut self) {
        self.binary_op_inplace(|x: &mut T, y: &T| {
            *x = *y / *x;
        });
    }

    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<i32>::new(1, 2, 3, 4);
    /// stack.change_sign();
    ///
    /// assert_eq!(stack.x(), -1);
    /// assert_eq!(stack.y(), 2);
    /// assert_eq!(stack.z(), 3);
    /// assert_eq!(stack.t(), 4);
    /// ```
    fn change_sign(&mut self) {
        self.unary_op_inplace(|x: &mut T| {
            *x = -(*x);
        });
    }

    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<i32>::new(-1, -2, -3, -4);
    /// stack.absolute_value();
    ///
    /// assert_eq!(stack.x(), 1);
    /// assert_eq!(stack.y(), -2);
    /// assert_eq!(stack.z(), -3);
    /// assert_eq!(stack.t(), -4);
    /// ```
    fn absolute_value(&mut self) {
        self.unary_op_inplace(|x: &mut T| {
            *x = x.abs();
        });
    }
}
