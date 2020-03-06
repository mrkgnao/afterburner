pub mod absorb;
pub mod atomic;
pub mod diffable;
pub mod jet;
pub mod last;
pub mod patch;
pub mod vec;

pub use crate::{absorb::*, last::*, patch::*};
use std::ops::Add;

fn main() {
  println!("Hello, world!");
}
