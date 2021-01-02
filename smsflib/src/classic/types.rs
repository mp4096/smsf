#[derive(Debug)]
pub struct ClassicStack<T> {
    pub(super) x: T,
    pub(super) y: T,
    pub(super) z: T,
    pub(super) t: T,
}

impl<T> ClassicStack<T> {
    pub fn new(x: T, y: T, z: T, t: T) -> Self {
        ClassicStack { x, y, z, t }
    }
}

impl<T: num_traits::Zero> ClassicStack<T> {
    pub fn new_zero() -> Self {
        use num_traits::identities::zero;
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

impl<T: std::fmt::Display> std::fmt::Display for ClassicStack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "T: {}\nZ: {}\nY: {}\nX: {}\n",
            self.t, self.z, self.y, self.x
        )
    }
}
