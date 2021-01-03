use super::ClassicStack;
use crate::traits::BasicStackOperations;
use crate::traits::LogExpOperations;

use num_traits::Float;

impl<T: Float> LogExpOperations for ClassicStack<T> {
    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    /// use assert_approx_eq::assert_approx_eq;
    ///
    /// let mut stack = ClassicStack::<f64>::new(2.0, 10.0, 2.0, 3.0);
    /// stack.pow();
    ///
    /// assert_approx_eq!(*stack.x(), 100.0);
    /// assert_eq!(*stack.y(), 2.0);
    /// assert_eq!(*stack.z(), 3.0);
    /// assert_eq!(*stack.t(), 3.0);
    /// ```
    fn pow(&mut self) {
        self.binary_op_inplace_first_arg(|x: &mut T, y: &T| {
            *x = y.powf(*x);
        });
    }

    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    /// use assert_approx_eq::assert_approx_eq;
    ///
    /// let mut stack = ClassicStack::<f64>::new(42.0, 1.0, 2.0, 3.0);
    /// stack.ln();
    ///
    /// assert_approx_eq!(*stack.x(), 3.73766961828);
    /// assert_eq!(*stack.y(), 1.0);
    /// assert_eq!(*stack.z(), 2.0);
    /// assert_eq!(*stack.t(), 3.0);
    /// ```
    fn ln(&mut self) {
        self.unary_op_inplace(|x: &mut T| {
            *x = x.ln();
        });
    }

    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    /// use assert_approx_eq::assert_approx_eq;
    ///
    /// let mut stack = ClassicStack::<f64>::new(64.0, 1.0, 2.0, 3.0);
    /// stack.log2();
    ///
    /// assert_approx_eq!(*stack.x(), 6.0);
    /// assert_eq!(*stack.y(), 1.0);
    /// assert_eq!(*stack.z(), 2.0);
    /// assert_eq!(*stack.t(), 3.0);
    /// ```
    fn log2(&mut self) {
        self.unary_op_inplace(|x: &mut T| {
            *x = x.log2();
        });
    }

    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    /// use assert_approx_eq::assert_approx_eq;
    ///
    /// let mut stack = ClassicStack::<f64>::new(100.0, 1.0, 2.0, 3.0);
    /// stack.log10();
    ///
    /// assert_approx_eq!(*stack.x(), 2.0);
    /// assert_eq!(*stack.y(), 1.0);
    /// assert_eq!(*stack.z(), 2.0);
    /// assert_eq!(*stack.t(), 3.0);
    /// ```
    fn log10(&mut self) {
        self.unary_op_inplace(|x: &mut T| {
            *x = x.log10();
        });
    }

    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    /// use assert_approx_eq::assert_approx_eq;
    ///
    /// let mut stack = ClassicStack::<f64>::new(1.0, 1.0, 2.0, 3.0);
    /// stack.exp();
    ///
    /// assert_approx_eq!(*stack.x(), 2.718281828);
    /// assert_eq!(*stack.y(), 1.0);
    /// assert_eq!(*stack.z(), 2.0);
    /// assert_eq!(*stack.t(), 3.0);
    /// ```
    fn exp(&mut self) {
        self.unary_op_inplace(|x: &mut T| {
            *x = x.exp();
        });
    }

    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    /// use assert_approx_eq::assert_approx_eq;
    ///
    /// let mut stack = ClassicStack::<f64>::new(12.0, 1.0, 2.0, 3.0);
    /// stack.exp2();
    ///
    /// assert_approx_eq!(*stack.x(), 4096.0);
    /// assert_eq!(*stack.y(), 1.0);
    /// assert_eq!(*stack.z(), 2.0);
    /// assert_eq!(*stack.t(), 3.0);
    /// ```
    fn exp2(&mut self) {
        self.unary_op_inplace(|x: &mut T| {
            *x = x.exp2();
        });
    }
}
