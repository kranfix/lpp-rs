use std::ops::Deref;

use crate::utils::{is_digit, is_letter};

pub struct Literal<'s>(&'s str);
impl<'s> Deref for Literal<'s> {
  type Target = str;

  fn deref(&self) -> &Self::Target {
    self.0
  }
}
impl<'s> AsRef<str> for Literal<'s> {
  fn as_ref(&self) -> &str {
    self.0
  }
}

impl<'s> Literal<'s> {
  pub fn contained_at_start(text: &'s str) -> Option<Literal<'s>> {
    let mut chars = text.chars();
    let len = match chars.next() {
      Some(c) => c.len_utf8(),
      None => return None,
    };

    let len = text[len..]
      .chars()
      .take_while(|c| is_letter(*c) || is_digit(*c))
      .fold(len, |n, c| n + c.len_utf8());
    Some(Literal(&text[0..len]))
  }
}
