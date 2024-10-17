use crate::statement::FunctionStatement;

use super::{error::ValueResult, Interpreter};

#[derive(PartialEq, Clone)]
pub enum Value {
	Double(f64),
	Nil,
	Boolean(bool),
	String(String),
	NativeFn(Native),
	Function(LoxFunction)
}

pub trait Callable {
	fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> ValueResult<Value>;
	fn arity(&self) -> usize;
	fn to_string(&self) -> String;
}

#[derive(PartialEq, Clone)]
pub struct Native {
	arity: usize,
	to_string: String,
	fn_call: fn() -> Value
}

impl Native {
	
	pub fn new(arity: usize, fn_call: fn() -> Value) -> Self{
		Self {
			arity,
			fn_call,
			to_string: "<native fn>".to_string(),
		}
	}
}

impl Callable for Native {

	fn call(&self, _: &mut Interpreter, _: Vec<Value>) -> ValueResult<Value> {
		Ok((self.fn_call)())
	}

	fn arity(&self) -> usize {
		self.arity
	}

	fn to_string(&self) -> String {
		self.to_string.clone()
	}
}

#[derive(Clone)]
pub struct LoxFunction {
	declaration: FunctionStatement
}

impl PartialEq for LoxFunction {
	/// Lox Functions are never equal
	fn eq(&self, _: &Self) -> bool {
		false
	}

	/// Lox functions are never equal
	fn ne(&self, _: &Self) -> bool {
		true
	}
}

impl LoxFunction {
	pub fn new(declaration: FunctionStatement) -> Self {
		Self {declaration}
	}
}

impl Callable for LoxFunction {
	fn arity(&self) -> usize {
		self.declaration.params.len()
	}

	fn to_string(&self) -> String {
		format!("<fn {}>",self.declaration.name.lexeme)
	}

	fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> ValueResult<Value> {
		let mut environment = interpreter.globals.create_cloned_inner();

		for i in 0..self.declaration.params.len() {
			environment.define(self.declaration.params[i].lexeme.clone(), arguments[i].clone());
		}

		let statements = self.declaration.body.clone();


		interpreter.execute_external_block(statements, environment)?;
		return Ok(Value::Nil);
	}
}

impl Value {
	pub fn is_truthy(&self) -> bool {
		match self {
			Value::Boolean(false) | Value::Nil => false,
			_ => true
		}
	}
}

impl std::fmt::Display for Value {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let as_str = match self {
			Value::Boolean(x) => &x.to_string(),
			Value::Double(x) => &format!("{}", x),
			Value::Nil => "nil",
			Value::NativeFn(x) => &format!("{}", x.to_string()),
			Value::Function(x) => &format!("{}", x.to_string()),
			Value::String(x) => &x
		};

		write!(f, "{}", as_str)
	}
}