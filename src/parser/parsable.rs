use crate::token::{Token, TokenKind};

use crate::ast::Ident;

trait Parsable: Sized {
  fn parse<'a, T>(tokens: &'a mut T) -> Option<Self>
  where
    T: Iterator<Item = &'a Token>;
}

impl Parsable for Ident {
  fn parse<'a, T>(tokens: &'a mut T) -> Option<Self>
  where
    T: Iterator<Item = &'a Token>,
  {
    let token = tokens.next()?;
    match token.kind() {
      TokenKind::Ident => Some(Ident::new(token.clone())),
      _ => None,
    }
  }
}
struct ParserBranch {}
