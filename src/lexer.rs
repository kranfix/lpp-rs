mod read_from;

use crate::token::{Token, TokenKind, TokenValue};
use crate::types::Literal;
use read_from::{ExtractStringError, ReadFrom};

pub struct Lexer<T> {
  source: T,
  pos: usize,
  stop: Option<usize>,
}

#[derive(Debug, PartialEq)]
pub enum LexerStatus {
  Open,
  Ended,
  ErrorAt(usize),
}

impl<T> Lexer<T> {
  pub fn new(source: T) -> Lexer<T> {
    Lexer {
      source,
      pos: 0,
      stop: None,
    }
  }
  pub fn status(&self) -> LexerStatus {
    match self.stop {
      Some(idx) => {
        if self.pos == idx {
          LexerStatus::Ended
        } else {
          LexerStatus::ErrorAt(idx)
        }
      }
      None => LexerStatus::Open,
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
  type Item = (Token, Option<TokenValue>);

  fn next(&mut self) -> Option<Self::Item> {
    if self.stop.is_some() {
      return None;
    }

    self.skip_whitespaces();
    let rem = self.rem();
    if rem.is_empty() {
      self.stop = Some(self.source.as_ref().len());
      return None;
    }

    let mut token_value: Option<TokenValue> = None;

    let token = if let Some(len) = self.read_char('=') {
      if let Some(len1) = self.read_char_with_offset('=', len) {
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
      if let Some(len1) = self.read_char_with_offset('=', len) {
        self.update_pos(len + len1, TokenKind::NotEq)
      } else {
        self.update_pos(len, TokenKind::Neg)
      }
    } else if let Some((len, lit)) = Literal::read_from(rem) {
      let kind = TokenKind::from_literal(lit);
      match kind {
        TokenKind::True => token_value = Some(true.into()),
        TokenKind::False => token_value = Some(false.into()),
        _ => {}
      }
      self.update_pos(len, kind)
    } else if let Some((len, value)) = u32::read_from(rem) {
      token_value = Some(value.into());
      self.update_pos(len, TokenKind::Int)
    } else if let Some((len, value)) = String::read_from(rem) {
      match value {
        Ok(value) => {
          token_value = Some(value.into());
          self.update_pos(len, TokenKind::String)
        }
        Err(err) => match err {
          ExtractStringError::Incomplete => {
            self.mark_blocked(len);
            return None;
          }
        },
      }
    } else {
      let len = rem.chars().next().unwrap().len_utf8();
      self.update_pos(len, TokenKind::Illegal)
    };
    Some((token, token_value))
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

  fn read_char_with_offset(&self, target: char, offset: usize) -> Option<usize> {
    let rem = &self.rem()[offset..];
    let mut chars = rem.chars();

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

#[cfg(test)]
mod test {
  use crate::{lexer::LexerStatus, utils::read_file};

  use super::Lexer;

  #[test]
  fn parse_file() {
    let source = read_file("fixtures/tokens/tokens.lpp").unwrap();
    let expected = read_file("fixtures/tokens/result.snapshot").unwrap();
    let mut lexer = Lexer::new(source);
    let status = lexer.status();
    assert_eq!(status, LexerStatus::Open);

    let mut result = String::new();
    let mut idx = 0;
    while let Some((token, value)) = lexer.next() {
      let line = format!("[{idx}] {token:?} -- {value:?}\n");
      result.push_str(&line);
      idx += 1;
      let status = lexer.status();
      assert_eq!(status, LexerStatus::Open);
    }
    assert_eq!(result, expected);

    let status = lexer.status();
    assert_eq!(status, LexerStatus::Ended);
  }
}
