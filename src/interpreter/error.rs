use crate::{scanner::token::Token, statement::environment::error::EnvironmentError};

use super::values::Value;

pub enum ValueError {
	Break,
	Std {token: Token, message: String},
}

impl ValueError {
	pub fn new(token: Token, message: &str) -> Self {
		Self::Std {token, message: message.to_string()}
	}

	pub fn error(&self) {
		match self {
			Self::Std { token, message } => eprintln!("{}\n[line {}]", message, token.line),
			Self::Break => eprintln!("'BREAK' value error detected")
		}
	}
}

impl From<EnvironmentError> for ValueError {
	fn from(value: EnvironmentError) -> Self {
		Self::Std {token: value.token, message: value.message}
	}
}

pub type ValueResult<T> = Result<T, ValueError>;

pub fn check_number_operand(operator: Token, operand: &Value) -> ValueResult<f64> {
	match operand {
		Value::Double(n) => Ok(*n),
		_ => Err(ValueError::new(operator, "Operand must be a number."))
	}
}

pub fn check_number_operands(operator: &Token, left: &Value, right: &Value) -> ValueResult<(f64, f64)> {
	match (left, right) {
		(Value::Double(l), Value::Double(r)) => Ok((*l, *r)),
		_ => Err(ValueError::new(operator.clone(), "Operands must be a numbers."))
	}
}