use super::types::ClassicStack;
use crate::error::Error as SmsfError;
use crate::stack::BasicStackOperations;

impl<T: num_traits::Zero + Clone> BasicStackOperations for ClassicStack<T> {
    /// # Note
    /// All functions always return [Ok], since the stack has fixed size.

    type Elem = T;

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
    ///
    /// ```text
    /// T───┬─T
    /// Z──╮╰─T
    /// Y─╮╰──Z
    /// X ╰───Y
    /// ↓
    /// ```
    /// Note: [ClassicStack] will always return a value, signature has a [Result] since a
    /// [DynamicSizedStack](crate::stack::DynamicSizedStack) can be empty.
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
