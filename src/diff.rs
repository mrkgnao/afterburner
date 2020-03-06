pub use crate::patch::*;
pub use crate::delta::*;

pub trait Diff: Patch + Sized {
  fn diff(&self, post: &Self) -> Delta<Self>;
}

impl Diff for () {
  fn diff(&self, _post: &Self) -> Delta<Self> {
    Delta(())
  }
}
