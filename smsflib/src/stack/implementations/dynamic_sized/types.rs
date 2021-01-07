/// Dynamic-sized RPL-like stack
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

    /// Check if the stack is empty
    ///
    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = DynamicSizedStack::<i32>::new();
    ///
    /// assert_eq!(stack.is_empty(), true);
    ///
    /// let res = stack.push(1);
    /// assert_eq!(res, Ok(()));
    ///
    /// assert_eq!(stack.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.container.is_empty()
    }

    /// Get an element by its index
    ///
    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let stack = DynamicSizedStack::<i32>::new();
    ///
    /// assert_eq!(stack.get(0), None);
    /// assert_eq!(stack.get(1), None);
    /// ```
    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx < self.len() {
            let reverse_idx = self.len() - (idx + 1);
            self.container.get(reverse_idx)
        } else {
            None
        }
    }
}

impl<T> Default for DynamicSizedStack<T> {
    fn default() -> Self {
        DynamicSizedStack::new()
    }
}

impl<T: Clone> DynamicSizedStack<T> {
    /// Create a new stack by cloning the elements of the provided slice
    ///
    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let stack = DynamicSizedStack::<i32>::clone_from_slice(&[1, 2, 3]);
    ///
    /// assert_eq!(stack.len(), 3);
    /// assert_eq!(stack.get(0), Some(&3));
    /// assert_eq!(stack.get(1), Some(&2));
    /// assert_eq!(stack.get(2), Some(&1));
    /// ```
    pub fn clone_from_slice(source: &[T]) -> Self {
        DynamicSizedStack {
            container: source.to_vec(),
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for DynamicSizedStack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, elem) in self.container.iter().enumerate() {
            writeln!(f, "{}: {}", self.len() - (idx + 1), elem)?;
        }
        Ok(())
    }
}
