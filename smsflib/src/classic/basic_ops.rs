use super::ClassicStack;
use crate::error::Error as SmsfError;
use crate::traits::BasicStackOperations;
use crate::traits::InPlaceFnApplication;

impl<T: num_traits::Zero + Clone> BasicStackOperations for ClassicStack<T> {
    /// # Note
    /// All functions always return [Ok], since the stack has fixed size.

    type Elem = T;

    /// Drop the X register, shifting other registers down and cloning the T register.
    ///
    /// ```text
    /// T───┬─T
    /// Z──╮╰─T
    /// Y─╮╰──Z
    /// X ╰───Y
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<u32>::new(1, 2, 3, 4);
    /// let res = stack.drop();
    ///
    /// assert_eq!(res, Ok(()));
    ///
    /// assert_eq!(*stack.x(), 2);
    /// assert_eq!(*stack.y(), 3);
    /// assert_eq!(*stack.z(), 4);
    /// assert_eq!(*stack.t(), 4);
    /// ```
    fn drop(&mut self) -> Result<(), SmsfError> {
        self.x = std::mem::replace(&mut self.y, std::mem::replace(&mut self.z, self.t.clone()));
        Ok(())
    }

    /// Rotate stack up:
    ///
    /// ```text
    /// ╭─T ╭───Y
    /// │ Z─╯╭──Z
    /// │ Y──╯╭─X
    /// │ X───╯ Y
    /// ╰───────╯
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<u32>::new(1, 2, 3, 4);
    /// let res = stack.rotate_up();
    ///
    /// assert_eq!(res, Ok(()));
    ///
    /// assert_eq!(*stack.x(), 4);
    /// assert_eq!(*stack.y(), 1);
    /// assert_eq!(*stack.z(), 2);
    /// assert_eq!(*stack.t(), 3);
    /// ```
    ///
    /// Four rotations in a row result in the same stack as before:
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<u32>::new(1, 2, 3, 4);
    /// for _ in 0..4 {
    ///   let res = stack.rotate_up();
    ///   assert_eq!(res, Ok(()));
    /// }
    ///
    /// assert_eq!(*stack.x(), 1);
    /// assert_eq!(*stack.y(), 2);
    /// assert_eq!(*stack.z(), 3);
    /// assert_eq!(*stack.t(), 4);
    /// ```
    ///
    fn rotate_up(&mut self) -> Result<(), SmsfError> {
        std::mem::swap(&mut self.x, &mut self.y);
        std::mem::swap(&mut self.x, &mut self.z);
        std::mem::swap(&mut self.x, &mut self.t);
        Ok(())
    }

    /// Rotate stack down:
    ///
    /// ```text
    /// ╭───────╮
    /// │ T───╮ X
    /// │ Z──╮╰─T
    /// │ Y─╮╰──Z
    /// ╰─X ╰───Y
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<u32>::new(1, 2, 3, 4);
    /// let res = stack.rotate_down();
    ///
    /// assert_eq!(res, Ok(()));
    ///
    /// assert_eq!(*stack.x(), 2);
    /// assert_eq!(*stack.y(), 3);
    /// assert_eq!(*stack.z(), 4);
    /// assert_eq!(*stack.t(), 1);
    /// ```
    ///
    /// Four rotations in a row result in the same stack as before:
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<u32>::new(1, 2, 3, 4);
    /// for _ in 0..4 {
    ///   let res = stack.rotate_down();
    ///   assert_eq!(res, Ok(()));
    /// }
    ///
    /// assert_eq!(*stack.x(), 1);
    /// assert_eq!(*stack.y(), 2);
    /// assert_eq!(*stack.z(), 3);
    /// assert_eq!(*stack.t(), 4);
    /// ```
    ///
    fn rotate_down(&mut self) -> Result<(), SmsfError> {
        std::mem::swap(&mut self.x, &mut self.t);
        std::mem::swap(&mut self.x, &mut self.z);
        std::mem::swap(&mut self.x, &mut self.y);
        Ok(())
    }

