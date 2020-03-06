pub trait Absorb {
  fn nil() -> Self;
  /// Combine two elements
  fn absorb(&mut self, other: &mut Self);
}

impl Absorb for () {
  fn nil() -> () {
    ()
  }
  fn absorb(&mut self, _other: &mut ()) {}
}

/// Pointwise
impl<A: Absorb, B: Absorb> Absorb for (A, B) {
  fn nil() -> (A, B) {
    (A::nil(), B::nil())
  }
  fn absorb(&mut self, (a2, b2): &mut (A, B)) {
    let (a1, b1) = self;
    a1.absorb(a2);
    b1.absorb(b2);
  }
}

impl<A> Absorb for Vec<A> {
  fn nil() -> Vec<A> {
    Vec::default()
  }

  fn absorb(&mut self, other: &mut Vec<A>) {
    self.append(other);
  }
}
