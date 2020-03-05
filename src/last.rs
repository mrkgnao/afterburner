use crate::monoid::*;

pub struct Last<T>(pub Option<T>);

impl<T> Semigroup for Last<T> {
    fn append(self, other: Last<T>) -> Last<T> {
        match other.0 {
            None => self,
            Some(_) => other,
        }
    }
}

impl<T> Monoid for Last<T> {
    fn identity() -> Last<T> {
        Last(None)
    }
}

