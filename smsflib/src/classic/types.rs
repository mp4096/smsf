#[derive(Debug)]
pub struct ClassicStack<T> {
    pub(super) x: T,
    pub(super) y: T,
    pub(super) z: T,
    pub(super) t: T,
}

impl<T> ClassicStack<T> {
    /// Create a new stack with given values
    ///
    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let stack = ClassicStack::<i32>::new(1, 2, 3, 4);
    ///
    /// assert_eq!(*stack.x(), 1);
    /// assert_eq!(*stack.y(), 2);
    /// assert_eq!(*stack.z(), 3);
    /// assert_eq!(*stack.t(), 4);
    /// ```
    pub fn new(x: T, y: T, z: T, t: T) -> Self {
        ClassicStack { x, y, z, t }
    }

    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn y(&self) -> &T {
        &self.y
    }

    pub fn z(&self) -> &T {
        &self.z
    }

    pub fn t(&self) -> &T {
        &self.t
    }
}

impl<T: num_traits::Zero> ClassicStack<T> {
    /// Create a new stack filled with zero values
    ///
    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let stack = ClassicStack::<i32>::new_zero();
    ///
    /// assert_eq!(*stack.x(), 0);
    /// assert_eq!(*stack.y(), 0);
    /// assert_eq!(*stack.z(), 0);
    /// assert_eq!(*stack.t(), 0);
    /// ```
    pub fn new_zero() -> Self {
        use num_traits::identities::zero;
        ClassicStack {
            x: zero(),
            y: zero(),
            z: zero(),
            t: zero(),
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for ClassicStack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "T: {}\nZ: {}\nY: {}\nX: {}\n",
            self.t, self.z, self.y, self.x
        )
    }
}
