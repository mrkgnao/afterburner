pub trait Semigroup {
    fn append(self, other: Self) -> Self;
}

impl Semigroup for () {
    fn append(self, _other: ()) -> () {
        ()
    }
}

impl<A: Semigroup, B: Semigroup> Semigroup for (A, B) {
    fn append(self, (a2, b2): (A, B)) -> (A, B) {
        let (a1, b1) = self;
        (a1.append(a2), b1.append(b2))
    }
}

