use crate::delta::*;
use crate::patch::*;

pub enum VecChange<A: Patch> {
    InsertAt(usize, A),
    DeleteAt(usize),
    ModifyAt(usize, Delta<A>),
}

// impl<A: Patch> Patch for Vec<A> {
//     type Change = 
// }
