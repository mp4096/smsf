#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughOperands {
        num_required: usize,
        num_available: usize,
    },
    Other,
}
