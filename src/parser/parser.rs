use dupe::OptionDupedExt;

use crate::branch::{Branch, BranchData, BranchRoot};
use crate::lexer::{Lexer, Source};
use crate::token::{Token, TokenKind, TokenValue};
use crate::types::DefaultCell;
use std::cell::RefCell;
use std::iter::Iterator;

#[repr(u8)]
enum Precedence {
  Lowest = 1,
  Equals = 2,
  LessGreater = 3,
  Sum = 4,
  Product = 5,
  Prefix = 6,
  Call = 7,
}

#[derive(Debug)]
pub struct Parser<S> {
  lexer: RefCell<Lexer<S>>,
  tokens: RefCell<Vec<Token>>,
  errors: DefaultCell<Vec<ParseError>>,
  values: DefaultCell<Vec<TokenValue>>,
  branch_data: ParserBranchData,
}

impl<S> Parser<S> {
  pub fn new(lexer: Lexer<S>) -> Parser<S> {
    Parser {
      lexer: RefCell::new(lexer),
      tokens: RefCell::default(),
      errors: DefaultCell::default(),
      values: DefaultCell::default(),
      branch_data: ParserBranchData::default(),
    }
  }

  pub(crate) fn add_error(&self, error: ParseError) {
    let mut errors = self.errors.borrow_mut();
    errors.push(error);
  }
  pub fn value_at(&self, index: usize) -> Option<TokenValue> {
    let values = self.values.lazy_borrow()?;
    values.get(index).duped()
  }
}
impl<S: Source> Parser<S> {
  pub fn token_at(&self, index: usize) -> Option<Token> {
    let mut tokens = self.tokens.borrow_mut();
    let mut lexer = self.lexer.borrow_mut();

    while tokens.len() <= index {
      let Some((token, value)) = lexer.next() else {
        self.add_error(ParseError::NoMoreTokens);
        return None;
      };
      tokens.push(token);
      if let Some(value) = value {
        self.values.borrow_mut().push(value)
      }
    }
    tokens.get(index).duped()
  }
}

impl BranchData for ParserBranchData {
  fn child_data(&self) -> Self {
    ParserBranchData {
      token_pos: self.token_pos,
      value_idx: self.value_idx,
    }
  }
  fn update_from(&mut self, other: Self) {
    self.token_pos = other.token_pos;
    self.value_idx = other.value_idx;
  }
}

impl<S: Source> BranchRoot for Parser<S> {
  type BranchData = ParserBranchData;

  type CommitError = ();

  fn data(&self) -> ParserBranchData {
    ParserBranchData::default()
  }
}

#[derive(Debug, Default)]
pub struct ParserBranchData {
  pub(crate) token_pos: usize,
  pub(crate) value_idx: usize,
}

#[derive(Debug)]
pub enum ParseError {
  Msg(String),
  InvalidValueFormat(String),
  NoMoreTokens,
}
type ParserBranch<'p, S> = Branch<'p, Parser<S>>;

impl<'p, S: Source> ParserBranch<'p, S> {
  pub(crate) fn take_next_token(&mut self) -> Option<Token> {
    let token_pos = self.token_pos;
    let token = self.root().token_at(token_pos)?;
    self.token_pos += 1;
    return Some(token);
  }
  pub(crate) fn take_next_token_by_kind(&mut self, kind: TokenKind) -> Option<Token> {
    let token_pos = self.token_pos;
    let token = self.root().token_at(token_pos)?;

    if token.kind() != kind {
      return None;
    }

    self.token_pos += 1;
    return Some(token);
  }

  pub(crate) fn take_next_value(&mut self) -> Option<TokenValue> {
    let index = self.value_idx;

    let token_value = self.root().value_at(index)?;

    self.value_idx += 1;

    Some(token_value)
  }
}

