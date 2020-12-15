use super::ClassicStack;
use crate::traits::BasicStackOperations;

impl<T: Clone> ClassicStack<T> {
    pub(super) fn move_down_after_binop(&mut self) {
        self.y = std::mem::replace(&mut self.z, self.t.clone());
    }
}

impl<T: num_traits::Zero + Clone> BasicStackOperations for ClassicStack<T> {
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
    /// stack.drop();
    ///
    /// assert_eq!(stack.x(), 2);
    /// assert_eq!(stack.y(), 3);
    /// assert_eq!(stack.z(), 4);
    /// assert_eq!(stack.t(), 4);
    /// ```
    fn drop(&mut self) {
        self.x = std::mem::replace(&mut self.y, std::mem::replace(&mut self.z, self.t.clone()));
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
    /// stack.rotate_up();
    ///
    /// assert_eq!(stack.x(), 4);
    /// assert_eq!(stack.y(), 1);
    /// assert_eq!(stack.z(), 2);
    /// assert_eq!(stack.t(), 3);
    /// ```
    ///
    /// Four rotations in a row result in the same stack as before:
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<u32>::new(1, 2, 3, 4);
    /// for x in 0..4 { stack.rotate_up(); }
    ///
    /// assert_eq!(stack.x(), 1);
    /// assert_eq!(stack.y(), 2);
    /// assert_eq!(stack.z(), 3);
    /// assert_eq!(stack.t(), 4);
    /// ```
    ///
    fn rotate_up(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y);
        std::mem::swap(&mut self.x, &mut self.z);
        std::mem::swap(&mut self.x, &mut self.t);
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
    /// stack.rotate_down();
    ///
    /// assert_eq!(stack.x(), 2);
    /// assert_eq!(stack.y(), 3);
    /// assert_eq!(stack.z(), 4);
    /// assert_eq!(stack.t(), 1);
    /// ```
    ///
    /// Four rotations in a row result in the same stack as before:
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<u32>::new(1, 2, 3, 4);
    /// for x in 0..4 { stack.rotate_down(); }
    ///
    /// assert_eq!(stack.x(), 1);
    /// assert_eq!(stack.y(), 2);
    /// assert_eq!(stack.z(), 3);
    /// assert_eq!(stack.t(), 4);
    /// ```
    ///
    fn rotate_down(&mut self) {
        std::mem::swap(&mut self.x, &mut self.t);
        std::mem::swap(&mut self.x, &mut self.z);
        std::mem::swap(&mut self.x, &mut self.y);
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
    /// stack.swap();
    ///
    /// assert_eq!(stack.x(), 2);
    /// assert_eq!(stack.y(), 1);
    /// assert_eq!(stack.z(), 3);
    /// assert_eq!(stack.t(), 4);
    /// ```
    ///
    /// Two swaps in a row result in the same stack as before:
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<u32>::new(1, 2, 3, 4);
    /// for x in 0..2 { stack.swap(); }
    ///
    /// assert_eq!(stack.x(), 1);
    /// assert_eq!(stack.y(), 2);
    /// assert_eq!(stack.z(), 3);
    /// assert_eq!(stack.t(), 4);
    /// ```
    ///
    fn swap(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y);
    }

    /// Pop a value from the X register, shifting other registers down and cloning the T register.
    ///
    /// ClassicStack will always return a value, signature has an Option since the infinite stack
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
    /// assert_eq!(res, Some(1));
    /// assert_eq!(stack.x(), 2);
    /// assert_eq!(stack.y(), 3);
    /// assert_eq!(stack.z(), 4);
    /// assert_eq!(stack.t(), 4);
    /// ```
    fn pop(&mut self) -> Option<Self::Elem> {
        Some(std::mem::replace(
            &mut self.x,
            std::mem::replace(&mut self.y, std::mem::replace(&mut self.z, self.t.clone())),
        ))
    }

    fn push(&mut self, value: Self::Elem) {
        self.t = std::mem::replace(
            &mut self.z,
            std::mem::replace(&mut self.y, std::mem::replace(&mut self.x, value)),
        );
    }

    fn clear(&mut self) {
        use num_traits::identities::zero;
        self.x = zero();
        self.y = zero();
        self.z = zero();
        self.t = zero();
    }

    /// Appy a unary operation to the X register in-place.
    ///
    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<u32>::new(1, 2, 3, 4);
    /// stack.unary_op_inplace(|x: &mut u32| {*x += 10; } );
    ///
    /// assert_eq!(stack.x(), 11);
    /// assert_eq!(stack.y(), 2);
    /// assert_eq!(stack.z(), 3);
    /// assert_eq!(stack.t(), 4);
    /// ```
    fn unary_op_inplace<U: FnOnce(&mut Self::Elem)>(&mut self, unary_fn: U) {
        unary_fn(&mut self.x);
    }

    /// Appy a binary operation to the X and Y registers, consuming them.
    /// Leave the result in X, shift other registers down, cloning the T register.
    ///
    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = ClassicStack::<u32>::new(10, 20, 3, 4);
    /// stack.binary_op_inplace(|x: &mut u32, y: &u32| {*x += *y; } );
    ///
    /// assert_eq!(stack.x(), 30);
    /// assert_eq!(stack.y(), 3);
    /// assert_eq!(stack.z(), 4);
    /// assert_eq!(stack.t(), 4);
    /// ```
    fn binary_op_inplace<U: FnOnce(&mut Self::Elem, &Self::Elem)>(&mut self, binary_fn: U) {
        binary_fn(&mut self.x, &self.y);
        self.y = std::mem::replace(&mut self.z, self.t.clone());
    }
}
