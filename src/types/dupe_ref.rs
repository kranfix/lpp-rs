use std::{ops::Deref, rc::Rc, sync::Arc};

pub trait DupeRef<T: ?Sized>: Deref<Target = T> {
  fn dupe_ref(&self) -> Self;
}

impl<'a, T> DupeRef<T> for &'a T {
  fn dupe_ref(&self) -> Self {
    self
  }
}

impl<T> DupeRef<T> for Rc<T> {
  fn dupe_ref(&self) -> Self {
    self.clone()
  }
}

impl<T> DupeRef<T> for Arc<T> {
  fn dupe_ref(&self) -> Self {
    self.clone()
  }
}
