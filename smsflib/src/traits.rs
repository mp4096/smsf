pub trait BasicStackOperations {
    type Elem;

    fn drop(&mut self);

    fn rotate_up(&mut self);

    fn rotate_down(&mut self);

    fn swap(&mut self);

    fn pop(&mut self) -> Self::Elem;

    fn push(&mut self, new: Self::Elem);

    fn clear(&mut self);
}

pub trait MathOperations {
    fn add(&mut self);

    fn subtract(&mut self);

    fn multiply(&mut self);

    fn divide(&mut self);

    fn change_sign(&mut self);

    fn power(&mut self);

    fn log(&mut self);

    fn log10(&mut self);
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
