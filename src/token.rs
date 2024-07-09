pub struct Token {
  kind: TokenKind,
  start: usize,
  end: usize,
}

/// Supported `TokenType`s in LPP
#[derive(Clone, Copy)]
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

impl Token {
  pub fn raw_from_literal(source: &str, start: usize, end: usize) -> Token {
    let literal = &source[start..end];
    let kind = TokenKind::from_literal(literal);
    Token { kind, start, end }
  }
}

impl TokenKind {
  pub fn from_literal(literal: &str) -> TokenKind {
    match LITERALS.binary_search_by(|(text, _)| text.cmp(&literal)) {
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
