use crate::{absorb::*, patch::*};

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

impl<A: Patch> Patch for Jet<A> {
  type Diff = <A as Patch>::Diff;
  fn patch(&mut self, diff: Diff<Jet<A>>) {
    // coerce from diff<jet<A>> to diff<A>
    let diff = Diff(diff.0);
    self.pos.patch(diff);
  }
}
