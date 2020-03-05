pub use crate::semigroup::*;

pub trait Monoid: Semigroup {
    fn identity() -> Self;
}

impl Monoid for () {
    fn identity() -> Self {
        ()
    }
}

impl<A: Monoid, B: Monoid> Monoid for (A, B) {
    fn identity() -> (A, B) {
        (A::identity(), B::identity())
    }
}
