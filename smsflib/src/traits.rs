use crate::error::Error as SmsfError;
use num_traits::Float;
use num_traits::Signed;

pub trait BasicStackOperations {
    type Elem;

    fn drop(&mut self) -> Result<(), SmsfError>;

    fn rotate_up(&mut self) -> Result<(), SmsfError>;

    fn rotate_down(&mut self) -> Result<(), SmsfError>;

    fn swap(&mut self) -> Result<(), SmsfError>;

    fn pop(&mut self) -> Result<Self::Elem, SmsfError>;

    fn push(&mut self, value: Self::Elem) -> Result<(), SmsfError>;

    fn clear(&mut self) -> Result<(), SmsfError>;

    fn unary_op_inplace<U: FnOnce(&mut Self::Elem)>(
        &mut self,
        unary_fn: U,
    ) -> Result<(), SmsfError>;

    fn binary_op_inplace_first_arg<U: FnOnce(&mut Self::Elem, &Self::Elem)>(
        &mut self,
        binary_fn: U,
    ) -> Result<(), SmsfError>;

    fn binary_op_inplace_second_arg<U: FnOnce(&Self::Elem, &mut Self::Elem)>(
        &mut self,
        binary_fn: U,
    ) -> Result<(), SmsfError>;
}

pub trait BasicMathOperations: BasicStackOperations
where
    <Self as BasicStackOperations>::Elem: Clone + num_traits::NumAssignRef + num_traits::Signed,
{
    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<i32>::new(1, 2, 0, 1);
    /// stack.add();
    ///
    /// assert_eq!(*stack.x(), 3);
    /// assert_eq!(*stack.y(), 0);
    /// assert_eq!(*stack.z(), 1);
    /// assert_eq!(*stack.t(), 1);
    /// ```
    fn add(&mut self) -> Result<(), SmsfError> {
        self.binary_op_inplace_first_arg(
            |x: &mut <Self as BasicStackOperations>::Elem,
             y: &<Self as BasicStackOperations>::Elem| {
                *x += y;
            },
        )
    }

    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<i32>::new(1, 2, 0, 1);
    /// stack.subtract();
    ///
    /// assert_eq!(*stack.x(), -1);
    /// assert_eq!(*stack.y(), 0);
    /// assert_eq!(*stack.z(), 1);
    /// assert_eq!(*stack.t(), 1);
    /// ```
    fn subtract(&mut self) -> Result<(), SmsfError> {
        self.binary_op_inplace_first_arg(
            |x: &mut <Self as BasicStackOperations>::Elem,
             y: &<Self as BasicStackOperations>::Elem| {
                *x -= y;
            },
        )
    }

    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<i32>::new(6, 7, 0, 1);
    /// stack.multiply();
    ///
    /// assert_eq!(*stack.x(), 42);
    /// assert_eq!(*stack.y(), 0);
    /// assert_eq!(*stack.z(), 1);
    /// assert_eq!(*stack.t(), 1);
    /// ```
    fn multiply(&mut self) -> Result<(), SmsfError> {
        self.binary_op_inplace_first_arg(
            |x: &mut <Self as BasicStackOperations>::Elem,
             y: &<Self as BasicStackOperations>::Elem| {
                *x *= y;
            },
        )
    }

    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<i32>::new(5, 10, 0, 1);
    /// stack.divide();
    ///
    /// assert_eq!(*stack.x(), 2);
    /// assert_eq!(*stack.y(), 0);
    /// assert_eq!(*stack.z(), 1);
    /// assert_eq!(*stack.t(), 1);
    /// ```
    fn divide(&mut self) -> Result<(), SmsfError> {
        self.binary_op_inplace_second_arg(
            |x: &<Self as BasicStackOperations>::Elem,
             y: &mut <Self as BasicStackOperations>::Elem| {
                *y /= x;
            },
        )
    }

    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<i32>::new(1, 2, 3, 4);
    /// stack.change_sign();
    ///
    /// assert_eq!(*stack.x(), -1);
    /// assert_eq!(*stack.y(), 2);
    /// assert_eq!(*stack.z(), 3);
    /// assert_eq!(*stack.t(), 4);
    /// ```
    fn change_sign(&mut self) -> Result<(), SmsfError> {
        self.unary_op_inplace(|x: &mut <Self as BasicStackOperations>::Elem| {
            *x = -x.clone();
        })
    }

    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<i32>::new(-1, -2, -3, -4);
    /// stack.absolute_value();
    ///
    /// assert_eq!(*stack.x(), 1);
    /// assert_eq!(*stack.y(), -2);
    /// assert_eq!(*stack.z(), -3);
    /// assert_eq!(*stack.t(), -4);
    /// ```
    fn absolute_value(&mut self) -> Result<(), SmsfError> {
        self.unary_op_inplace(|x: &mut <Self as BasicStackOperations>::Elem| {
            *x = x.abs();
        })
    }
}

pub trait LogExpOperations: BasicStackOperations
where
    <Self as BasicStackOperations>::Elem: num_traits::Float,
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
    fn pow(&mut self) -> Result<(), SmsfError> {
        self.binary_op_inplace_first_arg(
            |x: &mut <Self as BasicStackOperations>::Elem,
             y: &<Self as BasicStackOperations>::Elem| {
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
    fn ln(&mut self) -> Result<(), SmsfError> {
        self.unary_op_inplace(|x: &mut <Self as BasicStackOperations>::Elem| {
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
    fn log2(&mut self) -> Result<(), SmsfError> {
        self.unary_op_inplace(|x: &mut <Self as BasicStackOperations>::Elem| {
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
    fn log10(&mut self) -> Result<(), SmsfError> {
        self.unary_op_inplace(|x: &mut <Self as BasicStackOperations>::Elem| {
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
    fn exp(&mut self) -> Result<(), SmsfError> {
        self.unary_op_inplace(|x: &mut <Self as BasicStackOperations>::Elem| {
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
    fn exp2(&mut self) -> Result<(), SmsfError> {
        self.unary_op_inplace(|x: &mut <Self as BasicStackOperations>::Elem| {
            *x = x.exp2();
        })
    }
}

pub trait TrigOperations: BasicStackOperations
where
    <Self as BasicStackOperations>::Elem: num_traits::Float,
{
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
    fn sin(&mut self) -> Result<(), SmsfError> {
        self.unary_op_inplace(|x: &mut <Self as BasicStackOperations>::Elem| {
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
    fn cos(&mut self) -> Result<(), SmsfError> {
        self.unary_op_inplace(|x: &mut <Self as BasicStackOperations>::Elem| {
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
    fn tan(&mut self) -> Result<(), SmsfError> {
        self.unary_op_inplace(|x: &mut <Self as BasicStackOperations>::Elem| {
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
    fn asin(&mut self) -> Result<(), SmsfError> {
        self.unary_op_inplace(|x: &mut <Self as BasicStackOperations>::Elem| {
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
    fn acos(&mut self) -> Result<(), SmsfError> {
        self.unary_op_inplace(|x: &mut <Self as BasicStackOperations>::Elem| {
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
    fn atan(&mut self) -> Result<(), SmsfError> {
        self.unary_op_inplace(|x: &mut <Self as BasicStackOperations>::Elem| {
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
    fn atan2(&mut self) -> Result<(), SmsfError> {
        self.binary_op_inplace_first_arg(
            |x: &mut <Self as BasicStackOperations>::Elem,
             y: &<Self as BasicStackOperations>::Elem| {
                *x = y.atan2(*x);
            },
        )
    }
}
