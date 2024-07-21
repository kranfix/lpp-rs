use enum_dispatch::enum_dispatch;

use crate::tokened;

use super::{
  ast_node::{AstNode, NodeDisplay},
  Expression, Ident, Token,
};

pub struct Program {
  statements: Vec<Statement>,
}
impl Program {
  pub fn new(statements: Vec<Statement>) -> Program {
    Program { statements }
  }
}
impl NodeDisplay for Program {
  fn source_fmt<'s>(&self, source: &'s str, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for st in &self.statements {
      st.source_fmt(source, f)?;
      write!(f, ";")?;
    }
    Ok(())
  }
}
impl AstNode for Program {
  fn token_literal<'s>(&self, source: &'s str) -> &'s str {
    if self.statements.is_empty() {
      ""
    } else {
      self.statements[0].token_literal(source)
    }
  }
}

#[enum_dispatch(NodeDisplay, AstNode)]
pub enum Statement {
  Let(LetStatement),
  Return(ReturnStatement),
  Expression(ExpressionStatement),
  Block(Block),
}

pub struct LetStatement {
  token: Token,
  name: Ident,
  value: Expression,
}
impl LetStatement {
  pub fn new(token: Token, name: Ident, value: Expression) -> LetStatement {
    LetStatement { token, name, value }
  }
}
tokened!(LetStatement);
impl NodeDisplay for LetStatement {
  fn source_fmt<'s>(&self, source: &'s str, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let let_st = self.token_literal(source);
    write!(f, "{let_st} ")?;
    self.name.source_fmt(source, f)?;
    write!(f, " = ")?;
    self.value.source_fmt(source, f)
  }
}

pub struct ReturnStatement {
  token: Token,
  return_exp: Expression,
}
impl ReturnStatement {
  pub fn new(token: Token, return_exp: Expression) -> ReturnStatement {
    ReturnStatement { token, return_exp }
  }
}
tokened!(ReturnStatement);
impl NodeDisplay for ReturnStatement {
  fn source_fmt<'s>(&self, source: &'s str, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let ret = self.token_literal(source);
    write!(f, "{ret} ")?;
    self.return_exp.source_fmt(source, f)
  }
}

pub struct ExpressionStatement {
  expression: Box<Expression>,
}
impl ExpressionStatement {
  pub fn new(expression: Expression) -> ExpressionStatement {
    ExpressionStatement {
      expression: Box::new(expression),
    }
  }
}
impl AstNode for ExpressionStatement {
  fn token_literal<'s>(&self, source: &'s str) -> &'s str {
    self.expression.token_literal(source)
  }
}
impl NodeDisplay for ExpressionStatement {
  fn source_fmt<'s>(&self, source: &'s str, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.expression.source_fmt(source, f)
  }
}

pub struct Block {
  token: Token,
  statements: Vec<Statement>,
}
impl Block {
  pub fn new(token: Token, statements: Vec<Statement>) -> Block {
    Block { token, statements }
  }
}
tokened!(Block);
impl NodeDisplay for Block {
  fn source_fmt<'s>(&self, source: &'s str, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut is_first = true;
    for st in &self.statements {
      if !is_first {
        write!(f, ";")?;
        is_first = false;
      }
      st.source_fmt(source, f)?;
    }
    Ok(())
  }
}
