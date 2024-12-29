use crate::{
  ast::{
    Block, Bool, Expression, ExpressionStatement, Ident, If, Int, LetStatement, Prefix, Program,
    ReturnStatement, Statement, StringLiteral,
  },
  branch::Branch,
  lexer::Source,
  token::{IntTokenKind, StringTokenKind, TokenKind},
};

use super::parser::{ParseError, Parser};

trait Parsable: Sized {
  fn raw_parse<'b, S: Source>(branch: &'b Branch<'b, Parser<S>>) -> Option<Self>;

  fn parse<'b, S: Source>(parent_branch: &'b Branch<'b, Parser<S>>) -> Option<Self> {
    parent_branch.scoped(Self::raw_parse)
  }
}

impl Parsable for Program {
  fn raw_parse<'b, S: Source>(branch: &'b Branch<'b, Parser<S>>) -> Option<Self> {
    let mut statements = Vec::new();
    while let Some(st) = Statement::parse(branch) {
      statements.push(st)
    }

    Some(Program::new(statements))
  }
}

impl Parsable for Statement {
  fn raw_parse<'b, S: Source>(branch: &'b Branch<'b, Parser<S>>) -> Option<Self> {
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
  fn raw_parse<'b, S: Source>(branch: &'b Branch<'b, Parser<S>>) -> Option<Self> {
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
  fn raw_parse<'b, S: Source>(branch: &'b Branch<'b, Parser<S>>) -> Option<Self> {
    let return_token = branch.take_token_kind(TokenKind::Return)?;
    let return_exp = Expression::parse(branch)?;
    branch.take_token_kind(TokenKind::Semicolon)?;

    let st = ReturnStatement::new(return_token, return_exp);
    Some(st)
  }
}

impl Parsable for ExpressionStatement {
  fn raw_parse<'b, S: Source>(branch: &'b Branch<'b, Parser<S>>) -> Option<Self> {
    let expression = Expression::parse(branch)?;
    branch.take_token_kind(TokenKind::Semicolon)?;
    let st = ExpressionStatement::new(expression);
    Some(st)
  }
}

impl Parsable for Block {
  fn raw_parse<'b, S: Source>(branch: &'b Branch<'b, Parser<S>>) -> Option<Self> {
    let token = branch.take_token_kind(TokenKind::LBrace)?;

    let mut statements = Vec::new();
    loop {
      if let Some(_) = branch.take_token_kind(TokenKind::RBrace) {
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
  fn raw_parse<'b, S: Source>(branch: &'b Branch<'b, Parser<S>>) -> Option<Self> {
    if let Some(ident) = Ident::parse(branch) {
      return Some(Expression::Ident(ident));
    }

    if let Some(int) = Int::parse(branch) {
      return Some(Expression::Int(int));
    }

    if let Some(boolean) = Bool::parse(branch) {
      return Some(Expression::Bool(boolean));
    }

    if let Some(string_literal) = StringLiteral::parse(branch) {
      return Some(Expression::StringLiteral(string_literal));
    }

    None

    // pub enum Expression {
    //   [x] Ident(Ident),
    //   [x] Int(Int),
    //   [x] Bool(Bool),
    //   [x] StringLiteral(StringLiteral),
    //   [x] If(If),
    //   [ ] Func(Func),
    //   [ ] Call(Call),
    //   [x] Prefix(Prefix),
    //   [ ] Infix(Infix),
    // }
  }
}

impl Parsable for Ident {
  fn raw_parse<'b, S: Source>(branch: &'b Branch<'b, Parser<S>>) -> Option<Self> {
    let ident_token = branch.take_token_kind(TokenKind::Ident)?;
    Some(Ident::new(ident_token))
  }
}

impl Parsable for Int {
  fn raw_parse<'b, S: Source>(branch: &'b Branch<'b, Parser<S>>) -> Option<Self> {
    let (token, value) = branch.take_token_kind_and_value(IntTokenKind)?;
    Some(Int::new(token, value))
  }
}

impl Parsable for Bool {
  fn raw_parse<'b, S: Source>(branch: &'b Branch<'b, Parser<S>>) -> Option<Self> {
    if let Some(token) = branch.take_token_kind(TokenKind::True) {
      return Some(Bool::new(token, true));
    }
    if let Some(token) = branch.take_token_kind(TokenKind::False) {
      return Some(Bool::new(token, false));
    }
    None
  }
}

