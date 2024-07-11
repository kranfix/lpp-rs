use enum_dispatch::enum_dispatch;

use super::Token;

#[derive(Clone, Copy)]
pub struct Source<'s>(&'s str);
impl<'s> From<&'s str> for Source<'s> {
  fn from(value: &'s str) -> Self {
    Source(value)
  }
}
impl<'s> Source<'s> {
  pub fn after(&self, idx: usize) -> &'s str {
    &self.0[idx..]
  }
}
impl Token {
  pub fn literal<'s>(&self, source: Source<'s>) -> &'s str {
    let range = self.range();
    &source.0[range]
  }
}

#[enum_dispatch]
pub trait NodeDisplay {
  fn source_fmt<'s>(&self, source: Source<'s>, f: &mut std::fmt::Formatter<'_>)
    -> std::fmt::Result;
}

#[enum_dispatch]
pub trait AstNode: NodeDisplay {
  fn token_literal<'s>(&self, source: Source<'s>) -> &'s str;
}

#[macro_export]
macro_rules! tokened {
  ($node:ty) => {
    impl $crate::ast::ast_node::AstNode for $node {
      fn token_literal<'s>(&self, source: $crate::ast::ast_node::Source<'s>) -> &'s str {
        self.token.literal(source)
      }
    }
  };
}

// struct Formatter<'s, N> {
//   source: Source<'s>,
//   node: &'s N,
// }
// impl<'s, N: AstNode> std::fmt::Display for Formatter<'s, N> {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     self.node.source_fmt(self.source, f)
//   }
// }
