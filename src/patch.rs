use crate::monoid::*;

pub trait Patch {
    type Delta: Monoid;
    fn patch(&self, change: &Self::Delta) -> Self;
}

impl Patch for () {
    type Delta = ();
    fn patch(&self, _change: &Self::Delta) -> Self {
        ()
    }
}

