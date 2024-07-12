mod read_from;

use crate::token::{Token, TokenKind};
use crate::types::Literal;
use read_from::{ExtractStringError, ReadFrom};

pub struct Lexer<T> {
  source: T,
  pos: usize,
  stop: Option<usize>,
}

impl<T> Lexer<T> {
  pub fn new(source: T) -> Lexer<T> {
    Lexer {
      source,
      pos: 0,
      stop: None,
    }
  }
  fn update_pos(&mut self, len: usize, kind: TokenKind) -> Token {
    let start = self.pos;
    self.pos += len;
    Token::new(kind, start, self.pos)
  }
  fn mark_blocked(&mut self, stop_len: usize) {
    self.stop = Some(self.pos + stop_len);
  }
}

impl<T: AsRef<str>> Iterator for Lexer<T> {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {
    if self.stop.is_some() {
      return None;
    }

    self.skip_whitespaces();
    let rem = self.rem();
    if rem.is_empty() {
      return None;
    }

    let token = if let Some(len) = self.read_char('=') {
      if let Some(len1) = self.read_char('=') {
        self.update_pos(len + len1, TokenKind::Eq)
      } else {
        self.update_pos(len, TokenKind::Assign)
      }
    } else if let Some(len) = self.read_char('+') {
      self.update_pos(len, TokenKind::Plus)
    } else if let Some(len) = self.read_char('\0') {
      self.update_pos(len, TokenKind::EOF)
    } else if let Some(len) = self.read_char('(') {
      self.update_pos(len, TokenKind::LParen)
    } else if let Some(len) = self.read_char(')') {
      self.update_pos(len, TokenKind::RParen)
    } else if let Some(len) = self.read_char('{') {
      self.update_pos(len, TokenKind::LBrace)
    } else if let Some(len) = self.read_char('}') {
      self.update_pos(len, TokenKind::RBrace)
    } else if let Some(len) = self.read_char(',') {
      self.update_pos(len, TokenKind::Comma)
    } else if let Some(len) = self.read_char(';') {
      self.update_pos(len, TokenKind::Semicolon)
    } else if let Some(len) = self.read_char('-') {
      self.update_pos(len, TokenKind::Minus)
    } else if let Some(len) = self.read_char('/') {
      self.update_pos(len, TokenKind::Division)
    } else if let Some(len) = self.read_char('*') {
      self.update_pos(len, TokenKind::Mul)
    } else if let Some(len) = self.read_char('<') {
      self.update_pos(len, TokenKind::LT)
    } else if let Some(len) = self.read_char('>') {
      self.update_pos(len, TokenKind::GT)
    } else if let Some(len) = self.read_char('!') {
      if let Some(len1) = self.read_char('=') {
        self.update_pos(len + len1, TokenKind::NotEq)
      } else {
        self.update_pos(len, TokenKind::Neg)
      }
    } else if let Some((len, lit)) = Literal::read_from(rem) {
      self.update_pos(len, TokenKind::from_literal(lit))
    } else if let Some((len, value)) = u32::read_from(rem) {
      self.update_pos(len, TokenKind::Int(value))
    } else if let Some((len, value)) = String::read_from(rem) {
      match value {
        Ok(value) => self.update_pos(len, TokenKind::String(value)),
        Err(err) => match err {
          ExtractStringError::Incomplete => {
            self.mark_blocked(len);
            return None;
          }
        },
      }
    } else {
      self.update_pos(0, TokenKind::Illegal)
    };
    Some(token)
  }
}

impl<T: AsRef<str>> Lexer<T> {
  fn rem<'a>(&'a self) -> &'a str {
    &self.source.as_ref()[self.pos..]
  }

  fn read_char(&self, target: char) -> Option<usize> {
    let mut chars = self.rem().chars();

    match chars.next() {
      Some(c) if c == target => Some(target.len_utf8()),
      Some(_) => None,
      None => None,
    }
  }

  fn skip_whitespaces(&mut self) {
    let mut n: usize = 0;
    let mut chars = self.rem().chars();
    while let Some(c) = chars.next() {
      if " \t\n\r".contains(c) {
        n += c.len_utf8();
      } else {
        break;
      }
    }
    self.pos += n;
  }
}
