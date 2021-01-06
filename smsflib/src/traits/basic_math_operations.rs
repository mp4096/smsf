use crate::error::Error as SmsfError;
use crate::traits::InPlaceFnApplication;
use num_traits::Signed;

pub trait BasicMathOperations: InPlaceFnApplication
where
    <Self as InPlaceFnApplication>::Elem: Clone + num_traits::NumAssignRef + num_traits::Signed,
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
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = DynamicSizedStack::<i32>::clone_from_slice(&[7, 2, 1]);
    /// let res = stack.add();
    ///
    /// assert_eq!(res, Ok(()));
    ///
    /// assert_eq!(stack.len(), 2);
    /// assert_eq!(stack.get(0), Some(&3));
    /// assert_eq!(stack.get(1), Some(&7));
    /// ```
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = DynamicSizedStack::<i32>::clone_from_slice(&[7]);
    /// let res = stack.add();
    ///
    /// assert_eq!(res, Err(smsflib::error::Error::NotEnoughOperands{ num_required: 2, num_available: 1 }));
    ///
    /// assert_eq!(stack.len(), 1);
    /// assert_eq!(stack.get(0), Some(&7));
    /// ```
    ///
    fn add(&mut self) -> Result<(), SmsfError> {
        self.binary_fn_in_place_first_arg(
            |x: &mut <Self as InPlaceFnApplication>::Elem,
             y: &<Self as InPlaceFnApplication>::Elem| {
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
        self.binary_fn_in_place_first_arg(
            |x: &mut <Self as InPlaceFnApplication>::Elem,
             y: &<Self as InPlaceFnApplication>::Elem| {
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
        self.binary_fn_in_place_first_arg(
            |x: &mut <Self as InPlaceFnApplication>::Elem,
             y: &<Self as InPlaceFnApplication>::Elem| {
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
        self.binary_fn_in_place_second_arg(
            |x: &<Self as InPlaceFnApplication>::Elem,
             y: &mut <Self as InPlaceFnApplication>::Elem| {
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
        self.unary_fn_in_place(|x: &mut <Self as InPlaceFnApplication>::Elem| {
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
        self.unary_fn_in_place(|x: &mut <Self as InPlaceFnApplication>::Elem| {
            *x = x.abs();
        })
    }
}
