use std::{ops::Deref, rc::Rc, sync::Arc};

pub trait Source: Clone {
  fn source(&self) -> &str;
}

impl<'a> Source for &'a str {
  fn source(&self) -> &str {
    self
  }
}

impl<'a> Source for &'a String {
  fn source(&self) -> &str {
    self
  }
}

impl Source for Rc<str> {
  fn source(&self) -> &str {
    self.deref()
  }
}

impl Source for Arc<str> {
  fn source(&self) -> &str {
    self.deref()
  }
}

impl Source for Rc<String> {
  fn source(&self) -> &str {
    self.deref()
  }
}

impl Source for Arc<String> {
  fn source(&self) -> &str {
    self.deref()
  }
}
