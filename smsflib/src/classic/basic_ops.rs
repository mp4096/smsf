use super::ClassicStack;
use crate::traits::BasicStackOperations;

impl<T: Clone> ClassicStack<T> {
    pub(super) fn move_down_after_binop(&mut self) {
        self.y = std::mem::replace(&mut self.z, self.t.clone());
    }
}

impl<T: num_traits::Zero + Clone> BasicStackOperations for ClassicStack<T> {
    type Elem = T;

    /// Drop the X register, shifting other registers down and copying the T register.
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

    fn rotate_up(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y);
        std::mem::swap(&mut self.x, &mut self.z);
        std::mem::swap(&mut self.x, &mut self.t);
    }

    fn rotate_down(&mut self) {
        std::mem::swap(&mut self.x, &mut self.t);
        std::mem::swap(&mut self.x, &mut self.z);
        std::mem::swap(&mut self.x, &mut self.y);
    }

    fn swap(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y);
    }

    /// Pop a value from the X register, shifting other registers down and copying the T register.
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

    fn unary_op_inplace<U: FnOnce(&mut Self::Elem)>(&mut self, unary_fn: U) {
        unary_fn(&mut self.x);
    }

    fn binary_op_inplace<U: FnOnce(&mut Self::Elem, &Self::Elem)>(&mut self, binary_fn: U) {
        binary_fn(&mut self.x, &self.y);
    }
}

// LCOV_EXCL_START
#[cfg(test)]
mod tests {
    use super::BasicStackOperations;
    use super::ClassicStack;

    #[test]
    fn test_drop() {
        let mut stack = ClassicStack::<u32>::new(1, 2, 3, 4);
        stack.drop();

        assert_eq!(stack.x(), 2);
        assert_eq!(stack.y(), 3);
        assert_eq!(stack.z(), 4);
        assert_eq!(stack.t(), 4);
    }

    #[test]
    fn test_rotate_down() {
        let mut stack = ClassicStack::<u32>::new(1, 2, 3, 4);
        stack.rotate_down();

        assert_eq!(stack.x(), 2);
        assert_eq!(stack.y(), 3);
        assert_eq!(stack.z(), 4);
        assert_eq!(stack.t(), 1);
    }

    #[test]
    fn test_rotate_up() {
        let mut stack = ClassicStack::<u32>::new(1, 2, 3, 4);
        stack.rotate_up();

        assert_eq!(stack.x(), 4);
        assert_eq!(stack.y(), 1);
        assert_eq!(stack.z(), 2);
        assert_eq!(stack.t(), 3);
    }

    #[test]
    fn test_swap() {
        let mut stack = ClassicStack::<u32>::new(1, 2, 3, 4);
        stack.swap();

        assert_eq!(stack.x(), 2);
        assert_eq!(stack.y(), 1);
        assert_eq!(stack.z(), 3);
        assert_eq!(stack.t(), 4);
    }
}
// LCOV_EXCL_STOP
