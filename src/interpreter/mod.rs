use std::{collections::HashMap, time::UNIX_EPOCH};

use error::{check_number_operand, check_number_operands, ValueError, ValueResult};
use values::{Callable, Native, Value, ValueCell};

use crate::{parser::expr::{Expr, ExprAssignment, ExprBinary, ExprCall, ExprGet, ExprGrouping, ExprLiteral, ExprLogical, ExprSet, ExprThis, ExprUnary, ExprVariable}, scanner::token::{Token, TokenType}, statement::environment::EnvCell};

pub mod values;
pub mod error;

/// A Lox interpreter
pub struct Interpreter {
	pub environment: EnvCell,
	pub globals: EnvCell,
	pub locals: HashMap<Expr, usize>
}

impl Interpreter {
	/// Initialize a new interpreter
	pub fn new() -> Self {
		let globals = EnvCell::new();
		let mut new = Self {environment: EnvCell::with_enclosing(&globals), globals, locals: HashMap::new()};
		
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
			Ok(e) => {Some(e.value())},
			Err(e) => {e.error(); None}
		}
	}

	/// Interpret an expression
	pub fn interpret_expr(&mut self, expr: Expr) -> ValueResult<ValueCell> {
		match expr {
			Expr::Assignment(x) => {self.interpret_expr_assignment(x)}
			Expr::Binary(x) => {self.interpret_expr_binary(x)},
			Expr::Literal(x) => {self.interpret_expr_literal(x)},
			Expr::Unary(x) => {self.interpret_expr_unary(x)},
			Expr::Call(c) => {self.interpret_expr_call(c)},
			Expr::Get(g) => {self.interpret_expr_get(g)},
			Expr::Set(s) => {self.interpret_expr_set(s)},
			Expr::This(t) => {self.interpret_expr_this(t)},
			Expr::Grouping(x) => {self.interpret_expr_grouping(x)},
			Expr::Logical(x) => {self.interpret_expr_logical(x)},
			Expr::Variable(x) => {Ok(self.environment.get(x.name)?)},
		}
	}
}

impl Interpreter {
	/// Interpret a Binary expression
	pub fn interpret_expr_binary(&mut self, expr: ExprBinary) -> ValueResult<ValueCell> {
		let left = self.interpret_expr(*expr.left)?.value();
		let right = self.interpret_expr(*expr.right)?.value();
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

		Ok(ValueCell::new(v))
	}
}


impl Interpreter{
	/// Interpret a literal expression
	pub fn interpret_expr_literal(&mut self, expr: ExprLiteral) -> ValueResult<ValueCell> {
		let v = match expr {
			ExprLiteral::True => Value::Boolean(true),
			ExprLiteral::False => Value::Boolean(false),
			ExprLiteral::NUMBER(n) => Value::Double(n),
			ExprLiteral::STRING(s) => Value::String(s),
			ExprLiteral::Null => Value::Nil,
		};

		Ok(ValueCell::new(v))
	}
}

impl Interpreter {
	/// Interpret a grouping expression
	pub fn interpret_expr_grouping(&mut self, expr: ExprGrouping) -> ValueResult<ValueCell> {
		return self.interpret_expr(*expr.0);
	}
}

impl Interpreter {
	/// Interpret a unary expression
	pub fn interpret_expr_unary(&mut self, expr: ExprUnary) -> ValueResult<ValueCell> {
		let right = self.interpret_expr(*expr.right)?.value();
		let o = expr.operator;

		let v = match o.token_type {
			TokenType::MINUS=> {
				let n = check_number_operand(o, &right)?;
				Value::Double(-n)
			},
			TokenType::BANG => { Value::Boolean(!right.is_truthy()) }
			_ => Value::Nil
		};

		Ok(ValueCell::new(v))
	}
}

