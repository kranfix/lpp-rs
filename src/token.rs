use std::{
  ops::{Deref, Range},
  rc::Rc,
};

use dupe::Dupe;

use crate::types::Literal;

#[derive(Clone, Debug, Dupe)]
pub struct Token {
  kind: TokenKind,
  start: usize,
  end: usize,
}

impl Token {
  pub fn new(kind: TokenKind, start: usize, end: usize) -> Token {
    Token { kind, start, end }
  }

  pub fn range(&self) -> Range<usize> {
    self.start..self.end
  }

  pub fn kind(&self) -> TokenKind {
    self.kind
  }
}

/// Supported `TokenType`s in LPP
#[derive(Clone, Copy, Debug, PartialEq, Eq, Dupe)]
pub enum TokenKind {
  Assign,
  Comma,
  Division,
  Else,
  EOF,
  Eq,
  False,
  Func,
  GT,
  Ident,
  If,
  Illegal,
  Int,
  LBrace,
  Let,
  LParen,
  LT,
  Minus,
  Mul, // Multiplication
  Neg, // Negation
  NotEq,
  Plus,
  Return,
  RParen,
  RBrace,
  Semicolon,
  String,
  True,
}

impl TokenKind {
  pub fn from_literal<'s>(lit: Literal<'s>) -> TokenKind {
    match LITERALS.binary_search_by(|(text, _)| text.cmp(&lit.deref())) {
      Ok(idx) => LITERALS[idx].1,
      Err(_) => TokenKind::Ident,
    }
  }
}

static LITERALS: [(&str, TokenKind); 7] = [
  ("else", TokenKind::Else),
  ("false", TokenKind::False),
  ("fn", TokenKind::Func),
  ("if", TokenKind::If),
  ("let", TokenKind::Let),
  ("return", TokenKind::Return),
  ("true", TokenKind::True),
];

#[derive(Debug, Clone, Dupe)]
pub enum TokenValue {
  Int(u32),
  String(Rc<str>),
  //Bool(bool),
}

impl TokenValue {
  pub fn to_int(self) -> Option<u32> {
    match self {
      TokenValue::Int(value) => Some(value),
      _ => None,
    }
  }
  pub fn to_string(self) -> Option<Rc<str>> {
    match self {
      TokenValue::String(value) => Some(value),
      _ => None,
    }
  }
}

impl From<u32> for TokenValue {
  fn from(value: u32) -> Self {
    TokenValue::Int(value)
  }
}
//impl From<bool> for TokenValue {
//  fn from(value: bool) -> Self {
//    TokenValue::Bool(value)
//  }
//}
impl From<String> for TokenValue {
  fn from(value: String) -> Self {
    TokenValue::String(value.into())
  }
}
