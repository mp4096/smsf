pub trait BasicStack {
    type Elem;

    fn drop(&mut self);

    fn rotate_up(&mut self);

    fn rotate_down(&mut self);

    fn swap(&mut self);

    fn pop(&mut self) -> Self::Elem;

    fn push(&mut self, new: Self::Elem);

    fn clear(&mut self);
}
