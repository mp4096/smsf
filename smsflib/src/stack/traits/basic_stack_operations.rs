use crate::StackError as SmsfStackError;

pub trait BasicStackOperations {
    type Elem;

    fn rotate_up(&mut self) -> Result<(), SmsfStackError>;

    fn rotate_down(&mut self) -> Result<(), SmsfStackError>;

    fn swap(&mut self) -> Result<(), SmsfStackError>;

    fn pop(&mut self) -> Result<Self::Elem, SmsfStackError>;

    fn push(&mut self, value: Self::Elem) -> Result<(), SmsfStackError>;

    fn clear(&mut self) -> Result<(), SmsfStackError>;

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
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = DynamicSizedStack::<u32>::clone_from_slice(&[3, 2, 1]);
    /// let res = stack.drop();
    ///
    /// assert_eq!(res, Ok(()));
    ///
    /// assert_eq!(stack.len(), 2);
    /// assert_eq!(stack.get(0), Some(&2));
    /// assert_eq!(stack.get(1), Some(&3));
    /// ```
    ///
    /// ```
    /// use smsflib::prelude::*;
    ///
    /// let mut stack = DynamicSizedStack::<u32>::new();
    /// let res = stack.drop();
    ///
    /// assert_eq!(res, Err(smsflib::StackError::NotEnoughOperands{num_required: 1, num_available: 0}));
    ///
    /// assert_eq!(stack.is_empty(), true);
    /// ```
    fn drop(&mut self) -> Result<(), SmsfStackError> {
        self.pop().and(Ok(()))
    }
}
