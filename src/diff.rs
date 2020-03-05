pub use crate::patch::*;

pub trait Diff: Patch {
    fn diff(&self, post: &Self) -> Self::Delta;
}

impl Diff for () {
    fn diff(&self, _post: &Self) -> Self::Delta {
        ()
    }
}