/*
from lpp.ast import (
    Block,
    Boolean,
    Call,
    Expression,
    ExpressionStatement,
    Function,
    Identifier,
    If,
    Infix,
    Integer,
    LetStatement,
    Prefix,
    Program,
    ReturnStatement,
    Statement,
    StringLiteral,
)
from lpp.lexer import Lexer
from lpp.token import (
    Token,
    TokenType,
)


PrefixParseFn = Callable[[], Optional[Expression]]
InfixParseFn = Callable[[Expression], Optional[Expression]]
PrefixParseFns = Dict[TokenType, PrefixParseFn]
InfixParseFns = Dict[TokenType, InfixParseFn]


class Precedence(IntEnum):
    LOWEST = 1
    EQUALS = 2
    LESSGREATER = 3
    SUM = 4
    PRODUCT = 5
    PREFIX = 6
    CALL = 7


PRECEDENCES: Dict[TokenType, Precedence] = {
    TokenType.EQ: Precedence.EQUALS,
    TokenType.NOT_EQ: Precedence.EQUALS,
    TokenType.LT: Precedence.LESSGREATER,
    TokenType.GT: Precedence.LESSGREATER,
    TokenType.PLUS: Precedence.SUM,
    TokenType.MINUS: Precedence.SUM,
    TokenType.DIVISION: Precedence.PRODUCT,
    TokenType.MULTIPLICATION: Precedence.PRODUCT,
    TokenType.LPAREN: Precedence.CALL,
}


class Parser:

    def __init__(self, lexer: Lexer) -> None:
        self._lexer = lexer
        self._current_token: Optional[Token] = None
        self._peek_token: Optional[Token] = None
        self._errors: List[str] = []

        self._prefix_parse_fns: PrefixParseFns = self._register_prefix_fns()
        self._infix_parse_fns: InfixParseFns = self._register_infix_fns()

        self._advance_tokens()
        self._advance_tokens()

    @property
    def errors(self) -> List[str]:
        return self._errors

    def parse_program(self) -> Program:
        program: Program = Program(statements=[])

        assert self._current_token is not None
        while self._current_token.token_type != TokenType.EOF:
            statement = self._parse_statement()
            if statement is not None:
                program.statements.append(statement)

            self._advance_tokens()

        return program

    def _advance_tokens(self) -> None:
        self._current_token = self._peek_token
        self._peek_token = self._lexer.next_token()

    def _current_precedence(self) -> Precedence:
        assert self._current_token is not None
        try:
            return PRECEDENCES[self._current_token.token_type]
        except KeyError:
            return Precedence.LOWEST

    def _expected_token(self, token_type: TokenType) -> bool:
        assert self._peek_token is not None
        if self._peek_token.token_type == token_type:
            self._advance_tokens()

            return True

        self._expected_token_error(token_type)
        return False

    def _expected_token_error(self, token_type: TokenType) -> None:
        assert self._peek_token is not None
        error = f'Se esperaba que el siguiente tokne fuera {token_type} ' + \
            f'pero se obtuvo {self._peek_token.token_type}'

        self._errors.append(error)

    def _parse_block(self) -> Block:
        assert self._current_token is not None
        block_statement = Block(token=self._current_token,
                                statements=[])

        self._advance_tokens()

        while not self._current_token.token_type == TokenType.RBRACE \
                and not self._current_token.token_type == TokenType.EOF:
            statement = self._parse_statement()

            if statement:
                block_statement.statements.append(statement)

            self._advance_tokens()

        return block_statement

    def _parse_boolean(self) -> Boolean:
        assert self._current_token is not None

        return Boolean(token=self._current_token,
                       value=self._current_token.token_type == TokenType.TRUE)

    def _parse_call(self, function: Expression) -> Call:
        assert self._current_token is not None
        call = Call(self._current_token, function)
        call.arguments = self._parse_call_arguments()

        return call

    def _parse_call_arguments(self) -> Optional[List[Expression]]:
        arguments: List[Expression] = []

        assert self._peek_token is not None
        if self._peek_token.token_type == TokenType.RPAREN:
            self._advance_tokens()

            return arguments

        self._advance_tokens()
        if expression := self._parse_expression(Precedence.LOWEST):
            arguments.append(expression)

        while self._peek_token.token_type == TokenType.COMMA:
            self._advance_tokens()
            self._advance_tokens()

            if expression := self._parse_expression(Precedence.LOWEST):
                arguments.append(expression)

        if not self._expected_token(TokenType.RPAREN):
            return None

        return arguments

    def _parse_expression(self, precedence: Precedence) -> Optional[Expression]:
        assert self._current_token is not None
        try:
            prefix_parse_fn = self._prefix_parse_fns[self._current_token.token_type]
        except KeyError:
            message = f'No se encontro ninguna funcion para parsear {self._current_token.literal}'
            self._errors.append(message)

            return None

        left_expression = prefix_parse_fn()

        assert self._peek_token is not None
        while not self._peek_token.token_type == TokenType.SEMICOLON and \
                precedence < self._peek_precedence():
            try:
                infix_parse_fn = self._infix_parse_fns[self._peek_token.token_type]

                self._advance_tokens()

                assert left_expression is not None
                left_expression = infix_parse_fn(left_expression)
            except KeyError:
                return left_expression

        return left_expression

    def _parse_expression_statement(self) -> Optional[ExpressionStatement]:
        assert self._current_token is not None
        expression_statement = ExpressionStatement(token=self._current_token)

        expression_statement.expression = self._parse_expression(Precedence.LOWEST)

        assert self._peek_token is not None
        if self._peek_token.token_type == TokenType.SEMICOLON:
            self._advance_tokens()

        return expression_statement

    def _parse_grouped_expression(self) -> Optional[Expression]:
        self._advance_tokens()

        expression = self._parse_expression(Precedence.LOWEST)

        if not self._expected_token(TokenType.RPAREN):
            return None

        return expression

    def _parse_function(self) -> Optional[Function]:
        assert self._current_token is not None
        function = Function(token=self._current_token)

        if not self._expected_token(TokenType.LPAREN):
            return None

        function.parameters = self._parse_function_parameters()

        if not self._expected_token(TokenType.LBRACE):
            return None

        function.body = self._parse_block()

        return function


    def _parse_function_parameters(self) -> List[Identifier]:
        params: List[Identifier] = []

        assert self._peek_token is not None
        if self._peek_token.token_type == TokenType.RPAREN:
            self._advance_tokens()

            return params

        self._advance_tokens()

        assert self._current_token is not None
        identifier = Identifier(token=self._current_token,
                                value=self._current_token.literal)
        params.append(identifier)

        while self._peek_token.token_type == TokenType.COMMA:
            self._advance_tokens()
            self._advance_tokens()

            identifier = Identifier(token=self._current_token,
                                    value=self._current_token.literal)
            params.append(identifier)

        if not self._expected_token(TokenType.RPAREN):
            return []

        return params

    def _parse_identifier(self) -> Identifier:
        assert self._current_token is not None

        return Identifier(token=self._current_token,
                          value=self._current_token.literal)

    def _parse_if(self) -> Optional[If]:
        assert self._current_token is not None
        if_expression = If(token=self._current_token)

        if not self._expected_token(TokenType.LPAREN):
            return None

        self._advance_tokens()

        if_expression.condition = self._parse_expression(Precedence.LOWEST)

        if not self._expected_token(TokenType.RPAREN):
            return None

        if not self._expected_token(TokenType.LBRACE):
            return None

        if_expression.consequence = self._parse_block()

        assert self._peek_token is not None
        if self._peek_token.token_type == TokenType.ELSE:
            self._advance_tokens()

            if not self._expected_token(TokenType.LBRACE):
                return None

            if_expression.alternative = self._parse_block()

        return if_expression

    def _parse_infix_expression(self, left: Expression) -> Infix:
        assert self._current_token is not None
        infix = Infix(token=self._current_token,
                      operator=self._current_token.literal,
                      left=left)

        precedence = self._current_precedence()

        self._advance_tokens()

        infix.right = self._parse_expression(precedence)

        return infix

    def _parse_integer(self) -> Optional[Integer]:
        assert self._current_token is not None
        integer = Integer(token=self._current_token)

        try:
            integer.value = int(self._current_token.literal)
        except ValueError:
            message = f'No se ha podido parsear {self._current_token.literal} ' + \
                'como entero.'
            self._errors.append(message)

            return None

        return integer

    def _parse_let_statement(self) -> Optional[LetStatement]:
        assert self._current_token is not None
        let_statement = LetStatement(token=self._current_token)

        if not self._expected_token(TokenType.IDENT):
            return None

        let_statement.name = self._parse_identifier()

        if not self._expected_token(TokenType.ASSIGN):
            return None

        self._advance_tokens()

        let_statement.value = self._parse_expression(Precedence.LOWEST)

        assert self._peek_token is not None
        if self._peek_token.token_type == TokenType.SEMICOLON:
            self._advance_tokens()

        return let_statement

    def _parse_prefix_expression(self) -> Prefix:
        assert self._current_token is not None
        prefix_expression = Prefix(token=self._current_token,
                                   operator=self._current_token.literal)

        self._advance_tokens()

        prefix_expression.right = self._parse_expression(Precedence.PREFIX)

        return prefix_expression

    def _parse_string_literal(self) -> Expression:
        assert self._current_token is not None
        return StringLiteral(token=self._current_token,
                             value=self._current_token.literal)

    def _peek_precedence(self) -> Precedence:
        assert self._peek_token is not None
        try:
            return PRECEDENCES[self._peek_token.token_type]
        except KeyError:
            return Precedence.LOWEST

    def _register_infix_fns(self) -> InfixParseFns:
        return {
            TokenType.PLUS: self._parse_infix_expression,
            TokenType.MINUS: self._parse_infix_expression,
            TokenType.DIVISION: self._parse_infix_expression,
            TokenType.MULTIPLICATION: self._parse_infix_expression,
            TokenType.EQ: self._parse_infix_expression,
            TokenType.NOT_EQ: self._parse_infix_expression,
            TokenType.LT: self._parse_infix_expression,
            TokenType.GT: self._parse_infix_expression,
            TokenType.LPAREN: self._parse_call,
        }

    def _register_prefix_fns(self) -> PrefixParseFns:
        return {
            TokenType.FALSE: self._parse_boolean,
            TokenType.FUNCTION: self._parse_function,
            TokenType.IDENT: self._parse_identifier,
            TokenType.IF: self._parse_if,
            TokenType.INT: self._parse_integer,
            TokenType.LPAREN: self._parse_grouped_expression,
            TokenType.MINUS: self._parse_prefix_expression,
            TokenType.NEGATION: self._parse_prefix_expression,
            TokenType.TRUE: self._parse_boolean,
            TokenType.STRING: self._parse_string_literal
        }
*/

