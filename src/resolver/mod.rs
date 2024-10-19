use std::collections::HashMap;

use error::{ResolverError, ResolverResult};

use crate::{interpreter::Interpreter, parser::expr::{Expr, ExprAssignment, ExprBinary, ExprCall, ExprGrouping, ExprLiteral, ExprLogical, ExprUnary, ExprVariable}, scanner::token::Token, statement::{BlockStatement, ExprStatement, FunctionStatement, IfStatement, PrintStatement, ReturnStatement, Statement, VarDeclaration, WhileStatement}};

pub mod error;
pub struct Resolver {
	pub interpreter: Interpreter,
	pub scopes: Vec<HashMap<String, bool>>,
	current_function: FunctionType,
}

#[derive(Clone, PartialEq)]
pub enum FunctionType {
	NONE,
	FUNCTION
}


impl Resolver {
	pub fn new(interpreter: Interpreter) -> Self {
		Self {interpreter, scopes: Vec::new(), current_function: FunctionType::NONE}
	}

	// region:    --- Statements
	

	pub fn resolve_statements(&mut self, statements: Vec<Statement>) -> ResolverResult<() >{
		for statement in statements {
			self.resolve_statement(statement)?;
		}

		Ok(())
	}

	fn resolve_func(&mut self, function: FunctionStatement,  function_type: FunctionType) -> ResolverResult<()> {
		let enclosing_function = self.current_function.clone();

		self.current_function = function_type;

		self.begin_scope();

		let FunctionStatement {name: _, body, params} = function;

		for param in params {
			self.declare(param.clone());
			self.define(param);
		}

		self.resolve_statements(body);

		self.end_scope();

		self.current_function = enclosing_function;

		Ok(())
	}

	

	pub fn resolve_block_statement(&mut self, s: BlockStatement) -> ResolverResult<()> {
		self.begin_scope();
		self.resolve_statements(s.statements);
		self.end_scope();

		Ok(())
	}

	pub fn resolve_expression_statement(&mut self, ExprStatement(expression): ExprStatement) -> ResolverResult<()> {
		self.resolve_expr(expression)?;

		Ok(())
	}

	pub fn resolve_func_statement(&mut self, s: FunctionStatement,) -> ResolverResult<()> {
		// Eagerly resolve name to allow recursion
		self.declare(s.name.clone());
		self.define(s.name.clone());

		self.resolve_func(s, FunctionType::FUNCTION);

		Ok(())
	}

	pub fn resolve_if_statement(&mut self, statement: IfStatement,) -> ResolverResult<()> {
		let IfStatement {condition, then_branch, else_branch} = statement;
		self.resolve_expr(condition)?;
		self.resolve_statement(*then_branch)?;

		if let Some(else_branch) = else_branch {
			self.resolve_statement(*else_branch)?
		}

		Ok(())
	}

	pub fn resolve_print_statement(&mut self, PrintStatement(expr): PrintStatement) -> ResolverResult<()> {
		self.resolve_expr(expr)?;
		
		Ok(())
	}

	pub fn resolve_return_statement(&mut self, statement: ReturnStatement) -> ResolverResult<()> {
		if self.current_function == FunctionType::NONE {
			return Err(self.error(statement.keyword, "Can't return from top-level code".to_string()));
		}

		if let Some(value) = statement.value {
			self.resolve_expr(value)?;
		}
		
		Ok(())
	}

	pub fn resolve_var_statement(&mut self, s: VarDeclaration) -> ResolverResult<()> {
		self.declare(s.name.clone())?;

		if let Some(initializer) = s.initializer {
			self.resolve_expr(initializer)?;
		}

		self.define(s.name);

		Ok(())
	}

	pub fn resolve_while_statement(&mut self, statement: WhileStatement) -> ResolverResult<()> {
		self.resolve_expr(statement.condition)?;
		self.resolve_statement(*statement.body)?;
		
		Ok(())
	}

	pub fn resolve_expr_assignment(&mut self, expr: ExprAssignment) -> ResolverResult<()> {
		let name = expr.name.clone();
		self.resolve_expr(*expr.value.clone())?;
		self.resolve_local(Expr::Assignment(expr), name);

		Ok(())
	}

	pub fn resolve_expr_binary(&mut self, expr: ExprBinary) -> ResolverResult<()> {
		self.resolve_expr(*expr.left)?;
		self.resolve_expr(*expr.right)?;

		Ok(())
	}

