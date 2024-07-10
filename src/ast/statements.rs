use enum_dispatch::enum_dispatch;

use crate::tokened;

use super::{
  ast_node::{AstNode, NodeDisplay},
  Expression, Source, Token,
};

pub struct Program {
  statements: Vec<Statement>,
}

impl NodeDisplay for Program {
  fn source_fmt<'s>(
    &self,
    source: Source<'s>,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    for st in &self.statements {
      st.source_fmt(source, f)?;
      write!(f, ";")?;
    }
    Ok(())
  }
}
impl AstNode for Program {
  fn token_literal<'s>(&self, source: Source<'s>) -> &'s str {
    if self.statements.is_empty() {
      ""
    } else {
      self.statements[0].token_literal(source)
    }
  }
}

#[enum_dispatch(NodeDisplay, AstNode)]
enum Statement {
  Expression(ExpressionStatement),
  Block(Block),
}

/*
class LetStatement(Statement):

    def __init__(self,
                 token: Token,
                 name: Optional[Identifier] = None,
                 value: Optional[Expression] = None) -> None:
        super().__init__(token)
        self.name = name
        self.value = value

    def __str__(self) -> str:
        return f'{self.token_literal()} {str(self.name)} = {str(self.value)};'


class ReturnStatement(Statement):

    def __init__(self,
                 token: Token,
                 return_value: Optional[Expression] = None) -> None:
        super().__init__(token)
        self.return_value = return_value

    def __str__(self) -> str:
        return f'{self.token_literal()} {str(self.return_value)};'


class ExpressionStatement(Statement):

    def __init__(self,
                 token: Token,
                 expression: Optional[Expression] = None) -> None:
        super().__init__(token)
        self.expression = expression

    def __str__(self) -> str:
        return str(self.expression)s
 */
struct ExpressionStatement {
  token: Token,
  expression: Box<Expression>,
}
impl ExpressionStatement {
  pub fn new(token: Token, expression: Expression) -> ExpressionStatement {
    ExpressionStatement {
      token,
      expression: Box::new(expression),
    }
  }
}
tokened!(ExpressionStatement);
impl NodeDisplay for ExpressionStatement {
  fn source_fmt<'s>(
    &self,
    source: Source<'s>,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    self.expression.source_fmt(source, f)
  }
}

struct Block {
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
  fn source_fmt<'s>(
    &self,
    source: Source<'s>,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
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
