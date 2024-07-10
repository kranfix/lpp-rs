use enum_dispatch::enum_dispatch;

use super::Source;

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
      fn token_literal<'s>(&self, source: Source<'s>) -> &'s str {
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
