use crate::absorb::*;

pub struct Diff<A: ?Sized + Patch>(pub <A as Patch>::Diff);

impl<A: Absorb + Patch> Absorb for Diff<A> {
  fn nil() -> Diff<A> {
    Diff(<A as Patch>::Diff::nil())
  }
  fn absorb(&mut self, other: &mut Diff<A>) {
    self.0.absorb(&mut other.0);
  }
}

pub trait Patch {
  type Diff: Absorb;
  fn patch(&mut self, diff: Diff<Self>);
}

impl Patch for () {
  type Diff = ();
  fn patch(&mut self, _diff: Diff<Self>) {}
}
