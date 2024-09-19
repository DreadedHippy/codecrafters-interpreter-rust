use error::{check_number_operand, check_number_operands, ValueError, ValueResult};
use values::Values;

use crate::{parser::expr::{Expr, ExprBinary, ExprGrouping, ExprLiteral, ExprUnary}, scanner::token::TokenType};

pub mod values;
pub mod error;
pub trait Interpret {
	fn interpret(self) -> ValueResult<Values>;
}

impl Expr {
	pub fn evaluate_self(self) -> Option<Values>{
		let res = self.interpret();

		match res {
			Ok(e) => {Some(e)},
			Err(e) => {e.error(); None}
		}
	}
}

impl Interpret for Expr {
	fn interpret(self) -> ValueResult<Values> {
		match self {
			Expr::Binary(x) => {x.interpret()},
			Expr::Literal(x) => {x.interpret()},
			Expr::Unary(x) => {x.interpret()},
			Expr::Grouping(x) => {x.interpret()},
		}

	}
}

impl Interpret for ExprLiteral {
	fn interpret(self) -> ValueResult<Values> {
		let v = match self {
			ExprLiteral::True => Values::Boolean(true),
			ExprLiteral::False => Values::Boolean(false),
			ExprLiteral::NUMBER(n) => Values::Double(n),
			ExprLiteral::STRING(s) => Values::String(s),
			ExprLiteral::Null => Values::Nil,
		};

		Ok(v)
	}
}

impl Interpret for ExprGrouping {
	fn interpret(self) -> ValueResult<Values> {
		return self.0.interpret();
	}
}


impl Interpret for ExprUnary {
	fn interpret(self) -> ValueResult<Values> {
		let right = self.right.interpret()?;
		let o = self.operator;

		let v = match o.token_type {
			TokenType::MINUS=> {
				let n = check_number_operand(o, &right)?;
				Values::Double(-n)
			},
			TokenType::BANG => { Values::Boolean(!right.is_truthy()) }
			_ => Values::Nil
		};

		Ok(v)
	}
}

impl Interpret for ExprBinary {
	fn interpret(self) -> ValueResult<Values> {
		let left = self.left.interpret()?;
		let right = self.right.interpret()?;
		let o = self.operator;

		let v = match o.token_type {
			TokenType::MINUS => {
				let (l, r) = check_number_operands(&o, &left, &right)?;
				Values::Double(l - r)
			},
			TokenType::PLUS => {
				match (left, right) {
					(Values::Double(l), Values::Double(r)) => Values::Double(l + r),
					(Values::Double(l), Values::String(r)) => Values::String(l.to_string() + &r),
					(Values::String(l), Values::Double(r)) => Values::String(l + &r.to_string()),
					(Values::String(l), Values::String(r)) => Values::String(l + &r),
					_ => return Err(ValueError::new(o, "Operands can only be numbers or strings"))
				}
			},
			TokenType::STAR => {
				let (l, r) = check_number_operands(&o, &left, &right)?;
				Values::Double(l * r)
			},
			TokenType::SLASH => {
				let (l, r) = check_number_operands(&o, &left, &right)?;
				if r == 0.0 { return Err(ValueError::new(o, "Denominator cannot be 0"))}
				Values::Double(l/r)
			},
			TokenType::GREATER => {
				let (l, r) = check_number_operands(&o, &left, &right)?;
				Values::Boolean(l > r)
			},
			TokenType::GREATER_EQUAL => {
				let (l, r) = check_number_operands(&o, &left, &right)?;
				Values::Boolean(l >= r)
			},
			TokenType::LESS => {
				let (l, r) = check_number_operands(&o, &left, &right)?;
				Values::Boolean(l < r)
			},
			TokenType::LESS_EQUAL => {
				let (l, r) = check_number_operands(&o, &left, &right)?;
				Values::Boolean(l <= r)
			},
			TokenType::BANG_EQUAL => Values::Boolean(!left.eq(&right)),
			TokenType::EQUAL_EQUAL => Values::Boolean(left.eq(&right)),
			_ => Values::Nil
		};

		Ok(v)
	}
}
