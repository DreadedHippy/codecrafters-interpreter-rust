// use std::collections::HashMap;

use crate::statement::{environment::Environment, FunctionStatement};

use super::{error::{ValueError, ValueResult}, Interpreter};

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
	declaration: FunctionStatement,
	// pub closure: Environment,
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


	/// Call procedure for a LoxFunction
	fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> ValueResult<Value> {
		let mut environment = Environment::with_enclosing(interpreter.globals.clone());

		self.declaration.params.iter().zip(arguments.iter())
			.map(|(param, arg)| {
				(param.lexeme.clone(), arg.clone())
			})
			.for_each(|(k, v)| {
				environment.define(k, v);
			})
		;

		let previous = interpreter.environment.clone();
		interpreter.environment = environment;

		let result = match interpreter.execute_statements(self.declaration.body.clone()) {
				Err(value) => {
					match value {
						ValueError::Return(v) => Ok(v),
						k => {
							// Ideally this should never happen but just in case it somehow does
							k.error();
							Err(ValueError::new(self.declaration.name.clone(), "Non-return value error detected in function call", ))
						}
					}
				},
				_ => Ok(Value::Nil)
		};

		interpreter.environment = previous;
		result
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