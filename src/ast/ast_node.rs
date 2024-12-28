use std::sync::{atomic::AtomicBool, Arc};

use enum_dispatch::enum_dispatch;

use super::Token;

impl Token {
  pub fn literal<'s>(&self, source: &'s str) -> &'s str {
    let range = self.range();
    &source[range]
  }
}

#[enum_dispatch]
pub trait NodeDisplay: Sized {
  fn source_fmt<'s>(&self, source: &'s str, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

#[enum_dispatch]
pub trait AstNode: NodeDisplay {
  fn token_literal<'s>(&self, source: &'s str) -> &'s str;
}

#[macro_export]
macro_rules! tokened {
  ($node:ty) => {
    impl $crate::ast::ast_node::AstNode for $node {
      fn token_literal<'s>(&self, source: &'s str) -> &'s str {
        self.token.literal(source)
      }
    }
  };
}

pub struct NodeFormatter<'n, N> {
  source: &'n str,
  node: &'n N,
}
impl<'n, N> NodeFormatter<'n, N> {
  pub fn new(source: &'n str, node: &'n N) -> NodeFormatter<'n, N> {
    NodeFormatter { source, node }
  }
}

impl<'s, 'n, N: AstNode> std::fmt::Display for NodeFormatter<'n, N> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.node.source_fmt(self.source, f)
  }
}
