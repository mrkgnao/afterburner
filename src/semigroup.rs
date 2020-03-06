/// A type with an associative binary operation on it.
pub trait Semigroup {
  /// Combine two elements
  fn append(self, other: Self) -> Self;
}

impl Semigroup for () {
  fn append(self, _other: ()) -> () {
    ()
  }
}

/// Pointwise
impl<A: Semigroup, B: Semigroup> Semigroup for (A, B) {
  fn append(self, (a2, b2): (A, B)) -> (A, B) {
    let (a1, b1) = self;
    (a1.append(a2), b1.append(b2))
  }
}
