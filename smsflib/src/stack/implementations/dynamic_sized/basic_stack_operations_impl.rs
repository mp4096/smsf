use super::DynamicSizedStack;
use crate::stack::BasicStackOperations;

impl<T: Clone> BasicStackOperations for DynamicSizedStack<T> {
    type Elem = T;

    /// Rotate stack up.
    ///
    /// ```text
    /// ╭─Z ╭───Y
    /// │ Y─╯   ┊
    /// │ ┊  ╭──B
    /// │ B──╯╭─A
    /// │ A───╯ Z
    /// ╰───────╯
    /// ```
    /// No-op if the stack is empty or has only one element.
    ///
    /// # Examples
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = DynamicSizedStack::<u32>::clone_from_slice(&[6, 5, 4, 3, 2, 1]);
    /// let res = stack.rotate_up();
    ///
    /// assert_eq!(res, Ok(()));
    ///
    /// assert_eq!(stack.len(), 6);
    /// assert_eq!(stack.get(0), Some(&6));
    /// assert_eq!(stack.get(1), Some(&1));
    /// assert_eq!(stack.get(2), Some(&2));
    /// assert_eq!(stack.get(3), Some(&3));
    /// assert_eq!(stack.get(4), Some(&4));
    /// assert_eq!(stack.get(5), Some(&5));
    /// ```
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = DynamicSizedStack::<u32>::new();
    ///
    /// let res = stack.rotate_up();
    /// assert_eq!(res, Ok(()));
    /// assert_eq!(stack.is_empty(), true);
    /// ```
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = DynamicSizedStack::<u32>::clone_from_slice(&[1]);
    ///
    /// let res = stack.rotate_up();
    /// assert_eq!(res, Ok(()));
    /// assert_eq!(stack.len(), 1);
    /// assert_eq!(stack.get(0), Some(&1));
    /// ```
    ///
    fn rotate_up(&mut self) -> Result<(), crate::StackError> {
        if !self.is_empty() {
            self.container.rotate_left(1);
        }
        Ok(())
    }

    /// Rotate stack down.
    ///
    /// ```text
    /// ╭───────╮
    /// │ Z───╮ A
    /// │ Y──╮╰─Z
    /// │ ┊  ╰──Y
    /// │ B─╮   ┊
    /// ╰─A ╰───B
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = DynamicSizedStack::<u32>::clone_from_slice(&[6, 5, 4, 3, 2, 1]);
    /// let res = stack.rotate_down();
    ///
    /// assert_eq!(res, Ok(()));
    ///
    /// assert_eq!(stack.len(), 6);
    /// assert_eq!(stack.get(0), Some(&2));
    /// assert_eq!(stack.get(1), Some(&3));
    /// assert_eq!(stack.get(2), Some(&4));
    /// assert_eq!(stack.get(3), Some(&5));
    /// assert_eq!(stack.get(4), Some(&6));
    /// assert_eq!(stack.get(5), Some(&1));
    /// ```
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = DynamicSizedStack::<u32>::new();
    ///
    /// let res = stack.rotate_down();
    /// assert_eq!(res, Ok(()));
    /// assert_eq!(stack.is_empty(), true);
    /// ```
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = DynamicSizedStack::<u32>::clone_from_slice(&[1]);
    ///
    /// let res = stack.rotate_down();
    /// assert_eq!(res, Ok(()));
    /// assert_eq!(stack.len(), 1);
    /// assert_eq!(stack.get(0), Some(&1));
    /// ```
    ///
    fn rotate_down(&mut self) -> Result<(), crate::StackError> {
        if !self.is_empty() {
            self.container.rotate_right(1);
        }
        Ok(())
    }

    /// Swap two lowermost registers.
    ///
    /// ```text
    ///  Z───Z
    ///  Y───Y
    ///  ┊   ┊
    ///  C───C
    ///  B─╮ A─╮
    ///  A ╰─B │
    ///  ╰─────╯
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = DynamicSizedStack::<u32>::clone_from_slice(&[4, 3, 2, 1]);
    /// let res = stack.swap();
    ///
    /// assert_eq!(res, Ok(()));
    ///
    /// assert_eq!(stack.len(), 4);
    /// assert_eq!(stack.get(0), Some(&2));
    /// assert_eq!(stack.get(1), Some(&1));
    /// assert_eq!(stack.get(2), Some(&3));
    /// assert_eq!(stack.get(3), Some(&4));
    /// ```
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = DynamicSizedStack::<u32>::clone_from_slice(&[1]);
    /// let res = stack.swap();
    ///
    /// assert_eq!(res, Err(smsflib::StackError::NotEnoughOperands{num_required: 2, num_available: 1}));
    ///
    /// assert_eq!(stack.len(), 1);
    /// assert_eq!(stack.get(0), Some(&1));
    /// ```
    ///
    fn swap(&mut self) -> Result<(), crate::StackError> {
        if self.len() >= 2 {
            let idx_ultimate = self.len() - 1;
            let idx_penultimate = self.len() - 2;
            self.container.swap(idx_penultimate, idx_ultimate);
            Ok(())
        } else {
            Err(crate::StackError::NotEnoughOperands {
                num_required: 2,
                num_available: self.len(),
            })
        }
    }

    /// Pop a value from the lowermost register, shifting other registers down.
    ///
    /// Return [Err] if the stack is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = DynamicSizedStack::<u32>::clone_from_slice(&[4, 3, 2, 1]);
    /// let res = stack.pop();
    ///
    /// assert_eq!(res, Ok(1));
    ///
    /// assert_eq!(stack.len(), 3);
    /// assert_eq!(stack.get(0), Some(&2));
    /// assert_eq!(stack.get(1), Some(&3));
    /// assert_eq!(stack.get(2), Some(&4));
    /// ```
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = DynamicSizedStack::<u32>::new();
    /// let res = stack.pop();
    ///
    /// assert_eq!(res, Err(smsflib::StackError::NotEnoughOperands{ num_required: 1, num_available: 0 }));
    /// ```
    ///
    fn pop(&mut self) -> Result<Self::Elem, crate::StackError> {
        match self.container.pop() {
            Some(e) => Ok(e),
            None => Err(crate::StackError::NotEnoughOperands {
                num_required: 1,
                num_available: 0,
            }),
        }
    }

    /// Push a value into the lowermost register, increasing stack size by one.
    ///
    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = DynamicSizedStack::<u32>::new();
    /// let res = stack.push(1);
    ///
    /// assert_eq!(res, Ok(()));
    ///
    /// assert_eq!(stack.len(), 1);
    ///
    /// let res = stack.push(2);
    ///
    /// assert_eq!(stack.len(), 2);
    /// assert_eq!(stack.get(0), Some(&2));
    /// assert_eq!(stack.get(1), Some(&1));
    /// ```
    ///
    fn push(&mut self, value: Self::Elem) -> Result<(), crate::StackError> {
        self.container.push(value);
        Ok(())
    }

    /// Set all registers to zero.
    ///
    /// # Example
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = DynamicSizedStack::<u32>::clone_from_slice(&[1, 1, 1]);
    /// let res = stack.clear();
    ///
    /// assert_eq!(res, Ok(()));
    ///
    /// assert_eq!(stack.is_empty(), true);
    /// ```
    ///
    fn clear(&mut self) -> Result<(), crate::StackError> {
        self.container.clear();
        Ok(())
    }
}
