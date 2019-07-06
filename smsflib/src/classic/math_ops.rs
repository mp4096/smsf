use super::ClassicStack;
use crate::traits::BasicMathOperations;

use num_traits::{Num, NumAssignOps, Signed};

impl<T: Num + NumAssignOps + Copy + Signed> BasicMathOperations for ClassicStack<T> {
    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<i32>::new(1, 2, 0, 1);
    /// stack.add();
    ///
    /// assert_eq!(stack.x, 3);
    /// assert_eq!(stack.y, 0);
    /// assert_eq!(stack.z, 1);
    /// assert_eq!(stack.t, 1);
    /// ```
    fn add(&mut self) {
        self.x += self.y;
        self.move_down_after_binop();
    }

    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<i32>::new(1, 2, 0, 1);
    /// stack.subtract();
    ///
    /// assert_eq!(stack.x, -1);
    /// assert_eq!(stack.y, 0);
    /// assert_eq!(stack.z, 1);
    /// assert_eq!(stack.t, 1);
    /// ```
    fn subtract(&mut self) {
        self.x -= self.y;
        self.move_down_after_binop();
    }

    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<i32>::new(6, 7, 0, 1);
    /// stack.multiply();
    ///
    /// assert_eq!(stack.x, 42);
    /// assert_eq!(stack.y, 0);
    /// assert_eq!(stack.z, 1);
    /// assert_eq!(stack.t, 1);
    /// ```
    fn multiply(&mut self) {
        self.x *= self.y;
        self.move_down_after_binop();
    }

    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<i32>::new(5, 10, 0, 1);
    /// stack.divide();
    ///
    /// assert_eq!(stack.x, 2);
    /// assert_eq!(stack.y, 0);
    /// assert_eq!(stack.z, 1);
    /// assert_eq!(stack.t, 1);
    /// ```
    fn divide(&mut self) {
        self.x = self.y / self.x;
        self.move_down_after_binop();
    }

    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<i32>::new(1, 2, 3, 4);
    /// stack.change_sign();
    ///
    /// assert_eq!(stack.x, -1);
    /// assert_eq!(stack.y, 2);
    /// assert_eq!(stack.z, 3);
    /// assert_eq!(stack.t, 4);
    /// ```
    fn change_sign(&mut self) {
        self.x = -self.x;
    }
}