    /// Swap the X and Y registers.
    ///
    /// ```text
    ///  T───T
    ///  Z───Z
    ///  Y─╮ X─╮
    ///  X ╰─Y │
    ///  ╰─────╯
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<u32>::new(1, 2, 3, 4);
    /// let res = stack.swap();
    ///
    /// assert_eq!(res, Ok(()));
    ///
    /// assert_eq!(*stack.x(), 2);
    /// assert_eq!(*stack.y(), 1);
    /// assert_eq!(*stack.z(), 3);
    /// assert_eq!(*stack.t(), 4);
    /// ```
    ///
    /// Two swaps in a row result in the same stack as before:
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<u32>::new(1, 2, 3, 4);
    /// for _ in 0..2 {
    ///   let res = stack.swap();
    ///   assert_eq!(res, Ok(()));
    /// }
    ///
    /// assert_eq!(*stack.x(), 1);
    /// assert_eq!(*stack.y(), 2);
    /// assert_eq!(*stack.z(), 3);
    /// assert_eq!(*stack.t(), 4);
    /// ```
    ///
    fn swap(&mut self) -> Result<(), SmsfError> {
        std::mem::swap(&mut self.x, &mut self.y);
        Ok(())
    }

    /// Pop a value from the X register, shifting other registers down and cloning the T register.
    ///
    /// [ClassicStack] will always return a value, signature has a [Result] since the infinite stack
    /// can be empty.
    ///
    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<u32>::new(1, 2, 3, 4);
    /// let res = stack.pop();
    ///
    /// assert_eq!(res, Ok(1));
    /// assert_eq!(*stack.x(), 2);
    /// assert_eq!(*stack.y(), 3);
    /// assert_eq!(*stack.z(), 4);
    /// assert_eq!(*stack.t(), 4);
    /// ```
    fn pop(&mut self) -> Result<Self::Elem, SmsfError> {
        Ok(std::mem::replace(
            &mut self.x,
            std::mem::replace(&mut self.y, std::mem::replace(&mut self.z, self.t.clone())),
        ))
    }

    /// Move a value into the X register, dropping the T register.
    ///
    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<u32>::new(1, 2, 3, 4);
    /// let res = stack.push(10);
    ///
    /// assert_eq!(res, Ok(()));
    ///
    /// assert_eq!(*stack.x(), 10);
    /// assert_eq!(*stack.y(), 1);
    /// assert_eq!(*stack.z(), 2);
    /// assert_eq!(*stack.t(), 3);
    /// ```
    fn push(&mut self, value: Self::Elem) -> Result<(), SmsfError> {
        self.t = std::mem::replace(
            &mut self.z,
            std::mem::replace(&mut self.y, std::mem::replace(&mut self.x, value)),
        );
        Ok(())
    }

    /// Set all registers to zero.
    ///
    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<u32>::new(1, 2, 3, 4);
    /// let res = stack.clear();
    ///
    /// assert_eq!(res, Ok(()));
    ///
    /// assert_eq!(*stack.x(), 0);
    /// assert_eq!(*stack.y(), 0);
    /// assert_eq!(*stack.z(), 0);
    /// assert_eq!(*stack.t(), 0);
    /// ```
    fn clear(&mut self) -> Result<(), SmsfError> {
        use num_traits::identities::zero;
        self.x = zero();
        self.y = zero();
        self.z = zero();
        self.t = zero();
        Ok(())
    }
}

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
    ) -> Result<(), SmsfError> {
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
    ) -> Result<(), SmsfError> {
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
    ) -> Result<(), SmsfError> {
        binary_fn(&self.x, &mut self.y);
        self.x = std::mem::replace(&mut self.y, std::mem::replace(&mut self.z, self.t.clone()));
        Ok(())
    }
}
