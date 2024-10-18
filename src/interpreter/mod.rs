use std::time::UNIX_EPOCH;

use error::{check_number_operand, check_number_operands, ValueError, ValueResult};
use values::{Callable, Native, Value};

use crate::{parser::expr::{Expr, ExprAssignment, ExprBinary, ExprCall, ExprGrouping, ExprLiteral, ExprLogical, ExprUnary}, scanner::token::TokenType, statement::environment::EnvCell};

pub mod values;
pub mod error;

/// A Lox interpreter
pub struct Interpreter {
	pub environment: EnvCell,
	pub globals: EnvCell,
}

impl Interpreter {
	/// Initialize a new interpreter
	pub fn new() -> Self {
		let globals = EnvCell::new();
		let mut new = Self {environment: EnvCell::with_enclosing(&globals), globals};
		
		fn get_curr_time() -> Value {
			let v = std::time::SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.expect("Time went backwards")
			.as_millis();

			return Value::Double(v as f64);
		}
	
		let clock = Native::new(0, get_curr_time);
		
		new.globals.define("clock".to_string(), Value::NativeFn(clock));
		new.environment = new.globals.clone();
		
		new

	}
}

impl Interpreter {
	/// Begin interpretation
	pub fn interpret(&mut self, expr: Expr) -> Option<Value>{
		let res = self.interpret_expr(expr);

		match res {
			Ok(e) => {Some(e)},
			Err(e) => {e.error(); None}
		}
	}

	/// Interpret an expression
	pub fn interpret_expr(&mut self, expr: Expr) -> ValueResult<Value> {
		match expr {
			Expr::Assignment(x) => {self.interpret_expr_assignment(x)}
			Expr::Binary(x) => {self.interpret_expr_binary(x)},
			Expr::Literal(x) => {self.interpret_expr_literal(x)},
			Expr::Unary(x) => {self.interpret_expr_unary(x)},
			Expr::Call(c) => {self.interpret_expr_call(c)},
			Expr::Grouping(x) => {self.interpret_expr_grouping(x)},
			Expr::Logical(x) => {self.interpret_expr_logical(x)},
			Expr::Variable(x) => {Ok(self.environment.get(x.name)?)},
		}
	}
}

impl Interpreter {
	/// Interpret a Binary expression
	pub fn interpret_expr_binary(&mut self, expr: ExprBinary) -> ValueResult<Value> {
		let left = self.interpret_expr(*expr.left)?;
		let right = self.interpret_expr(*expr.right)?;
		let o = expr.operator;

		let v = match o.token_type {
			TokenType::MINUS => {
				let (l, r) = check_number_operands(&o, &left, &right)?;
				Value::Double(l - r)
			},
			TokenType::PLUS => {
				match (left, right) {
					(Value::Double(l), Value::Double(r)) => Value::Double(l + r),
					// (Value::Double(l), Value::String(r)) => Value::String(l.to_string() + &r),
					// (Value::String(l), Value::Double(r)) => Value::String(l + &r.to_string()),
					(Value::String(l), Value::String(r)) => Value::String(l + &r),
					_ => return Err(ValueError::new(o, "Operands can only be numbers or strings"))
				}
			},
			TokenType::STAR => {
				let (l, r) = check_number_operands(&o, &left, &right)?;
				Value::Double(l * r)
			},
			TokenType::SLASH => {
				let (l, r) = check_number_operands(&o, &left, &right)?;
				if r == 0.0 { return Err(ValueError::new(o, "Denominator cannot be 0"))}
				Value::Double(l/r)
			},
			TokenType::GREATER => {
				let (l, r) = check_number_operands(&o, &left, &right)?;
				Value::Boolean(l > r)
			},
			TokenType::GREATER_EQUAL => {
				let (l, r) = check_number_operands(&o, &left, &right)?;
				Value::Boolean(l >= r)
			},
			TokenType::LESS => {
				let (l, r) = check_number_operands(&o, &left, &right)?;
				Value::Boolean(l < r)
			},
			TokenType::LESS_EQUAL => {
				let (l, r) = check_number_operands(&o, &left, &right)?;
				Value::Boolean(l <= r)
			},
			TokenType::BANG_EQUAL => Value::Boolean(!left.eq(&right)),
			TokenType::EQUAL_EQUAL => Value::Boolean(left.eq(&right)),
			_ => Value::Nil
		};

		Ok(v)
	}
}


impl Interpreter{
	/// Interpret a literal expression
	pub fn interpret_expr_literal(&mut self, expr: ExprLiteral) -> ValueResult<Value> {
		let v = match expr {
			ExprLiteral::True => Value::Boolean(true),
			ExprLiteral::False => Value::Boolean(false),
			ExprLiteral::NUMBER(n) => Value::Double(n),
			ExprLiteral::STRING(s) => Value::String(s),
			ExprLiteral::Null => Value::Nil,
		};

		Ok(v)
	}
}

impl Interpreter {
	/// Interpret a grouping expression
	pub fn interpret_expr_grouping(&mut self, expr: ExprGrouping) -> ValueResult<Value> {
		return self.interpret_expr(*expr.0);
	}
}

impl Interpreter {
	/// Interpret a unary expression
	pub fn interpret_expr_unary(&mut self, expr: ExprUnary) -> ValueResult<Value> {
		let right = self.interpret_expr(*expr.right)?;
		let o = expr.operator;

		let v = match o.token_type {
			TokenType::MINUS=> {
				let n = check_number_operand(o, &right)?;
				Value::Double(-n)
			},
			TokenType::BANG => { Value::Boolean(!right.is_truthy()) }
			_ => Value::Nil
		};

		Ok(v)
	}
}

impl Interpreter {
	/// Interpret a call expression
	pub fn interpret_expr_call(&mut self, expr: ExprCall) -> ValueResult<Value> {
		let callee = self.interpret_expr(*expr.callee)?;
		let mut arguments = Vec::new();

		for argument in expr.arguments {
			arguments.push(self.interpret_expr(argument)?);
		}

		let function: Box<dyn Callable> = match callee {
			Value::NativeFn(x) => Box::new(x),
			Value::Function(f) => Box::new(f),
			_ => return Err(ValueError::Std { token: expr.paren, message: "Can only call functions and classes".to_string() })
		};

		if arguments.len() != function.arity() {
			return Err(ValueError::Std { token: expr.paren, message: format!("Expected {} arguments but got {}.", function.arity(), arguments.len()) })
		}

		return Ok(function.call(self, arguments)?)
	}
}

impl Interpreter {
	/// Interpret an assignment expression
	pub fn interpret_expr_assignment(&mut self, expr: ExprAssignment) -> ValueResult<Value> {
		let value = self.interpret_expr(*expr.value)?;

		self.environment.assign(expr.name, value.clone())?;
		Ok(value)
	}
}

impl Interpreter {
	/// Interpret a Logical expression
	pub fn interpret_expr_logical(&mut self, expr: ExprLogical) -> ValueResult<Value> {
		let left = self.interpret_expr(*expr.left)?;

		if expr.operator.token_type == TokenType::OR {
			if left.is_truthy() {return Ok(left)}
		} else {
			if !left.is_truthy() {return Ok(left)}
		}

		return self.interpret_expr(*expr.right);
	}
}