use error::StatementResult;

use crate::{interpreter::{error::ValueResult, values::Value, Interpreter}, parser::{expr::Expr, Parser}, scanner::token::{Token, TokenType}};

pub mod error;
pub mod environment;

// pub enum Declaration {
// 	VarDecl(VarDeclaration),
// 	StmtDecl(StatementError)
// }
pub enum Statement {
	Print(PrintStatement),
	Expression(ExprStatement),
	Var(VarDeclaration),
	Block(BlockStatement)
}

impl Statement {
	fn new_var_statement(name: Token, initializer: Option<Expr>) -> Self{
		return Statement::Var(VarDeclaration{name, initializer})
	}
}

pub struct PrintStatement(Expr);
pub struct ExprStatement(Expr);

pub struct BlockStatement{statements: Vec<Statement>}
pub struct VarDeclaration{name: Token, initializer: Option<Expr>}

impl Interpreter {
	pub fn interpret_statements(&mut self, statements: Vec<Statement>) {
		for s in statements {
			let v = self.interpret_statement(s);

			match v {
				Ok(_) => {continue;},
				Err(_) => {break;}
			}
		}
	}
}

impl Interpreter {
	pub fn interpret_statement(&mut self, s: Statement) -> ValueResult<()> {
		match s {
			Statement::Expression(e) => {self.interpret_expr_statement(e)},
			Statement::Print(p) => {self.interpret_print_statement(p)},
			Statement::Var(v) => {self.interpret_var_statement(v)},
			Statement::Block(b) => {self.interpret_block_statement(b)}
		}
	}

	pub fn interpret_expr_statement(&mut self, s: ExprStatement) -> ValueResult<()> {
		self.interpret_expr(s.0)?;

		Ok(())
	}

	pub fn interpret_print_statement(&mut self, s: PrintStatement) -> ValueResult<()> {
		let v = self.interpret_expr(s.0)?;

		println!("{}", v);

		Ok(())
	}

	pub fn interpret_var_statement(&mut self, s: VarDeclaration) -> ValueResult<()> {
		let mut value = Value::Nil;

		if let Some(e) = s.initializer {
			value = self.interpret_expr(e)?;
		}

		self.environment.define(s.name.lexeme, value);

		Ok(())
	}

	pub fn interpret_block_statement(&mut self, s: BlockStatement) -> ValueResult<()> {
		self.environment = self.environment.create_inner();

		let statements = s.statements;

		for s in statements {
			self.interpret_statement(s)?;
		}

		self.environment = self.environment.get_enclosing();


		Ok(())
	}
}

impl From<Expr> for PrintStatement {
	fn from(value: Expr) -> Self {
		PrintStatement(value)
	}
}

impl From<Expr> for ExprStatement {
	fn from(value: Expr) -> Self {
		ExprStatement(value)
	}
}


impl Parser {
	pub fn parse_statement(&mut self) -> StatementResult<Vec<Statement>> {
		let mut statements = Vec::new();

		while !self.is_at_end() {
			match self.declaration() {
				Ok(s) => statements.push(s),
				Err(_) => {std::process::exit(70)},
			}
		}

		Ok(statements)
	}

	fn declaration(&mut self) -> StatementResult<Statement>{
		if self.match_next(vec![TokenType::VAR]) {
			return self.var_declaration()
		}
		return self.statement()
	}

	fn var_declaration(&mut self) -> StatementResult<Statement> {
		let name = self.consume(TokenType::IDENTIFIER, "Expect variable name.".to_string())?;
		
		let mut initializer = None;

		if self.match_next(vec![TokenType::EQUAL]) {
			initializer = Some(self.expression()?);
		}

		self.consume(TokenType::SEMICOLON, "Expect ';' after variable declaration.".to_string())?;

		return Ok(Statement::new_var_statement(name, initializer))
	}


	fn statement(&mut self) -> StatementResult<Statement> {
		if self.match_next(vec![TokenType::PRINT]) {
			return self.print_statement()
		}

		if self.match_next(vec![TokenType::LEFT_BRACE]) {
			return self.block_statement()
		}

		return self.expression_statement()
	}

	fn print_statement(&mut self) -> StatementResult<Statement> {
		let value = self.expression()?;
		self.consume(TokenType::SEMICOLON, "Expect ';' after value.".to_string())?;
		Ok(Statement::Print(value.into()))
	}

	fn block_statement(&mut self) -> StatementResult<Statement> {
		let mut statements = Vec::new();

		while !self.check(TokenType::RIGHT_BRACE) && !self.is_at_end() {
			statements.push(self.declaration()?);
		}

		self.consume(TokenType::RIGHT_BRACE, "Expect '}' after block.".to_string())?;

		Ok(Statement::Block(BlockStatement{statements}))
	}

	fn expression_statement(&mut self) -> StatementResult<Statement> {
		let value = self.expression()?;
		self.consume(TokenType::SEMICOLON, "Expect ';' after value.".to_string())?;
		Ok(Statement::Expression(value.into()))
	}


}

// impl Interpret for Statement{
// 	fn interpret(self) -> ValueResult<Value> {
// 		match self {
// 			Self::Expression(e) => e.0.interpret(),
// 			Self::Print(e) => {
// 				let v = e.0.interpret();
// 				if let Ok(v) = &v {
// 					println!("{}", v);
// 				}
// 				v
// 			},
// 			Self::Var(v) => {
// 				v.initializer.interpret()
// 			}
// 		}
// 	}
// }