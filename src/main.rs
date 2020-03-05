pub mod semigroup;
pub mod monoid;
pub mod last;
pub mod atomic;
pub mod patch;
pub mod diff;

use std::ops::Add;
use crate::monoid::*;
use crate::last::*;
use crate::patch::*;

fn main() {
    println!("Hello, world!");
}

type Delta<A> = <A as Patch>::Delta;

pub struct Jet<A: Patch> {
    pos: A,
    vel: Delta<A>,
}

pub fn constant<A: Patch>(value: A) -> Jet<A> {
    Jet {
        pos: value, vel: Delta::<A>::identity(),
    }
}

impl<A: Patch> Jet<A> {
    pub fn change(self: Jet<A>, delta: Delta<A>) -> Jet<A> {
        Jet {
            pos: self.pos.patch(&delta),
            vel: self.vel,
        }
    }
}
