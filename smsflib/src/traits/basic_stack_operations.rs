use crate::error::Error as SmsfError;

pub trait BasicStackOperations {
    type Elem;

    fn drop(&mut self) -> Result<(), SmsfError>;

    fn rotate_up(&mut self) -> Result<(), SmsfError>;

    fn rotate_down(&mut self) -> Result<(), SmsfError>;

    fn swap(&mut self) -> Result<(), SmsfError>;

    fn pop(&mut self) -> Result<Self::Elem, SmsfError>;

    fn push(&mut self, value: Self::Elem) -> Result<(), SmsfError>;

    fn clear(&mut self) -> Result<(), SmsfError>;
}
