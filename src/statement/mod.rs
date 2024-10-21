use std::collections::HashMap;

use environment::EnvCell;
use error::{StatementError, StatementResult};

use crate::{interpreter::{error::{ValueError, ValueResult}, values::{LoxClass, LoxFunction, Value}, Interpreter}, parser::{ expr::{Expr, ExprLiteral}, Parser}, scanner::token::{Token, TokenType}};

pub mod error;
pub mod environment;
#[derive(Clone)]
pub enum Statement {
	Print(PrintStatement),
	Expression(ExprStatement),
	Function(FunctionDecl),
	Class(ClassDecl),
	Return(ReturnStatement),
	If(IfStatement),
	While(WhileStatement),
	Break(),
	Continue(),
	Var(VarDeclaration),
	Block(BlockStatement)
}

impl Statement {
	fn new_var_statement(name: Token, initializer: Option<Expr>) -> Self{
		return Statement::Var(VarDeclaration{name, initializer})
	}
}

#[derive(Clone)]
pub struct PrintStatement(pub Expr);
#[derive(Clone)]
pub struct ExprStatement(pub Expr);

#[derive(Clone)]
pub struct FunctionDecl{pub name: Token, pub params: Vec<Token>, pub body: Vec<Statement> }
#[derive(Clone)]
pub struct ReturnStatement{ pub keyword: Token, pub value: Option<Expr> }
#[derive(Clone)]
pub struct IfStatement{ pub condition: Expr, pub then_branch: Box<Statement>, pub else_branch: Option<Box<Statement>> }
#[derive(Clone)]
pub struct WhileStatement{ pub condition: Expr, pub body: Box<Statement>}
#[derive(Clone)]
pub struct BlockStatement{ pub statements: Vec<Statement>}
#[derive(Clone)]
pub struct ClassDecl{ pub name: Token, pub methods: Vec<FunctionDecl>}
#[derive(Clone)]
pub struct VarDeclaration{ pub name: Token, pub initializer: Option<Expr> }

impl Interpreter {
	/// Interpret a list of statements sequentially. Quits the program upon error
	pub fn interpret_statements(&mut self, statements: Vec<Statement>) {
		for s in statements {
			let v = self.interpret_statement(s);

			match v {
				Ok(_) => {continue;},
				Err(e) => {e.error(); std::process::exit(70);}
			}
		}
	}
}

impl Interpreter {
	/// Interpret a given Lox Statement
	pub fn interpret_statement(&mut self, s: Statement) -> ValueResult<()> {
		match s {
			Statement::Expression(e) => {self.interpret_expr_statement(e)},
			Statement::Print(p) => {self.interpret_print_statement(p)},
			Statement::Var(v) => {self.interpret_var_statement(v)},
			Statement::Block(b) => {self.interpret_block_statement(b)},
			Statement::If(i) => {self.interpret_if_statement(i)},
			Statement::While(w) => {self.interpret_while_statement(w)},
			Statement::Break() => {self.interpret_break_statement()},
			Statement::Continue() => {self.interpret_continue_statement()},
			Statement::Function(f) => {self.interpret_function_statement(f)},
			Statement::Class(c) => {self.interpret_class_decl(c)},
			Statement::Return(r) => {self.interpret_return_statement(r)},
		}
	}

	/// Interpret an expression statement
	pub fn interpret_expr_statement(&mut self, s: ExprStatement) -> ValueResult<()> {
		self.interpret_expr(s.0)?;

		Ok(())
	}

	/// Interpret a print statement
	pub fn interpret_print_statement(&mut self, s: PrintStatement) -> ValueResult<()> {
		let v = self.interpret_expr(s.0)?;

		println!("{}", v.value());

		Ok(())
	}

	/// Interpret a var statement
	pub fn interpret_var_statement(&mut self, s: VarDeclaration) -> ValueResult<()> {
		let mut value = Value::Nil;

		if let Some(e) = s.initializer {
			value = self.interpret_expr(e)?.value();
		}

		self.environment.define(s.name.lexeme, value);

		Ok(())
	}

