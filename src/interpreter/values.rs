// use std::collections::HashMap;

use crate::statement::{environment::EnvCell, FunctionStatement};

use super::{error::{ValueError, ValueResult}, Interpreter};

/// An enum representing all possible Lox values
#[derive(PartialEq, Clone)]
pub enum Value {
	/// Lox Number
	Double(f64),
	/// Lox Null/nil
	Nil,
	/// Lox Boolean
	Boolean(bool),
	/// Lox String
	String(String),
	/// Lox Native Function/ In-built functions
	NativeFn(Native),
	/// Lox user-defined functions
	Function(LoxFunction)
}

/// A trait to be implemented for any call-able Lox value
pub trait Callable {
	/// This defines the result of a Lox Value call
	fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> ValueResult<Value>;
	/// This defines the number of arguments, taken by a Lox Callable
	fn arity(&self) -> usize;
	/// This defines the printed result of a Lox Callable Value
	fn to_string(&self) -> String;
}

/// A struct representing Lox Native/ In-built functions

#[derive(PartialEq, Clone)]
pub struct Native {
	arity: usize,
	to_string: String,
	fn_call: fn() -> Value
}

impl Native {
	/// Create a new Native function
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

/// A struct representing Lox user-defined functions
#[derive(Clone)]
pub struct LoxFunction {
	/// The associated function statement
	declaration: FunctionStatement,
	/// The closure/environment of the function
	pub closure: EnvCell,
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
	/// Initialize a user-defined function
	pub fn new(declaration: FunctionStatement, closure: EnvCell) -> Self {
		Self {declaration, closure}
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
		let mut environment = EnvCell::with_enclosing(&self.closure);

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
	/// Only the Lox boolean [`Value`] false and the Lox null/nill are falsy, every other is truthy
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