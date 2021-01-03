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

    fn unary_op_inplace<U: FnOnce(&mut Self::Elem)>(
        &mut self,
        unary_fn: U,
    ) -> Result<(), SmsfError>;

    fn binary_op_inplace_first_arg<U: FnOnce(&mut Self::Elem, &Self::Elem)>(
        &mut self,
        binary_fn: U,
    ) -> Result<(), SmsfError>;

    fn binary_op_inplace_second_arg<U: FnOnce(&Self::Elem, &mut Self::Elem)>(
        &mut self,
        binary_fn: U,
    ) -> Result<(), SmsfError>;
}

pub trait BasicMathOperations {
    fn add(&mut self) -> Result<(), SmsfError>;

    fn subtract(&mut self) -> Result<(), SmsfError>;

    fn multiply(&mut self) -> Result<(), SmsfError>;

    fn divide(&mut self) -> Result<(), SmsfError>;

    fn change_sign(&mut self) -> Result<(), SmsfError>;

    fn absolute_value(&mut self) -> Result<(), SmsfError>;
}

pub trait LogExpOperations {
    fn pow(&mut self) -> Result<(), SmsfError>;

    fn ln(&mut self) -> Result<(), SmsfError>;

    fn log2(&mut self) -> Result<(), SmsfError>;

    fn log10(&mut self) -> Result<(), SmsfError>;

    fn exp(&mut self) -> Result<(), SmsfError>;

    fn exp2(&mut self) -> Result<(), SmsfError>;
}

pub trait TrigOperations {
    fn sin(&mut self) -> Result<(), SmsfError>;

    fn cos(&mut self) -> Result<(), SmsfError>;

    fn tan(&mut self) -> Result<(), SmsfError>;

    fn asin(&mut self) -> Result<(), SmsfError>;

    fn acos(&mut self) -> Result<(), SmsfError>;

    fn atan(&mut self) -> Result<(), SmsfError>;

    fn atan2(&mut self) -> Result<(), SmsfError>;
}
