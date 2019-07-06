use crate::traits::BasicStack;
use num_traits::identities::zero;
use num_traits::Num;
use std::fmt;

#[derive(Debug)]
pub struct Stack<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub t: T,
}

impl<T: Num> Stack<T> {
    pub fn new(x: T, y: T, z: T, t: T) -> Self {
        Stack { x, y, z, t }
    }

    pub fn new_empty() -> Self {
        Stack {
            x: zero(),
            y: zero(),
            z: zero(),
            t: zero(),
        }
    }
}

impl<T: fmt::Display> fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "T: {}\nZ: {}\nY: {}\nX: {}\n", self.t, self.z, self.y, self.x)
    }
}

impl<T: Num + Copy> BasicStack for Stack<T> {
    type Elem = T;

    fn drop(&mut self) {
        self.x = self.y;
        self.y = self.z;
        self.z = self.t;
    }

    fn rotate_up(&mut self) {
        let tmp = self.t;
        self.t = self.z;
        self.z = self.y;
        self.y = self.x;
        self.x = tmp;
    }

    fn rotate_down(&mut self) {
        let tmp = self.x;
        self.x = self.y;
        self.y = self.z;
        self.z = self.t;
        self.t = tmp;
    }

    fn swap(&mut self) {
        let tmp = self.x;
        self.x = self.y;
        self.y = tmp;
    }

    fn pop(&mut self) -> Self::Elem {
        let tmp = self.x;
        self.drop();
        tmp
    }

    fn push(&mut self, new: Self::Elem) {
        self.rotate_up();
        self.x = new;
    }

    fn clear(&mut self) {
        self.x = zero();
        self.y = zero();
        self.z = zero();
        self.t = zero();
    }
}

// LCOV_EXCL_START
#[cfg(test)]
mod tests {
    use super::Stack;
    use crate::traits::BasicStack;

    #[test]
    fn test_drop() {
        let mut stack = Stack::<u32>::new(1, 2, 3, 4);
        stack.drop();

        assert_eq!(stack.x, 2);
        assert_eq!(stack.y, 3);
        assert_eq!(stack.z, 4);
        assert_eq!(stack.t, 4);
    }

    #[test]
    fn test_rotate_down() {
        let mut stack = Stack::<u32>::new(1, 2, 3, 4);
        stack.rotate_down();

        assert_eq!(stack.x, 2);
        assert_eq!(stack.y, 3);
        assert_eq!(stack.z, 4);
        assert_eq!(stack.t, 1);
    }

    #[test]
    fn test_rotate_up() {
        let mut stack = Stack::<u32>::new(1, 2, 3, 4);
        stack.rotate_up();

        assert_eq!(stack.x, 4);
        assert_eq!(stack.y, 1);
        assert_eq!(stack.z, 2);
        assert_eq!(stack.t, 3);
    }

    #[test]
    fn test_swap() {
        let mut stack = Stack::<u32>::new(1, 2, 3, 4);
        stack.swap();

        assert_eq!(stack.x, 2);
        assert_eq!(stack.y, 1);
        assert_eq!(stack.z, 3);
        assert_eq!(stack.t, 4);
    }
}
// LCOV_EXCL_STOP
