pub trait BasicStackOperations {
    type Elem;

    fn drop(&mut self);

    fn rotate_up(&mut self);

    fn rotate_down(&mut self);

    fn swap(&mut self);

    fn pop(&mut self) -> Option<Self::Elem>;

    fn push(&mut self, value: Self::Elem);

    fn clear(&mut self);

    fn unary_op_inplace<U: FnOnce(&mut Self::Elem)>(&mut self, unary_fn: U);

    fn binary_op_inplace_first_arg<U: FnOnce(&mut Self::Elem, &Self::Elem)>(
        &mut self,
        binary_fn: U,
    );

    fn binary_op_inplace_second_arg<U: FnOnce(&Self::Elem, &mut Self::Elem)>(
        &mut self,
        binary_fn: U,
    );
}

pub trait BasicMathOperations {
    fn add(&mut self);

    fn subtract(&mut self);

    fn multiply(&mut self);

    fn divide(&mut self);

    fn change_sign(&mut self);

    fn absolute_value(&mut self);
}

pub trait LogExpOperations {
    fn pow(&mut self);

    fn ln(&mut self);

    fn log2(&mut self);

    fn log10(&mut self);

    fn exp(&mut self);

    fn exp2(&mut self);
}

pub trait TrigOperations {
    fn sin(&mut self);

    fn cos(&mut self);

    fn tan(&mut self);

    fn asin(&mut self);

    fn acos(&mut self);

    fn atan(&mut self);

    fn atan2(&mut self);
}
