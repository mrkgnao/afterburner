use crate::absorb::*;

pub struct Last<T>(pub Option<T>);

impl<T> Last<T> {}

impl<T> Absorb for Last<T> {
  fn nil() -> Last<T> {
    Last(None)
  }

  fn absorb(&mut self, other: &mut Last<T>) {
    if let Some(value) = other.0.take() {
      self.0 = Some(value);
    }
  }
}
