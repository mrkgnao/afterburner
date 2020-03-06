pub use crate::{diff::*, patch::*};

pub trait Diffable: Patch + Sized {
  fn diff(&self, post: &Self) -> Diff<Self>;
}

impl Diffable for () {
  fn diff(&self, _post: &Self) -> Diff<Self> {
    Diff(())
  }
}
