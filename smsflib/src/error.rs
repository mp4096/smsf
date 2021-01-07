#[derive(Debug, PartialEq)]
pub enum StackError {
    NotEnoughOperands {
        num_required: usize,
        num_available: usize,
    },
    Other,
}
