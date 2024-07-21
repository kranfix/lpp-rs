use enum_dispatch::enum_dispatch;

use crate::tokened;

use super::{
  ast_node::{AstNode, NodeDisplay, Source},
  statements::Block,
  Token,
};

#[enum_dispatch(NodeDisplay, AstNode)]
pub enum Expression {
  Ident(Ident),
  Int(Int),
  Prefix(Prefix),
  Infix(Infix),
  Bool(Bool),
  If(If),
  Func(Func),
  Call(Call),
  StringLiteral(StringLiteral),
}

pub struct Ident {
  token: Token,
}
impl Ident {
  pub fn new(token: Token) -> Ident {
    Ident { token }
  }
}
tokened!(Ident);
impl NodeDisplay for Ident {
  fn source_fmt<'s>(
    &self,
    source: Source<'s>,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    let literal = self.token.literal(source);
    write!(f, "{literal}")
  }
}

pub struct Int {
  pub(crate) token: Token,
  pub(crate) value: u32,
}
tokened!(Int);
impl NodeDisplay for Int {
  fn source_fmt<'s>(
    &self,
    _source: Source<'s>,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    let val = self.value;
    write!(f, "{val}")
  }
}

pub struct Prefix {
  token: Token,
  operator: String,
  rhs: Option<Box<Expression>>,
}
impl Prefix {
  pub fn new(token: Token, operator: String, rhs: Option<Box<Expression>>) -> Prefix {
    Prefix {
      token,
      operator,
      rhs,
    }
  }
}
tokened!(Prefix);
impl NodeDisplay for Prefix {
  fn source_fmt<'s>(
    &self,
    source: Source<'s>,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "{}", self.operator)?;
    if let Some(exp) = &self.rhs {
      exp.source_fmt(source, f)?;
    }
    Ok(())
  }
}

pub struct Infix {
  token: Token,
  lhs: Box<Expression>,
  operator: String,
  rhs: Box<Expression>,
}
impl Infix {
  pub fn new(token: Token, lhs: Expression, operator: String, rhs: Expression) -> Infix {
    Infix {
      token,
      lhs: Box::new(lhs),
      operator,
      rhs: Box::new(rhs),
    }
  }
}
tokened!(Infix);
impl NodeDisplay for Infix {
  fn source_fmt<'s>(
    &self,
    source: Source<'s>,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    self.lhs.source_fmt(source, f)?;
    write!(f, " {} ", self.operator)?;
    self.rhs.source_fmt(source, f)?;
    Ok(())
  }
}

pub struct Bool {
  token: Token,
  value: bool,
}
impl Bool {
  pub fn new(token: Token, value: bool) -> Bool {
    Bool { token, value }
  }
}
tokened!(Bool);
impl NodeDisplay for Bool {
  fn source_fmt<'s>(
    &self,
    source: Source<'s>,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    let literal = self.token.literal(source);
    write!(f, "{literal}")
  }
}

pub struct If {
  token: Token,
  condition: Box<Expression>,
  consequence: Box<Expression>,
  alternative: Option<Box<Expression>>,
}
impl If {
  pub fn new(
    token: Token,
    condition: Box<Expression>,
    consequence: Box<Expression>,
    alternative: Option<Box<Expression>>,
  ) -> If {
    If {
      token,
      condition,
      consequence,
      alternative,
    }
  }
}
tokened!(If);
impl NodeDisplay for If {
  fn source_fmt<'s>(
    &self,
    source: Source<'s>,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "if(")?;
    self.condition.source_fmt(source, f)?;
    write!(f, ") {{")?;
    self.consequence.source_fmt(source, f)?;
    write!(f, "}}")?;
    if let Some(alternative) = &self.alternative {
      write!(f, " else {{")?;
      alternative.source_fmt(source, f)?;
      write!(f, "}}")?;
    }
    Ok(())
  }
}

pub struct Func {
  token: Token,
  params: Vec<Ident>,
  body: Option<Block>,
}
impl Func {
  pub fn new(token: Token, params: Vec<Ident>, body: Option<Block>) -> Func {
    Func {
      token,
      params,
      body,
    }
  }
}
tokened!(Func);
impl NodeDisplay for Func {
  fn source_fmt<'s>(
    &self,
    source: Source<'s>,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    let literal = self.token.literal(source);
    write!(f, "{literal}(")?;
    let mut is_first = true;
    for param in &self.params {
      if !is_first {
        write!(f, ", ")?;
        is_first = false;
      }
      param.source_fmt(source, f)?;
    }
    write!(f, ") {{")?;
    if let Some(body) = &self.body {
      todo!()
    }
    write!(f, ") }}")
  }
}

pub struct Call {
  token: Token,
  func: Box<Expression>, // TODO: try to avoid Box
  args: Option<Vec<Expression>>,
}

impl Call {
  pub fn new(token: Token, func: Expression, args: Option<Vec<Expression>>) -> Call {
    Call {
      token,
      func: Box::new(func),
      args,
    }
  }
}
tokened!(Call);
impl NodeDisplay for Call {
  fn source_fmt<'s>(
    &self,
    source: Source<'s>,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    self.func.source_fmt(source, f)?;
    write!(f, "(")?;
    if let Some(args) = &self.args {
      let mut is_first = true;
      for arg in args {
        if !is_first {
          write!(f, ", ")?;
          is_first = false;
        }
        arg.source_fmt(source, f)?;
      }
    }
    write!(f, ")")
  }
}

pub struct StringLiteral {
  token: Token,
  value: String,
}
impl StringLiteral {
  pub fn new(token: Token, value: String) -> StringLiteral {
    StringLiteral { token, value }
  }
}
tokened!(StringLiteral);
impl NodeDisplay for StringLiteral {
  fn source_fmt<'s>(
    &self,
    _source: Source<'s>,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "{}", self.value)
  }
}