	pub fn resolve_expr_call(&mut self, expr: ExprCall) -> ResolverResult<()> {
		self.resolve_expr(*expr.callee)?;

		for argument in expr.arguments {
			self.resolve_expr(argument)?;
		}

		Ok(())
	}

	pub fn resolve_expr_grouping(&mut self, ExprGrouping(expr): ExprGrouping) -> ResolverResult<()> {
		self.resolve_expr(*expr)?;

		Ok(())
	}

	pub fn resolve_expr_literal(&mut self, _: ExprLiteral) -> ResolverResult<()> {
		Ok(())
	}

	pub fn resolve_expr_logical(&mut self, expr: ExprLogical) -> ResolverResult<()> {
		self.resolve_expr(*expr.left)?;
		self.resolve_expr(*expr.right)?;

		Ok(())
	}

	pub fn resolve_expr_unary(&mut self, expr: ExprUnary) -> ResolverResult<()> {
		self.resolve_expr(*expr.right)?;

		Ok(())
	}

	pub fn resolve_expr_variable(&mut self, expr: ExprVariable) -> ResolverResult<()> {
		if !self.scopes.is_empty() {
			if let Some(scope) = self.scopes.last() {
				if let Some(&v) = scope.get(&expr.name.lexeme) {
					if !v {
						return Err(self.error(expr.name, "Can't read local variable in its own initializer".to_string()))
					}
				}
				// .expect("Unwrapped a scope entry and failed, this shouldn't happen").clone();
			}
		}

		let n = expr.name.clone();
		self.resolve_local(Expr::Variable(expr), n);

		Ok(())
	}

	fn resolve_statement(&mut self, statement: Statement) -> ResolverResult<()> {
		match statement {
			Statement::Block(s) => {self.resolve_block_statement(s)?},
			Statement::Break() => {},
			Statement::Continue() => {},
			Statement::If(s) => {self.resolve_if_statement(s)?},
			Statement::Print(s) => {self.resolve_print_statement(s)?},
			Statement::Return(s) => {self.resolve_return_statement(s)?},
			Statement::While(s) => {self.resolve_while_statement(s)?},
			Statement::Function(s) => {self.resolve_func_statement(s)?},
			Statement::Expression(s) => {self.resolve_expression_statement(s)?},
			Statement::Var(s) => {self.resolve_var_statement(s)?},
		}

		Ok(())
	}

	// endregion: --- Statements

	fn resolve_expr(&mut self, expr: Expr) -> ResolverResult<()> {
		match expr {
			Expr::Assignment(expr) => {self.resolve_expr_assignment(expr)?},
			Expr::Binary(expr) => {self.resolve_expr_binary(expr)?},
			Expr::Unary(expr) => {self.resolve_expr_unary(expr)?},
			Expr::Grouping(expr) => {self.resolve_expr_grouping(expr)?},
			Expr::Literal(expr) => {self.resolve_expr_literal(expr)?},
			Expr::Call(expr) => {self.resolve_expr_call(expr)?},
			Expr::Variable(expr) => {self.resolve_expr_variable(expr)?},
			Expr::Logical(expr) => {self.resolve_expr_logical(expr)?},
		}

		Ok(())
	}


	// region:    --- Utils

	

	fn begin_scope(&mut self) {
		self.scopes.push(HashMap::new());
	}

	fn end_scope(&mut self) {
		self.scopes.pop();
	}

	fn declare(&mut self, name: Token) -> ResolverResult<()> {
		if let Some(scope) = self.scopes.last_mut() {
			if scope.contains_key(&name.lexeme) {
				return Err(self.error(name.clone(), "Already a variable with this name in this scope".to_string()))
			}

			scope.insert(name.lexeme.clone(), false);

		}

		Ok(())
	}

	fn define(&mut self, name: Token) {
		if let Some(scope) = self.scopes.last_mut() {
			scope.insert(name.lexeme, true);
		}
	}

	fn resolve_local(&mut self, expr: Expr, name: Token) {
		let n = self.scopes.len();
		for i in (0..n).rev() {
			if self.scopes[i].contains_key(&name.lexeme) {
				self.interpreter.resolve_expr_depth(expr, (n - 1) - i);
				return
			}
		}
	}

	fn error(&self, token: Token, message: String) -> ResolverError {
		let e = ResolverError::new(token, message);
		e.error();

		e
	}
	// endregion: --- Utils

}