// Let, call, block
// let a = 5;
// {
//   let b = a + 3;
//   let inc = fn(x) { return x + 1 };
//   inc(b);
// }

/*
Program
 - Vec<Statements>


parser = Parser {
  tokens: [t1, t2]
  token_pos: 0
}

root_branch = { parent: None, parser: &parser, token_pos: 0 }

// First statemement: Try Let statement
branch_let = root_branch.branch() // { parent: Some(root_branch), parser: &parser, token_pos: 0 }
match branch_let.take_next_token() { // branch_let = { parent: Some(root_branch), parser: &parser, token_pos: 1 }
  Some(Token {kind: TokenKind.Let, ..}) => {}
  _ => return None;
}

match branch_let.take_next_token() { // branch_let = { parent: Some(root_branch), parser: &parser, token_pos: 2 }
  Some(Token {kind: TokenIdent.Ident, ..}) => {}
  _ => return None;
}

match branch_let.take_next_token() { // branch_let = { parent: Some(root_branch), parser: &parser, token_pos: 3 }
  Some(Token {kind: TokenIdent.Assing, ..}) => {}
  _ => return None;
}

let branch_let_exp = branch_let.branch(); // branch_let     = { parent: Some(root_branch), parser: &parser, token_pos: 3 }
                                          // branch_let_exp = { parent: Some(branch_let), parser: &parser, token_pos: 3 }

Expression::parse(branch_let_expression)?; // branch_let     = { parent: Some(root_branch), parser: &parser, token_pos: 3 }
                                           // branch_let_exp = { parent: Some(branch_let), parser: &parser, token_pos: 3 }
                                           //   branch_let_exp = { parent: Some(branch_let), parser: &parser, token_pos: 4 }
                                           //   ...
                                           //   branch_let_exp = { parent: Some(branch_let), parser: &parser, token_pos: 21 }
                                           // branch_let     = { parent: Some(root_branch), parser: &parser, token_pos: 21 }
match branch_let.take_next_token() { // branch_let = { parent: Some(root_branch), parser: &parser, token_pos: 22 }
  Some(Token {kind: TokenIdent.Semicolon, ..}) => {}
  _ => return None;
}
                    // root_branch = { parent: None, parser: &parser, token_pos: 0 }
branch_let.commit() // branch_let = { parent: Some(root_branch), parser: &parser, token_pos: 22 }
                    // root_branch = { parent: None, parser: &parser, token_pos: 22 }

// Second statement: Try Let statement
let branch_let = root_branch.branch();
LetStatement::parse(branch_let) // fails and doesn't commit

// Second statement: Try Call statement
let branch_call = root_branch.branch();
CallStatement::parse(branch_call) // fails and doesn't commit

// Second statement: Try Block statement
let branch_block = root_branch.branch();
BlockStatement::parse(branch_block) // success and commit

*/
