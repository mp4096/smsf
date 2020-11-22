pub trait BasicStackOperations {
    type Elem;

    fn drop(&mut self);

    fn rotate_up(&mut self);

    fn rotate_down(&mut self);

    fn swap(&mut self);

    fn pop(&mut self) -> Option<Self::Elem>;

    fn push(&mut self, new: Self::Elem);

    fn clear(&mut self);
}

pub trait BasicMathOperations {
    fn add(&mut self);

    fn subtract(&mut self);

    fn multiply(&mut self);

    fn divide(&mut self);

    fn change_sign(&mut self);
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
