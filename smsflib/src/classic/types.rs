use num_traits::identities::zero;
use num_traits::Num;
use std::fmt;

#[derive(Debug)]
pub struct ClassicStack<T> {
    pub(super) x: T,
    pub(super) y: T,
    pub(super) z: T,
    pub(super) t: T,
}

impl<T: Num> ClassicStack<T> {
    pub fn new(x: T, y: T, z: T, t: T) -> Self {
        ClassicStack { x, y, z, t }
    }

    pub fn new_empty() -> Self {
        ClassicStack {
            x: zero(),
            y: zero(),
            z: zero(),
            t: zero(),
        }
    }
}

impl<T: Copy> ClassicStack<T> {
    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }

    pub fn z(&self) -> T {
        self.z
    }

    pub fn t(&self) -> T {
        self.t
    }
}

impl<T: fmt::Display> fmt::Display for ClassicStack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "T: {}\nZ: {}\nY: {}\nX: {}\n",
            self.t, self.z, self.y, self.x
        )
    }
}
