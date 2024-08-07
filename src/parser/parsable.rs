use crate::{
  ast::{
    Block, Expression, ExpressionStatement, Ident, Int, LetStatement, Program, ReturnStatement,
    Statement,
  },
  branch::Branch,
  lexer::Source,
  token::{Token, TokenKind},
};

use super::parser::{ParseError, Parser};

type ParserBranch<'r, 'p, S> = Branch<'r, 'p, Parser<S>>;

trait Parsable: Sized {
  fn raw_parse<'p, 'b, S: Source>(branch: &'b Branch<'p, 'b, Parser<S>>) -> Option<Self>;
  fn parse<'p, 'b, S: Source>(parent_branch: &'b Branch<'p, 'b, Parser<S>>) -> Option<Self> {
    let branch = parent_branch.child();
    let val = Self::raw_parse(&branch)?;
    match branch.commit() {
      Ok(_) => Some(val),
      Err(_err) => None,
    }
  }
}

impl Parsable for Program {
  fn raw_parse<'p, 'b, S: Source>(branch: &'b Branch<'p, 'b, Parser<S>>) -> Option<Self> {
    let mut statements = Vec::new();
    loop {
      match Statement::parse(branch) {
        Some(st) => statements.push(st),
        None => break,
      }
    }

    Some(Program::new(statements))
  }
}

impl Parsable for Statement {
  fn raw_parse<'p, 'b, S: Source>(branch: &'b Branch<'p, 'b, Parser<S>>) -> Option<Self> {
    if let Some(st) = LetStatement::parse(branch) {
      return Some(Statement::Let(st));
    }
    if let Some(st) = ReturnStatement::parse(branch) {
      return Some(Statement::Return(st));
    }
    if let Some(st) = ExpressionStatement::parse(branch) {
      return Some(Statement::Expression(st));
    }
    if let Some(st) = Block::parse(branch) {
      return Some(Statement::Block(st));
    }
    None
  }
}

impl Parsable for LetStatement {
  fn raw_parse<'p, 'b, S: Source>(branch: &'b Branch<'p, 'b, Parser<S>>) -> Option<Self> {
    let let_token = branch.take_token_kind(TokenKind::Let)?;
    let name = Ident::parse(branch)?;
    branch.take_token_kind(TokenKind::Assign)?;
    let value = Expression::parse(branch)?;
    branch.take_token_kind(TokenKind::Semicolon)?;

    let st = LetStatement::new(let_token, name, value);
    Some(st)
  }
}

impl Parsable for ReturnStatement {
  fn raw_parse<'p, 'b, S: Source>(branch: &'b Branch<'p, 'b, Parser<S>>) -> Option<Self> {
    let return_token = branch.take_token_kind(TokenKind::Return)?;
    let return_exp = Expression::parse(branch)?;
    branch.take_token_kind(TokenKind::Semicolon)?;

    let st = ReturnStatement::new(return_token, return_exp);
    Some(st)
  }
}

impl Parsable for ExpressionStatement {
  fn raw_parse<'p, 'b, S: Source>(branch: &'b Branch<'p, 'b, Parser<S>>) -> Option<Self> {
    let expression = Expression::parse(branch)?;
    branch.take_token_kind(TokenKind::Semicolon)?;
    let st = ExpressionStatement::new(expression);
    Some(st)
  }
}

impl Parsable for Block {
  fn raw_parse<'p, 'b, S: Source>(branch: &'b Branch<'p, 'b, Parser<S>>) -> Option<Self> {
    let token = branch.take_token_kind(TokenKind::LBrace)?;

    let mut statements = Vec::new();
    loop {
      if let Some(_) = branch.take_token_kind(TokenKind::LBrace) {
        return Some(Block::new(token, statements));
      }
      match Statement::parse(branch) {
        Some(st) => statements.push(st),
        None => {
          if !statements.is_empty() {
            branch.add_error(ParseError::Msg("Expected a statement".into()))
          }
          return None;
        }
      }
    }
  }
}

impl Parsable for Expression {
  fn raw_parse<'p, 'b, S: Source>(branch: &'b Branch<'p, 'b, Parser<S>>) -> Option<Self> {
    let ident = Ident::parse(branch)?;
    Some(Expression::Ident(ident))
    // pub enum Expression {
    //   Ident(Ident),
    //   Int(Int),
    //   Prefix(Prefix),
    //   Infix(Infix),
    //   Bool(Bool),
    //   If(If),
    //   Func(Func),
    //   Call(Call),
    //   StringLiteral(StringLiteral),
    // }
  }
}

impl Parsable for Ident {
  fn raw_parse<'p, 'b, S: Source>(branch: &'b Branch<'p, 'b, Parser<S>>) -> Option<Self> {
    let ident_token = branch.take_token_kind(TokenKind::Ident)?;
    Some(Ident::new(ident_token))
  }
}

// impl Parsable for Int {
//   fn raw_parse<'p, 'b>(branch: &'b ParserBranch<'p, 'b>) -> Option<Self> {
//     let int_token = branch.take_token_kind(TokenKind::Int)?;
//     Int { token, value:  }
//   }
// }

#[cfg(test)]
mod test {
  use super::Parsable;
  use crate::{
    ast::{AstNode, Ident, LetStatement, NodeFormatter},
    branch::Branchable,
    lexer::Lexer,
    parser::parser::Parser,
  };

  #[test]
  fn ident_parse_test() {
    let source = " my_ident ";
    let lexer = Lexer::new(&source);

    let parser = Parser::new(lexer);
    let ident = Ident::parse(&parser.branch()).unwrap();
    let ident_name = ident.token_literal(&source);
    assert_eq!(ident_name, "my_ident")
  }

  #[test]
  fn let_statement_test() {
    let source = "  let my_var = other_var; ";
    let lexer = Lexer::new(&source);

    let parser = Parser::new(lexer);
    let st = LetStatement::parse(&parser.branch()).unwrap();
    let st_token_literal = NodeFormatter::new(source, &st).to_string();
    assert_eq!(st_token_literal, "let my_var = other_var");
  }
}
