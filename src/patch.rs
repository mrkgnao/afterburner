use crate::monoid::*;

pub trait Patch {
  type Change: Monoid;
  fn patch(&self, change: &Self::Change) -> Self;
}

impl Patch for () {
  type Change = ();
  fn patch(&self, _change: &Self::Change) -> Self {
    ()
  }
}
