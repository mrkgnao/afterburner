use crate::delta::*;
use crate::monoid::*;
use crate::patch::*;

pub struct Jet<A: Patch> {
  pos: A,
  vel: Delta<A>,
}

pub fn constant<A: Monoid + Patch>(value: A) -> Jet<A> {
  Jet {
    pos: value,
    vel: Delta::<A>::identity(),
  }
}

impl<A: Patch> Jet<A> {
  pub fn change(self: Jet<A>, delta: Delta<A>) -> Jet<A> {
    Jet {
      pos: self.pos.patch(&delta.0),
      vel: self.vel,
    }
  }
}
