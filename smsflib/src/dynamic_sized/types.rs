#[derive(Debug)]
pub struct DynamicSizedStack<T> {
    pub(super) container: Vec<T>,
}

impl<T> DynamicSizedStack<T> {
    /// Create a new empty stack
    ///
    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let stack = DynamicSizedStack::<i32>::new();
    ///
    /// assert_eq!(stack.len(), 0);
    /// ```
    pub fn new() -> Self {
        DynamicSizedStack {
            container: Vec::new(),
        }
    }

    /// Get current stack size
    ///
    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let stack = DynamicSizedStack::<i32>::new();
    ///
    /// assert_eq!(stack.len(), 0);
    /// ```
    pub fn len(&self) -> usize {
        self.container.len()
    }
}
//
// impl<T: std::fmt::Display> std::fmt::Display for DynamicSizedStack<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "T: {}\nZ: {}\nY: {}\nX: {}\n",
//             self.t, self.z, self.y, self.x
//         )
//     }
// }