impl Interpreter {
	/// Interpret a call expression
	pub fn interpret_expr_call(&mut self, expr: ExprCall) -> ValueResult<ValueCell> {
		let callee = self.interpret_expr(*expr.callee)?.value();
		let mut arguments = Vec::new();

		for argument in expr.arguments {
			arguments.push(self.interpret_expr(argument)?.value());
		}

		let mut function: Box<dyn Callable> = match callee {
			Value::NativeFn(x) => Box::new(x),
			Value::Function(f) => Box::new(f),
			Value::Class(c) => Box::new(c),
			_ => return Err(ValueError::Std { token: expr.paren, message: "Can only call functions and classes".to_string() })
		};

		if arguments.len() != function.arity() {
			return Err(ValueError::Std { token: expr.paren, message: format!("Expected {} arguments but got {}.", function.arity(), arguments.len()) })
		}

		return Ok(ValueCell::new(function.call(self, arguments)?))
	}
}


impl Interpreter {
	/// Interpret a get expression
	pub fn interpret_expr_get(&mut self, expr: ExprGet) -> ValueResult<ValueCell> {
		let object = self.interpret_expr(*expr.object)?.value();

		match object {
			Value::Instance(object) => {
				return Ok(ValueCell::new(object.get(expr.name)?))
			},
			_ => Err(self.error(expr.name, "Only instances have properties"))
		}

	}
}

impl Interpreter {
	/// Interpret an assignment expression
	pub fn interpret_expr_assignment(&mut self, expr: ExprAssignment) -> ValueResult<ValueCell> {
		let value = self.interpret_expr(*expr.value.clone())?;
		let name = expr.name.clone();

		if let Some(&distance) = self.locals.get(&Expr::Assignment(expr)) {
			self.environment.assign_at(distance, &name, value.value().clone());
		} else {
			self.globals.assign(name, value.value().clone())?;
		}

		Ok(value)
	}


}

impl Interpreter {
	/// Interpret a variable expression
	pub fn interpret_expr_variable(&mut self, expr: ExprVariable) -> ValueResult<ValueCell> {
		self.look_up_variable(expr.name.clone(), Expr::Variable(expr))
	}

	pub fn look_up_variable(&mut self, name: Token, expr: Expr) -> ValueResult<ValueCell> {
		if let Some(&distance) = self.locals.get(&expr) {
			return Ok(self.environment.get_at(distance, name.lexeme.clone()))
		} else {
			return Ok(self.globals.get(name)?)
		}
	}
}

impl Interpreter {
	/// Interpret a Logical expression
	pub fn interpret_expr_logical(&mut self, expr: ExprLogical) -> ValueResult<ValueCell> {
		let left = self.interpret_expr(*expr.left)?.value();

		if expr.operator.token_type == TokenType::OR {
			if left.is_truthy() {return Ok(ValueCell::new(left))}
		} else {
			if !left.is_truthy() {return Ok(ValueCell::new(left))}
		}

		return self.interpret_expr(*expr.right);
	}
}

impl Interpreter {
	/// Interpret a set expression
	pub fn interpret_expr_set(&mut self, expr: ExprSet) -> ValueResult<ValueCell> {
		let object = self.interpret_expr(*expr.object)?;
		let mut v = object.0.borrow_mut();
		// let e = object.0.borrow().clone();
		// let  = object.0.borrow_mut();

		match &mut *v {
			Value::Instance(ref mut object) => {
				let value = self.interpret_expr(*expr.value)?;
				object.set(&expr.name, value.value());
				Ok(value)
			},
			_ => Err(self.error(expr.name, "Only instances have fields"))
		}
	}
}

impl Interpreter {
	/// Interpret a set expression
	pub fn interpret_expr_this(&mut self, expr: ExprThis) -> ValueResult<ValueCell> {
		let name = expr.keyword.clone();
		self.look_up_variable(name, Expr::This(expr))
	}
}

impl Interpreter {
	pub fn resolve_expr_depth(&mut self, expr: Expr, depth: usize) {
		self.locals.insert(expr, depth);
	}

	pub fn error(&mut self, token: Token, message: &str) -> ValueError {
		let e = ValueError::new(token, message);
		e.error();
		e
	}
}