use crate::scanner::token::Token;

use super::values::Values;

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

pub type ValueResult<T> = Result<T, ValueError>;

pub fn check_number_operand(operator: Token, operand: &Values) -> ValueResult<f64> {
	match operand {
		Values::Double(n) => Ok(*n),
		_ => Err(ValueError::new(operator, "Operand must be a number."))
	}
}

pub fn check_number_operands(operator: &Token, left: &Values, right: &Values) -> ValueResult<(f64, f64)> {
	match (left, right) {
		(Values::Double(l), Values::Double(r)) => Ok((*l, *r)),
		_ => Err(ValueError::new(operator.clone(), "Operands must be a numbers."))
	}
}