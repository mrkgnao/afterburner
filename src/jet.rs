use crate::{absorb::*, diff::*, patch::*};

pub struct Jet<A: Patch> {
  pos: A,
  vel: Diff<A>,
}

pub fn constant<A: Absorb + Patch>(value: A) -> Jet<A> {
  Jet {
    pos: value,
    vel: Diff::<A>::nil(),
  }
}

impl<A: Patch> Jet<A> {
  pub fn diff(&mut self, diff: Diff<A>) {
    self.pos.patch(diff);
  }
}
