use std::ops::{Deref, Range};

use crate::core::Literal;

#[derive(Clone)]
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
}

/// Supported `TokenType`s in LPP
#[derive(Clone)]
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
  Int(u32),
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
  String(String),
  True,
}

impl TokenKind {
  pub fn from_literal<'s>(lit: Literal<'s>) -> TokenKind {
    match LITERALS.binary_search_by(|(text, _)| text.cmp(&lit.deref())) {
      Ok(idx) => LITERALS[idx].1.clone(),
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
