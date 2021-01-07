use super::types::ClassicStack;
use crate::stack::InPlaceFnApplication;

impl<T: Clone> InPlaceFnApplication for ClassicStack<T> {
    /// # Note
    /// All functions always return [Ok], since the stack has fixed size.

    type Elem = T;
    /// Appy a unary operation to the X register in-place.
    ///
    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<u32>::new(1, 2, 3, 4);
    /// let res = stack.unary_fn_in_place(|x: &mut u32| {*x += 10; } );
    ///
    /// assert_eq!(res, Ok(()));
    ///
    /// assert_eq!(*stack.x(), 11);
    /// assert_eq!(*stack.y(), 2);
    /// assert_eq!(*stack.z(), 3);
    /// assert_eq!(*stack.t(), 4);
    /// ```
    fn unary_fn_in_place<U: FnOnce(&mut Self::Elem)>(
        &mut self,
        unary_fn: U,
    ) -> Result<(), crate::StackError> {
        unary_fn(&mut self.x);
        Ok(())
    }

    /// Appy a binary operation to the X and Y registers, consuming them.
    /// Leave the result in X, shift other registers down, cloning the T register.
    ///
    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<u32>::new(100, 10, 3, 4);
    /// let res = stack.binary_fn_in_place_first_arg(|x: &mut u32, y: &u32| {*x -= y; } );
    ///
    /// assert_eq!(res, Ok(()));
    ///
    /// assert_eq!(*stack.x(), 90);
    /// assert_eq!(*stack.y(), 3);
    /// assert_eq!(*stack.z(), 4);
    /// assert_eq!(*stack.t(), 4);
    /// ```
    fn binary_fn_in_place_first_arg<U: FnOnce(&mut Self::Elem, &Self::Elem)>(
        &mut self,
        binary_fn: U,
    ) -> Result<(), crate::StackError> {
        binary_fn(&mut self.x, &self.y);
        self.y = std::mem::replace(&mut self.z, self.t.clone());
        Ok(())
    }

    /// Appy a binary operation to the X and Y registers, consuming them.
    /// Leave the result in X, shift other registers down, cloning the T register.
    ///
    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<u32>::new(10, 100, 3, 4);
    /// let res = stack.binary_fn_in_place_second_arg(|x: &u32, y: &mut u32| {*y -= x; } );
    ///
    /// assert_eq!(res, Ok(()));
    ///
    /// assert_eq!(*stack.x(), 90);
    /// assert_eq!(*stack.y(), 3);
    /// assert_eq!(*stack.z(), 4);
    /// assert_eq!(*stack.t(), 4);
    /// ```
    fn binary_fn_in_place_second_arg<U: FnOnce(&Self::Elem, &mut Self::Elem)>(
        &mut self,
        binary_fn: U,
    ) -> Result<(), crate::StackError> {
        binary_fn(&self.x, &mut self.y);
        self.x = std::mem::replace(&mut self.y, std::mem::replace(&mut self.z, self.t.clone()));
        Ok(())
    }
}
