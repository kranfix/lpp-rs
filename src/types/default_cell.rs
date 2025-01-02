use std::cell::{Cell, LazyCell, Ref, RefCell, RefMut};

#[derive(Debug, Default)]
pub struct DefaultCell<T> {
  // TODO(kranfix): remove when all the API of LazyCell is stable
  initialized: Cell<bool>,
  lazy: LazyCell<RefCell<T>>,
}

impl<T: Default> DefaultCell<T> {
  pub fn lazy_borrow(&self) -> Option<Ref<'_, T>> {
    match self.initialized.get() {
      true => Some(Ref::map(self.lazy.borrow(), |v| v)),
      false => None,
    }
  }

  pub fn borrow_mut(&self) -> RefMut<'_, T> {
    self.initialized.set(true);
    RefMut::map(self.lazy.borrow_mut(), |v| v)
  }
}

#[cfg(test)]
mod test {
  #[test]
  fn default_cell() {}
}