impl Parsable for StringLiteral {
  fn raw_parse<'b, S: Source>(branch: &'b Branch<'b, Parser<S>>) -> Option<Self> {
    if let Some((token, value)) = branch.take_token_kind_and_value(StringTokenKind) {
      return Some(StringLiteral::new(token, value));
    }
    None
  }
}

impl Parsable for If {
  fn raw_parse<'b, S: Source>(branch: &'b Branch<'b, Parser<S>>) -> Option<Self> {
    let if_token = branch.take_token_kind(TokenKind::If)?;
    let _lparent = branch.take_token_kind(TokenKind::LParen)?;
    let condition = Expression::parse(branch)?;
    let _rparent = branch.take_token_kind(TokenKind::RParen)?;

    let consequence = Block::parse(branch)?;

    let alternative = branch.scoped(|branch| {
      let _else_token = branch.take_token_kind(TokenKind::Else)?;
      Block::parse(branch)
    });

    Some(If::new(
      if_token,
      condition.into(),
      consequence.into(),
      alternative.map(Box::new),
    ))
  }
}

static _PREFIX_TOKENS: [TokenKind; 2] = [TokenKind::Neg, TokenKind::Minus];

/// (! | -)exp
impl Parsable for Prefix {
  fn raw_parse<'b, S: Source>(branch: &'b Branch<'b, Parser<S>>) -> Option<Self> {
    let prefix_token = branch.take_token_kind_when(|kind| _PREFIX_TOKENS.contains(&kind))?;
    let expression = Expression::parse(&branch)?;
    Some(Prefix::new(prefix_token, expression))
  }
}

#[cfg(test)]
mod test {
  use super::Parsable;
  use crate::{
    ast::{
      AstNode, Bool, Expression, Ident, If, Int, LetStatement, NodeFormatter, Prefix, StringLiteral,
    },
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
  fn int_parse_test() {
    let source = " 54321 ";
    let lexer = Lexer::new(&source);

    let parser = Parser::new(lexer);
    let int = Int::parse(&parser.branch()).unwrap();
    let int_text = NodeFormatter::new(source, &int).to_string();
    assert_eq!(int_text, "54321");
  }

  #[test]
  fn false_parse_test() {
    let source = " false ";
    let lexer = Lexer::new(&source);

    let parser = Parser::new(lexer);
    let boolean = Bool::parse(&parser.branch()).unwrap();
    assert_eq!(boolean.value(), false);
  }

  #[test]
  fn true_parse_test() {
    let source = " true ";
    let lexer = Lexer::new(&source);

    let parser = Parser::new(lexer);
    let boolean = Bool::parse(&parser.branch()).unwrap();
    assert_eq!(boolean.value(), true);
  }

  #[test]
  fn string_parse_test() {
    let source = r#" "hello world"  "#;
    let lexer = Lexer::new(&source);

    let parser = Parser::new(lexer);
    let string_literal = StringLiteral::parse(&parser.branch()).unwrap();
    assert_eq!(&*string_literal.value(), "hello world");
  }

  #[test]
  fn if_test() {
    let source = r#"
  if(true) {
    let a = 5;
    return a;
  }
"#;
    let lexer = Lexer::new(&source);

    let parser = Parser::new(lexer);
    let st = If::parse(&parser.branch()).unwrap();
    let (condition, _consequence, alternative) = st.parts();

    assert!(matches!(condition, Expression::Bool(_)));
    assert!(alternative.is_none());
  }

  #[test]
  fn if_else_test() {
    let source = r#"
  if(false) {
    let a = 5;
    return a;
  } else {
    return 4;
  }
"#;
    let lexer = Lexer::new(&source);

    let parser = Parser::new(lexer);
    let st = If::parse(&parser.branch()).unwrap();
    let (condition, _consequence, alternative) = st.parts();

    assert!(matches!(condition, Expression::Bool(_)));
    assert!(alternative.is_some());
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

  #[test]
  fn prefix_negation_parse_test() {
    let source = " !false ";
    let lexer = Lexer::new(&source);

    let parser = Parser::new(lexer);
    let prefix = Prefix::parse(&parser.branch()).unwrap();
    let prefix = NodeFormatter::new(source, &prefix).to_string();
    assert_eq!(prefix, "!false");
  }

  #[test]
  fn prefix_minus_parse_test() {
    let source = " -5 ";
    let lexer = Lexer::new(&source);

    let parser = Parser::new(lexer);
    let prefix = Prefix::parse(&parser.branch()).unwrap();
    let prefix = NodeFormatter::new(source, &prefix).to_string();
    assert_eq!(prefix, "-5");
  }
}
