pub mod absorb;
pub mod atomic;
pub mod diff;
pub mod diffable;
pub mod jet;
pub mod last;
pub mod patch;
pub mod vec;

pub use crate::{absorb::*, diff::*, last::*, patch::*};
use std::ops::Add;

fn main() {
  println!("Hello, world!");
}
