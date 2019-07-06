use num_traits::identities::zero;
use num_traits::Num;
use std::fmt;

#[derive(Debug)]
pub struct Stack<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub t: T,
}

impl<T: Num> Stack<T> {
    pub fn new(x: T, y: T, z: T, t: T) -> Self {
        Stack { x, y, z, t }
    }

    pub fn new_empty() -> Self {
        Stack {
            x: zero(),
            y: zero(),
            z: zero(),
            t: zero(),
        }
    }
}

impl<T: fmt::Display> fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "T: {}\nZ: {}\nY: {}\nX: {}\n",
            self.t, self.z, self.y, self.x
        )
    }
}
