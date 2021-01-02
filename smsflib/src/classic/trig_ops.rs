use super::ClassicStack;
use crate::traits::BasicStackOperations;
use crate::traits::TrigOperations;

impl<T: num_traits::Float> TrigOperations for ClassicStack<T> {
    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    /// use assert_approx_eq::assert_approx_eq;
    ///
    /// let mut stack = ClassicStack::<f64>::new(std::f64::consts::FRAC_PI_6, 1.0, 2.0, 3.0);
    /// stack.sin();
    ///
    /// assert_approx_eq!(stack.x(), 0.5);
    /// assert_eq!(stack.y(), 1.0);
    /// assert_eq!(stack.z(), 2.0);
    /// assert_eq!(stack.t(), 3.0);
    /// ```
    fn sin(&mut self) {
        self.unary_op_inplace(|x: &mut T| {
            *x = x.sin();
        });
    }

    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    /// use assert_approx_eq::assert_approx_eq;
    ///
    /// let mut stack = ClassicStack::<f64>::new(std::f64::consts::FRAC_PI_3, 1.0, 2.0, 3.0);
    /// stack.cos();
    ///
    /// assert_approx_eq!(stack.x(), 0.5);
    /// assert_eq!(stack.y(), 1.0);
    /// assert_eq!(stack.z(), 2.0);
    /// assert_eq!(stack.t(), 3.0);
    /// ```
    fn cos(&mut self) {
        self.unary_op_inplace(|x: &mut T| {
            *x = x.cos();
        });
    }

    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    /// use assert_approx_eq::assert_approx_eq;
    ///
    /// let mut stack = ClassicStack::<f64>::new(std::f64::consts::FRAC_PI_4, 1.0, 2.0, 3.0);
    /// stack.tan();
    ///
    /// assert_approx_eq!(stack.x(), 1.0);
    /// assert_eq!(stack.y(), 1.0);
    /// assert_eq!(stack.z(), 2.0);
    /// assert_eq!(stack.t(), 3.0);
    /// ```
    fn tan(&mut self) {
        self.unary_op_inplace(|x: &mut T| {
            *x = x.tan();
        });
    }

    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    /// use assert_approx_eq::assert_approx_eq;
    ///
    /// let mut stack = ClassicStack::<f64>::new(0.5, 1.0, 2.0, 3.0);
    /// stack.asin();
    ///
    /// assert_approx_eq!(stack.x(), std::f64::consts::FRAC_PI_6);
    /// assert_eq!(stack.y(), 1.0);
    /// assert_eq!(stack.z(), 2.0);
    /// assert_eq!(stack.t(), 3.0);
    /// ```
    fn asin(&mut self) {
        self.unary_op_inplace(|x: &mut T| {
            *x = x.asin();
        });
    }

    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    /// use assert_approx_eq::assert_approx_eq;
    ///
    /// let mut stack = ClassicStack::<f64>::new(0.5, 1.0, 2.0, 3.0);
    /// stack.acos();
    ///
    /// assert_approx_eq!(stack.x(), std::f64::consts::FRAC_PI_3);
    /// assert_eq!(stack.y(), 1.0);
    /// assert_eq!(stack.z(), 2.0);
    /// assert_eq!(stack.t(), 3.0);
    /// ```
    fn acos(&mut self) {
        self.unary_op_inplace(|x: &mut T| {
            *x = x.acos();
        });
    }

    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    /// use assert_approx_eq::assert_approx_eq;
    ///
    /// let mut stack = ClassicStack::<f64>::new(1.0, 1.0, 2.0, 3.0);
    /// stack.atan();
    ///
    /// assert_approx_eq!(stack.x(), std::f64::consts::FRAC_PI_4);
    /// assert_eq!(stack.y(), 1.0);
    /// assert_eq!(stack.z(), 2.0);
    /// assert_eq!(stack.t(), 3.0);
    /// ```
    fn atan(&mut self) {
        self.unary_op_inplace(|x: &mut T| {
            *x = x.atan();
        });
    }

    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    /// use assert_approx_eq::assert_approx_eq;
    ///
    /// let mut stack = ClassicStack::<f64>::new(3.0, 3.0_f64.sqrt(), 2.0, 3.0);
    /// stack.atan2();
    ///
    /// assert_approx_eq!(stack.x(), std::f64::consts::FRAC_PI_6);
    /// assert_eq!(stack.y(), 2.0);
    /// assert_eq!(stack.z(), 3.0);
    /// assert_eq!(stack.t(), 3.0);
    /// ```
    fn atan2(&mut self) {
        self.binary_op_inplace(|x: &mut T, y: &T| {
            *x = y.atan2(*x);
        });
    }
}
