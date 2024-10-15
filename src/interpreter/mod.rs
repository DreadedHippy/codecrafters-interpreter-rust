use error::{check_number_operand, check_number_operands, ValueError, ValueResult};
use values::Value;

use crate::{parser::expr::{Expr, ExprAssignment, ExprBinary, ExprGrouping, ExprLiteral, ExprLogical, ExprUnary}, scanner::token::TokenType, statement::environment::Environment};

pub mod values;
pub mod error;

pub struct Interpreter {
	pub environment: Environment
}

impl Interpreter {
	pub fn new() -> Self {
		Self {environment: Environment::default()}
	}
}
impl Interpreter {
	pub fn interpret(&mut self, expr: Expr) -> Option<Value>{
		let res = self.interpret_expr(expr);

		match res {
			Ok(e) => {Some(e)},
			Err(e) => {e.error(); None}
		}
	}

	pub fn interpret_expr(&mut self, expr: Expr) -> ValueResult<Value> {
		match expr {
			Expr::Assignment(x) => {self.interpret_expr_assignment(x)}
			Expr::Binary(x) => {self.interpret_expr_binary(x)},
			Expr::Literal(x) => {self.interpret_expr_literal(x)},
			Expr::Unary(x) => {self.interpret_expr_unary(x)},
			Expr::Grouping(x) => {self.interpret_expr_grouping(x)},
			Expr::Logical(x) => {self.interpret_expr_logical(x)},
			Expr::Variable(x) => {Ok(self.environment.get(x.name)?)},
		}
	}
}

/// for ExprBinary
impl Interpreter {
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
	pub fn interpret_expr_grouping(&mut self, expr: ExprGrouping) -> ValueResult<Value> {
		return self.interpret_expr(*expr.0);
	}
}


impl Interpreter {
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
	pub fn interpret_expr_assignment(&mut self, expr: ExprAssignment) -> ValueResult<Value> {
		let value = self.interpret_expr(*expr.value)?;

		self.environment.assign(expr.name, value.clone())?;
		Ok(value)
	}
}

impl Interpreter {
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

pub trait Interpret {
	fn interpret(self) -> ValueResult<Value>;
}