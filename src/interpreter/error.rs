use crate::{scanner::token::Token, statement::environment::error::EnvironmentError};

use super::values::Value;

pub struct ValueError {
	pub token: Token,
	pub message: String
}

impl ValueError {
	pub fn new(token: Token, message: &str) -> Self {
		Self {token, message: message.to_string()}
	}

	pub fn error(&self) {
		eprintln!("{}\n[line {}]", self.message, self.token.line)
	}
}

impl From<EnvironmentError> for ValueError {
	fn from(value: EnvironmentError) -> Self {
		Self {token: value.token, message: value.message}
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