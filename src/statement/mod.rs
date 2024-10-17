use environment::Environment;
use error::{StatementError, StatementResult};

use crate::{interpreter::{error::{ValueError, ValueResult}, values::{LoxFunction, Value}, Interpreter}, parser::{ expr::{Expr, ExprLiteral}, Parser}, scanner::token::{Token, TokenType}};

pub mod error;
pub mod environment;

// pub enum Declaration {
// 	VarDecl(VarDeclaration),
// 	StmtDecl(StatementError)
// }
#[derive(Clone)]
pub enum Statement {
	Print(PrintStatement),
	Expression(ExprStatement),
	Function(FunctionStatement),
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
pub struct PrintStatement(Expr);
#[derive(Clone)]
pub struct ExprStatement(Expr);

#[derive(Clone)]
pub struct FunctionStatement{pub name: Token, pub params: Vec<Token>, pub body: Vec<Statement>}
#[derive(Clone)]
pub struct IfStatement{condition: Expr, then_branch: Box<Statement>, else_branch: Option<Box<Statement>>}
#[derive(Clone)]
pub struct WhileStatement{condition: Expr, body: Box<Statement>}
#[derive(Clone)]
pub struct BlockStatement{statements: Vec<Statement>}
#[derive(Clone)]
pub struct VarDeclaration{name: Token, initializer: Option<Expr>}

impl Interpreter {
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
		self.environment.nest_self();

		let statements = s.statements;

		for s in statements {
			self.interpret_statement(s)?;
		}

		self.environment.set_as_parent();


		Ok(())
	}

	/// Executes a block of code, using an external scope child environment e.g. the global scope child environment\n
	/// *WARN*: THE STATE OF THE GIVEN INTERPRETER'S ENVIRONMENT WILL REMAIN UNCHANGED;
	pub fn execute_external_block(&mut self, statements: Vec<Statement>, environment: Environment) -> ValueResult<()> {

		let p = self.environment.clone();

		self.environment = environment;
		
		for s in statements {
			self.interpret_statement(s)?;
		}
		
		
		self.environment = p; // Go back to old environment


		Ok(())

	}

	pub fn interpret_if_statement(&mut self, s: IfStatement) -> ValueResult<()> {
		if self.interpret_expr(s.condition)?.is_truthy() {
			self.interpret_statement(*s.then_branch)?
		} else {
			if let Some(statement) = s.else_branch {
				self.interpret_statement(*statement)?
			}
		}

		Ok(())
	}

	pub fn interpret_while_statement(&mut self, s: WhileStatement) -> ValueResult<()> {
		while self.interpret_expr(s.condition.clone())?.is_truthy() {
			let v = self.interpret_statement(*s.body.clone());


			match v {
				Err(ValueError::Break) => break,
				Err(ValueError::Continue) => continue,
				k => k?
			}

		}

		Ok(())
	}

	pub fn interpret_break_statement(&mut self) -> ValueResult<()> {
		Err(ValueError::Break)
	}

	pub fn interpret_continue_statement(&mut self) -> ValueResult<()> {
		Err(ValueError::Continue)
	}

	pub fn interpret_function_statement(&mut self, s: FunctionStatement) -> ValueResult<()> {
		let function_name = s.name.lexeme.clone();
		let function = LoxFunction::new(s);

		self.environment.define(function_name, Value::Function(function));

		println!("Function defined");

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
				Err(e) => { e.error(); std::process::exit(65)},
			}
		}

		Ok(statements)
	}

	fn declaration(&mut self) -> StatementResult<Statement>{
		if self.match_next(vec![TokenType::FUN]) {
			return self.function("function".to_string())
		}

		if self.match_next(vec![TokenType::VAR]) {
			return self.var_declaration()
		}

		return self.statement()
	}

	fn function(&mut self, kind: String) -> StatementResult<Statement>{
		let name = self.consume(TokenType::IDENTIFIER, format!("Expect {} name.", kind))?;

		self.consume(TokenType::LEFT_PAREN, format!("Expect '(' after {} name.", kind))?;
		let mut parameters = Vec::new();

		if !self.check(TokenType::RIGHT_PAREN) {
			loop {
				if parameters.len() >= 255 {
					self.error(self.peek(), "Cant have more than 255 parameters".to_string());
				}

				parameters.push(self.consume(TokenType::IDENTIFIER, "Expect parameter name".to_string())?);

				if !self.match_next(vec![TokenType::COMMA]) {
					break
				}
			}
		}

		self.consume(TokenType::RIGHT_PAREN, "Expect ')' after parameters".to_string())?;

		self.consume(TokenType::LEFT_BRACE, format!("Expect '{{' before {} body", kind))?;

		let body = self.block_statement()?;

		let body = match body {
			Statement::Block(s) => s.statements,
			_ => return Err(StatementError::new(self.previous(), "Body not found inside after function, and '{', this ideally shouldn't happen"))
		};

		return Ok(Statement::Function(FunctionStatement {name, params: parameters, body}))

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

	fn print_statement(&mut self) -> StatementResult<Statement> {
		let value = self.expression()?;

		match &value {
			Expr::Literal(ExprLiteral::Null) => {return Err(StatementError::new(self.previous(), "Expect expression after PRINT"))},
			_ => {}
		}


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

	fn if_statement(&mut self) -> StatementResult<Statement> {
		self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'if'.".to_string())?;

		let condition = self.expression()?;

		self.consume(TokenType::RIGHT_PAREN, "Expect ')' after 'if' condition".to_string())?;

		let then_branch = Box::new(self.statement()?);
		let mut else_branch = None;

		if self.match_next(vec![TokenType::ELSE]) {
			else_branch = Some(Box::new(self.statement()?))
		}

		Ok(Statement::If(IfStatement {condition, then_branch, else_branch}))
	}

	/// Parse a while statement
	fn while_statement(&mut self) -> StatementResult<Statement> {
		self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'while'.".to_string())?;

		let condition = self.expression()?;
		self.consume(TokenType::RIGHT_PAREN, "Expect ')' after 'while' condition.".to_string())?;


		// Pre parse
		self.loop_depth += 1;

		// region:    --- Parse
		
		
		let body = Box::new(self.statement()?);
		
		
		// endregion: --- Parse
		
		// Post-parse
		self.loop_depth -= 1;

		Ok(Statement::While(WhileStatement {condition, body}))
	}

	fn for_statement(&mut self) -> StatementResult<Statement> {
		self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'for'.".to_string())?;

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
		self.consume(TokenType::SEMICOLON, "Expect ';' after loop condition".to_string())?;


		let mut increment = None;

		if !self.check(TokenType::RIGHT_PAREN) {
			increment = Some(self.expression()?)
		}

		self.consume(TokenType::RIGHT_PAREN, "Expect ')' after 'for' clauses".to_string())?;

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

	fn break_statement(&mut self) -> StatementResult<Statement> {
		if self.loop_depth == 0 {
			return Err(StatementError::new(self.previous(), "Must be inside a loop to use 'break'."))
		}

		self.consume(TokenType::SEMICOLON, "Expect ';' after 'break.".to_string())?;
		return Ok(Statement::Break())
	}

	fn continue_statement(&mut self) -> StatementResult<Statement> {
		if self.loop_depth == 0 {
			return Err(StatementError::new(self.previous(), "Must be inside a loop to use 'continue'."))
		}

		self.consume(TokenType::SEMICOLON, "Expect ';' after 'continue.".to_string())?;
		return Ok(Statement::Continue())
	}





}