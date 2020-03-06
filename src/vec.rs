use crate::{absorb::*, patch::*};

pub struct IVec<A>(pub Vec<A>);

pub enum VecEdit<A: Patch> {
  InsertAt(usize, A),
  RemoveAt(usize),
  ModifyAt(usize, Diff<A>),
}

pub struct VecDiff<A: Patch>(pub Vec<VecEdit<A>>);

impl<A: Patch> Absorb for VecDiff<A> {
  fn nil() -> VecDiff<A> {
    VecDiff(Absorb::nil())
  }

  fn absorb(&mut self, other: &mut VecDiff<A>) {
    self.0.append(&mut other.0);
  }
}

impl<A: Patch> Patch for IVec<A> {
  type Diff = VecDiff<A>;
  fn patch(&mut self, diff: Diff<IVec<A>>) {
    for edit in (diff.0).0.into_iter() {
      match edit {
        VecEdit::RemoveAt(ix) => {
          self.0.remove(ix);
        }
        VecEdit::InsertAt(ix, val) => {
          self.0.insert(ix, val);
        }
        VecEdit::ModifyAt(ix, v_diff) => {
          self.0[ix].patch(v_diff);
        }
      }
    }
  }
}
