use std::cell::{Cell, LazyCell, Ref, RefCell, RefMut};

#[derive(Debug, Default)]
pub struct DefaultCell<T> {
  // TODO(kranfix): remove when all the API of LazyCell is stable
  initialized: Cell<bool>,
  lazy: LazyCell<RefCell<T>>,
}

impl<T: Default> DefaultCell<T> {
  pub fn new() -> Self {
    DefaultCell::default()
  }

  pub fn is_initialized(&self) -> bool {
    self.initialized.get()
  }
  pub fn lazy_borrow(&self) -> Option<Ref<'_, T>> {
    match self.initialized.get() {
      true => Some(Ref::map(self.lazy.borrow(), |v| v)),
      false => None,
    }
  }

  pub fn borrow(&self) -> Ref<'_, T> {
    self.initialized.set(true);
    Ref::map(self.lazy.borrow(), |v| v)
  }
  pub fn borrow_mut(&self) -> RefMut<'_, T> {
    self.initialized.set(true);
    RefMut::map(self.lazy.borrow_mut(), |v| v)
  }
  pub fn update<F, R>(&self, f: F) -> R
  where
    F: FnOnce(&mut T) -> R,
  {
    f(&mut self.borrow_mut())
  }
}

#[cfg(test)]
mod test {
  #[test]
  fn default_cell() {}
}
