use crate::monoid::*;
use crate::patch::*;

pub struct Delta<A: Patch>(pub A::Change);

impl<A: Semigroup + Patch> Semigroup for Delta<A> {
  fn append(self: Delta<A>, other: Delta<A>) -> Delta<A> {
    Delta(self.0.append(other.0))
  }
}

impl<A: Monoid + Patch> Monoid for Delta<A> {
  fn identity() -> Delta<A> {
    Delta(<A as Patch>::Change::identity())
  }
}