	/// Interpret a block statement
	pub fn interpret_block_statement(&mut self, s: BlockStatement) -> ValueResult<()> {
		let previous = self.environment.clone();
		self.environment = EnvCell::with_enclosing(&self.environment);

		let statements = s.statements;

		for s in statements {
			self.interpret_statement(s)?;
		}

		self.environment = previous;
		Ok(())
	}

	pub fn interpret_class_decl(&mut self, s: ClassDecl) -> ValueResult<()> {
		self.environment.define(s.name.lexeme.clone(), Value::Nil);

		let mut methods = HashMap::new();

		for method in s.methods {
			let name = method.name.lexeme.clone();
			let function = LoxFunction::new(method, self.environment.clone(), name == "init");
			methods.insert(name, function);
		}

		let class = Value::Class(LoxClass::new(s.name.lexeme.clone(), methods));
		self.environment.assign(s.name.clone(), class)?;

		Ok(())
	}


	/// Interpret statements sequentially, bubbling up errors to the top
	pub fn execute_statements(&mut self, statements: Vec<Statement>) -> ValueResult<()> {

		for s in statements {
			self.interpret_statement(s)?;
		}
		
		
		Ok(())

	}

	/// Interpret if statement
	pub fn interpret_if_statement(&mut self, s: IfStatement) -> ValueResult<()> {
		if self.interpret_expr(s.condition)?.value().is_truthy() {
			self.interpret_statement(*s.then_branch)?
		} else {
			if let Some(statement) = s.else_branch {
				self.interpret_statement(*statement)?
			}
		}

		Ok(())
	}

	/// Interpret a while statement
	pub fn interpret_while_statement(&mut self, s: WhileStatement) -> ValueResult<()> {
		while self.interpret_expr(s.condition.clone())?.value().is_truthy() {
			let v = self.interpret_statement(*s.body.clone());


			match v {
				Err(ValueError::Break) => break,
				Err(ValueError::Continue) => continue,
				k => k?
			}

		}

		Ok(())
	}

	/// Interpret a break statement
	pub fn interpret_break_statement(&mut self) -> ValueResult<()> {
		Err(ValueError::Break)
	}

	/// Interpret a continue statement
	pub fn interpret_continue_statement(&mut self) -> ValueResult<()> {
		Err(ValueError::Continue)
	}

	/// Interpret a function statement
	pub fn interpret_function_statement(&mut self, s: FunctionDecl) -> ValueResult<()> {
		let function_name = s.name.lexeme.clone();
		let function = LoxFunction::new(s.clone(), self.environment.clone(), false);
		self.environment.define(function_name.clone(), Value::Function(function.clone()));


		Ok(())
	}

