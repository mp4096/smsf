use crate::stack::InPlaceFnApplication;
use num_traits::Float;

pub trait FloatMathOperations: InPlaceFnApplication
where
    <Self as InPlaceFnApplication>::Elem: num_traits::Float,
{
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
    fn pow(&mut self) -> Result<(), crate::StackError> {
        self.binary_fn_in_place_first_arg(
            |x: &mut <Self as InPlaceFnApplication>::Elem,
             y: &<Self as InPlaceFnApplication>::Elem| {
                *x = y.powf(*x);
            },
        )
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
    fn ln(&mut self) -> Result<(), crate::StackError> {
        self.unary_fn_in_place(|x: &mut <Self as InPlaceFnApplication>::Elem| {
            *x = x.ln();
        })
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
    fn log2(&mut self) -> Result<(), crate::StackError> {
        self.unary_fn_in_place(|x: &mut <Self as InPlaceFnApplication>::Elem| {
            *x = x.log2();
        })
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
    fn log10(&mut self) -> Result<(), crate::StackError> {
        self.unary_fn_in_place(|x: &mut <Self as InPlaceFnApplication>::Elem| {
            *x = x.log10();
        })
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
    fn exp(&mut self) -> Result<(), crate::StackError> {
        self.unary_fn_in_place(|x: &mut <Self as InPlaceFnApplication>::Elem| {
            *x = x.exp();
        })
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
    fn exp2(&mut self) -> Result<(), crate::StackError> {
        self.unary_fn_in_place(|x: &mut <Self as InPlaceFnApplication>::Elem| {
            *x = x.exp2();
        })
    }

    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    /// use assert_approx_eq::assert_approx_eq;
    ///
    /// let mut stack = ClassicStack::<f64>::new(std::f64::consts::FRAC_PI_6, 1.0, 2.0, 3.0);
    /// stack.sin();
    ///
    /// assert_approx_eq!(*stack.x(), 0.5);
    /// assert_eq!(*stack.y(), 1.0);
    /// assert_eq!(*stack.z(), 2.0);
    /// assert_eq!(*stack.t(), 3.0);
    /// ```
    fn sin(&mut self) -> Result<(), crate::StackError> {
        self.unary_fn_in_place(|x: &mut <Self as InPlaceFnApplication>::Elem| {
            *x = x.sin();
        })
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
    /// assert_approx_eq!(*stack.x(), 0.5);
    /// assert_eq!(*stack.y(), 1.0);
    /// assert_eq!(*stack.z(), 2.0);
    /// assert_eq!(*stack.t(), 3.0);
    /// ```
    fn cos(&mut self) -> Result<(), crate::StackError> {
        self.unary_fn_in_place(|x: &mut <Self as InPlaceFnApplication>::Elem| {
            *x = x.cos();
        })
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
    /// assert_approx_eq!(*stack.x(), 1.0);
    /// assert_eq!(*stack.y(), 1.0);
    /// assert_eq!(*stack.z(), 2.0);
    /// assert_eq!(*stack.t(), 3.0);
    /// ```
    fn tan(&mut self) -> Result<(), crate::StackError> {
        self.unary_fn_in_place(|x: &mut <Self as InPlaceFnApplication>::Elem| {
            *x = x.tan();
        })
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
    /// assert_approx_eq!(*stack.x(), std::f64::consts::FRAC_PI_6);
    /// assert_eq!(*stack.y(), 1.0);
    /// assert_eq!(*stack.z(), 2.0);
    /// assert_eq!(*stack.t(), 3.0);
    /// ```
    fn asin(&mut self) -> Result<(), crate::StackError> {
        self.unary_fn_in_place(|x: &mut <Self as InPlaceFnApplication>::Elem| {
            *x = x.asin();
        })
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
    /// assert_approx_eq!(*stack.x(), std::f64::consts::FRAC_PI_3);
    /// assert_eq!(*stack.y(), 1.0);
    /// assert_eq!(*stack.z(), 2.0);
    /// assert_eq!(*stack.t(), 3.0);
    /// ```
    fn acos(&mut self) -> Result<(), crate::StackError> {
        self.unary_fn_in_place(|x: &mut <Self as InPlaceFnApplication>::Elem| {
            *x = x.acos();
        })
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
    /// assert_approx_eq!(*stack.x(), std::f64::consts::FRAC_PI_4);
    /// assert_eq!(*stack.y(), 1.0);
    /// assert_eq!(*stack.z(), 2.0);
    /// assert_eq!(*stack.t(), 3.0);
    /// ```
    fn atan(&mut self) -> Result<(), crate::StackError> {
        self.unary_fn_in_place(|x: &mut <Self as InPlaceFnApplication>::Elem| {
            *x = x.atan();
        })
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
    /// assert_approx_eq!(*stack.x(), std::f64::consts::FRAC_PI_6);
    /// assert_eq!(*stack.y(), 2.0);
    /// assert_eq!(*stack.z(), 3.0);
    /// assert_eq!(*stack.t(), 3.0);
    /// ```
    fn atan2(&mut self) -> Result<(), crate::StackError> {
        self.binary_fn_in_place_first_arg(
            |x: &mut <Self as InPlaceFnApplication>::Elem,
             y: &<Self as InPlaceFnApplication>::Elem| {
                *x = y.atan2(*x);
            },
        )
    }
}
