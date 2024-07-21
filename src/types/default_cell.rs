use std::cell::{Ref, RefCell, RefMut};

pub struct DefaultCell<T>(RefCell<Option<T>>);

impl<T> Default for DefaultCell<T> {
  fn default() -> Self {
    DefaultCell(RefCell::new(None))
  }
}
impl<T> DefaultCell<T> {
  pub fn new() -> Self {
    DefaultCell::default()
  }
}

impl<T: Default> DefaultCell<T> {
  pub fn borrow(&self) -> Ref<'_, T> {
    self.init();
    Ref::map(self.0.borrow(), |v| v.as_ref().unwrap())
  }
  pub fn borrow_mut(&self) -> RefMut<'_, T> {
    self.init();
    RefMut::map(self.0.borrow_mut(), |v| v.as_mut().unwrap())
  }
  pub fn update<F, R>(&self, f: F) -> R
  where
    F: FnOnce(&mut T) -> R,
  {
    f(&mut self.borrow_mut())
  }
  pub fn is_initialized(&self) -> bool {
    self.0.borrow().is_some()
  }

  fn init(&self) {
    let mut inner = self.0.borrow_mut();
    if inner.is_none() {
      *inner = Some(T::default());
    }
  }
}

#[cfg(test)]
mod test {
  #[test]
  fn default_cell() {}
}