	/// Interpret a return statement
	pub fn interpret_return_statement(&mut self, s: ReturnStatement) -> ValueResult<()> {
		let mut value = Value::Nil;
		let _ = s.keyword; // Just so we read the field, and prevent compiler warning

		if let Some(v) = s.value {
			value = self.interpret_expr(v)?.value();
		}

		Err(ValueError::Return(value))
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
	/// Parse a statement
	pub fn parse_statement(&mut self) -> StatementResult<Vec<Statement>> {
		let mut statements = Vec::new();

		while !self.is_at_end() {
			match self.declaration() {
				Ok(s) => statements.push(s),
				Err(e) => { e.error(); std::process::exit(65)},
			}
		}

		Ok(statements)
	}

	/// Parse a declaration
	fn declaration(&mut self) -> StatementResult<Statement>{
		
		if self.match_next(vec![TokenType::CLASS]) {
			return self.class_declaration()
		}

		if self.match_next(vec![TokenType::FUN]) {
			return self.function("function")
		}

		if self.match_next(vec![TokenType::VAR]) {
			return self.var_declaration()
		}

		return self.statement()
	}

	fn class_declaration(&mut self) -> StatementResult<Statement> {
		let name = self.consume(TokenType::IDENTIFIER, "Expect class name.")?;

		self.consume(TokenType::LEFT_BRACE, "Expect '{' before class body")?;

		let mut methods = Vec::new();

		while !self.check(TokenType::RIGHT_BRACE)  && !self.is_at_end() {
			let s = self.function("method")?;

			match s {
				Statement::Function(s ) => {methods.push(s);},
				_ => {
					return Err(self.error(self.previous(), "Non-function statement found in class body").into())
				}
			}
		}

		self.consume(TokenType::RIGHT_BRACE, "Expect '}' after class body")?;
		return Ok(Statement::Class(ClassDecl {name, methods}));
	}

	/// Parse a function
	fn function(&mut self, kind: &str) -> StatementResult<Statement>{
		let name = self.consume(TokenType::IDENTIFIER, &format!("Expect {} name.", kind))?;

		self.consume(TokenType::LEFT_PAREN, &format!("Expect '(' after {} name.", kind))?;
		let mut parameters = Vec::new();

		if !self.check(TokenType::RIGHT_PAREN) {
			loop {
				if parameters.len() >= 255 {
					self.error(self.peek(), "Cant have more than 255 parameters");
				}

				parameters.push(self.consume(TokenType::IDENTIFIER, "Expect parameter name")?);

				if !self.match_next(vec![TokenType::COMMA]) {
					break
				}
			}
		}

		self.consume(TokenType::RIGHT_PAREN, "Expect ')' after parameters")?;

		self.consume(TokenType::LEFT_BRACE, &format!("Expect '{{' before {} body", kind))?;

		let body = self.block_statement()?;

		let body = match body {
			Statement::Block(s) => s.statements,
			_ => return Err(StatementError::new(self.previous(), &format!("Body not found inside after {}", kind)))
		};

		return Ok(Statement::Function(FunctionDecl {name, params: parameters, body}))

	}

	/// Parse a variable declaration
	fn var_declaration(&mut self) -> StatementResult<Statement> {
		let name = self.consume(TokenType::IDENTIFIER, "Expect variable name.")?;
		
		let mut initializer = None;

		if self.match_next(vec![TokenType::EQUAL]) {
			initializer = Some(self.expression()?);
		}

		self.consume(TokenType::SEMICOLON, "Expect ';' after variable declaration.")?;

		return Ok(Statement::new_var_statement(name, initializer))
	}


	/// Parse a statement
	fn statement(&mut self) -> StatementResult<Statement> {
		if self.match_next(vec![TokenType::PRINT]) {
			return self.print_statement()
		}

		if self.match_next(vec![TokenType::RETURN]) {
			return self.return_statement()
		}

		if self.match_next(vec![TokenType::IF]) {
			return self.if_statement()
		}

		if self.match_next(vec![TokenType::WHILE]) {
			return self.while_statement()
		}

		if self.match_next(vec![TokenType::FOR]) {
			return self.for_statement()
		}

		if self.match_next(vec![TokenType::BREAK]) {
			return self.break_statement()
		}

		if self.match_next(vec![TokenType::CONTINUE]) {
			return self.continue_statement()
		}

		if self.match_next(vec![TokenType::LEFT_BRACE]) {
			return self.block_statement()
		}

		return self.expression_statement()
	}

	/// Parse a print statement
	fn print_statement(&mut self) -> StatementResult<Statement> {
		let value = self.expression()?;

		match &value {
			Expr::Literal(ExprLiteral::Null) => {return Err(StatementError::new(self.previous(), "Expect expression after PRINT"))},
			_ => {}
		}


		self.consume(TokenType::SEMICOLON, "Expect ';' after value.")?;
		Ok(Statement::Print(value.into()))
	}

	/// Parse a return statement
	fn return_statement(&mut self) -> StatementResult<Statement> {
		let keyword = self.previous();
		let mut value = None;

		if !self.check(TokenType::SEMICOLON) {
			value = Some(self.expression()?);
		}

		self.consume(TokenType::SEMICOLON, "Expect ';' after a return value.")?;

		return Ok(Statement::Return(ReturnStatement { keyword, value }));
	}

	/// Parse a block statement
	fn block_statement(&mut self) -> StatementResult<Statement> {
		let mut statements = Vec::new();

		while !self.check(TokenType::RIGHT_BRACE) && !self.is_at_end() {
			statements.push(self.declaration()?);
		}

		self.consume(TokenType::RIGHT_BRACE, "Expect '}' after block.")?;

		Ok(Statement::Block(BlockStatement{statements}))
	}

	/// Parse an expression statement
	fn expression_statement(&mut self) -> StatementResult<Statement> {
		let value = self.expression()?;
		self.consume(TokenType::SEMICOLON, "Expect ';' after value.")?;
		Ok(Statement::Expression(value.into()))
	}

	/// Parse an if statement
	fn if_statement(&mut self) -> StatementResult<Statement> {
		self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'if'.")?;

		let condition = self.expression()?;

		self.consume(TokenType::RIGHT_PAREN, "Expect ')' after 'if' condition")?;

		let then_branch = Box::new(self.statement()?);
		let mut else_branch = None;

		if self.match_next(vec![TokenType::ELSE]) {
			else_branch = Some(Box::new(self.statement()?))
		}

		Ok(Statement::If(IfStatement {condition, then_branch, else_branch}))
	}

	/// Parse a while statement
	fn while_statement(&mut self) -> StatementResult<Statement> {
		self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'while'.")?;

		let condition = self.expression()?;
		self.consume(TokenType::RIGHT_PAREN, "Expect ')' after 'while' condition.")?;


		// Pre parse
		self.loop_depth += 1;

		// region:    --- Parse
		
		
		let body = Box::new(self.statement()?);
		
		
		// endregion: --- Parse
		
		// Post-parse
		self.loop_depth -= 1;

		Ok(Statement::While(WhileStatement {condition, body}))
	}

	/// Parse a for statement
	fn for_statement(&mut self) -> StatementResult<Statement> {
		self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'for'.")?;

		let initializer = if self.match_next(vec![TokenType::SEMICOLON]) {
			None
		} else if self.match_next(vec![TokenType::VAR]) {
			Some(self.var_declaration()?)
		} else {
			Some(self.expression_statement()?)
		};

		let mut condition = None;

		if !self.check(TokenType::SEMICOLON) {
			condition = Some(self.expression()?);
		}
		self.consume(TokenType::SEMICOLON, "Expect ';' after loop condition")?;


		let mut increment = None;

		if !self.check(TokenType::RIGHT_PAREN) {
			increment = Some(self.expression()?)
		}

		self.consume(TokenType::RIGHT_PAREN, "Expect ')' after 'for' clauses")?;

		// Pre-parse
		self.loop_depth += 1;

		// Parsing the loop

		let mut body = self.statement()?;

		if let Some(increment) = increment {
			body = Statement::Block(
				BlockStatement {
					statements: vec![body, Statement::Expression(ExprStatement(increment))]
				}) 
		}

		if condition.is_none() {
			condition = Some(Expr::Literal(ExprLiteral::True))
		}

		body = Statement::While(WhileStatement { condition: condition.expect("Condition is 'None', this shouldn't happen"), body: Box::new(body) });

		if let Some(initializer) = initializer {
			body = Statement::Block(
				BlockStatement {
					statements: vec![initializer, body]
				}) 
		}

		// End of parse

		// Post-parse
		self.loop_depth -= 1;

		return Ok(body);
	}


	/// Parse a break statement
	fn break_statement(&mut self) -> StatementResult<Statement> {
		if self.loop_depth == 0 {
			return Err(StatementError::new(self.previous(), "Must be inside a loop to use 'break'."))
		}

		self.consume(TokenType::SEMICOLON, "Expect ';' after 'break.")?;
		return Ok(Statement::Break())
	}

	/// Parse a continue statement
	fn continue_statement(&mut self) -> StatementResult<Statement> {
		if self.loop_depth == 0 {
			return Err(StatementError::new(self.previous(), "Must be inside a loop to use 'continue'."))
		}

		self.consume(TokenType::SEMICOLON, "Expect ';' after 'continue.")?;
		return Ok(Statement::Continue())
	}

}