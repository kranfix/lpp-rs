use crate::{types::Literal, utils::is_digit};

pub trait ReadFrom<'a> {
  type Value;
  fn read_from(text: &'a str) -> Option<(usize, Self::Value)>;
}

impl<'s> ReadFrom<'s> for Literal<'s> {
  type Value = Literal<'s>;

  fn read_from(text: &'s str) -> Option<(usize, Literal<'s>)> {
    let lit = Literal::contained_at_start(text)?;
    Some((lit.len(), lit))
  }
}

impl<'s> ReadFrom<'s> for u32 {
  type Value = u32;

  fn read_from(text: &'s str) -> Option<(usize, Self::Value)> {
    let mut value = 0;
    let mut len = 0;
    let chars = text.chars().take_while(|c| is_digit(*c));
    for c in chars {
      let d = c as u32 - '0' as u32;
      value = 10 * value + d;
      len += c.len_utf8();
    }

    if len == 0 {
      None
    } else {
      Some((len, value))
    }
  }
}

impl<'s> ReadFrom<'s> for String {
  type Value = Result<String, ExtractStringError>;

  // missing support for scaping characters
  fn read_from(text: &'s str) -> Option<(usize, Self::Value)> {
    let mut len = 0;
    let mut chars = text.chars();

    match chars.next() {
      Some('"') => len += 1,
      None => return None,
      Some(_) => return None,
    }

    let mut value = String::new();

    loop {
      match chars.next() {
        Some('"') => {
          len += 1;
          break;
        }
        None => return Some((len, Err(ExtractStringError::Incomplete))),
        Some(c) => {
          len += c.len_utf8();
          value.push(c);
        }
      }
    }

    Some((len, Ok(value)))
  }
}
#[derive(Debug)]
pub enum ExtractStringError {
  Incomplete,
}

#[cfg(test)]
mod test {
  use crate::types::Literal;

  use super::ReadFrom;

  #[test]
  fn read_u32_ok() {
    let source = "1234sdf";
    let (len, val) = u32::read_from(source).unwrap();
    assert_eq!(len, 4);
    assert_eq!(val, 1234);
  }

  #[test]
  fn read_u32_wrong() {
    assert!(u32::read_from(" 1234").is_none());
    assert!(u32::read_from("hola1234").is_none());
  }

  #[test]
  fn read_literal_ok() {
    let source = "hola_ñandu12 chau";
    let (len, lit) = Literal::read_from(source).unwrap();
    assert_eq!(len, 13);
    assert_eq!(lit.as_ref(), "hola_ñandu12");

    let source = "_12hola=";
    let (len, lit) = Literal::read_from(source).unwrap();
    assert_eq!(len, 7);
    assert_eq!(lit.as_ref(), "_12hola");
  }

  #[test]
  fn read_literal_wrong() {
    let source = "12hola_ñandu chau";
    assert!(Literal::read_from(source).is_none());
  }

  #[test]
  fn read_string_ok() {
    let source = r#""Hello World""#;
    let (len, result) = String::read_from(source).unwrap();
    assert_eq!(len, 13);
    let string = result.unwrap();
    assert_eq!(string, "Hello World");
    assert_eq!(string.len(), 11);
  }
}
