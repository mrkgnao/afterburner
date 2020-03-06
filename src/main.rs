pub mod atomic;
pub mod diff;
pub mod last;
pub mod monoid;
pub mod patch;
pub mod semigroup;
pub mod delta;
pub mod jet;
pub mod vec;

pub use crate::{last::*, monoid::*, patch::*, delta::*};
use std::ops::Add;

fn main() {
  println!("Hello, world!");
}
