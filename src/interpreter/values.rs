use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{scanner::token::Token, statement::{environment::{EnvCell, Environment}, FunctionDecl}};

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
	Function(LoxFunction),
	/// Lox class
	Class(LoxClass),
	/// Lox class
	Instance(LoxInstance)
}

#[derive(PartialEq, Clone)]
pub struct ValueCell(pub Rc<RefCell<Value>>);

impl ValueCell {
	pub fn new(v: Value) -> Self {
		ValueCell(Rc::new(RefCell::new(v)))
	}

	pub fn value(&self) -> Value {
		let e = self.0.borrow().clone();
		e
	} 
}

/// A trait to be implemented for any call-able Lox value
pub trait Callable {
	/// This defines the result of a Lox Value call
	fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> ValueResult<Value>;
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
	fn call(&mut self, _: &mut Interpreter, _: Vec<Value>) -> ValueResult<Value> {
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
	declaration: FunctionDecl,
	/// The closure/environment of the function
	pub closure: EnvCell,
	is_initializer: bool
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
	pub fn new(declaration: FunctionDecl, closure: EnvCell, is_initializer: bool) -> Self {
		Self {declaration, closure, is_initializer}
	}

	pub fn bind(&mut self, instance: LoxInstance) -> Self {
		let mut environment = Environment::with_enclosing(self.closure.clone());
		environment.define("this".to_string(), Value::Instance(instance));
		
		return LoxFunction::new(self.declaration.clone(), EnvCell::with_environment(environment) , self.is_initializer)
	}
}


impl Callable for LoxFunction {
	fn arity(&self) -> usize {
		self.declaration.params.len()
	}

	fn to_string(&self) -> String {
		format!("<fn {}>",self.declaration.name.lexeme)
	}


	fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> ValueResult<Value> {
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
					ValueError::Return(v) => {
						if self.is_initializer {
							Ok(self.closure.get_at(0, "this".to_string()).value())
						} else {
							Ok(v)
						}
					},
					k => {
						// Ideally this should never happen but just in case it somehow does
						k.error();
						Err(ValueError::new(self.declaration.name.clone(), "Non-return value error detected in function call", ))
					}
				}
			},
			_ => {
				if self.is_initializer {
					Ok(self.closure.get_at(0, "this".to_string()).value())
				} else {
					Ok(Value::Nil)
				}
			}
		};



		interpreter.environment = previous;
		result
	}
}

#[derive(PartialEq, Clone)]
pub struct LoxClass {
	pub name: String,
	pub methods: HashMap<String, LoxFunction>
}

impl LoxClass {
	pub fn new(name: String, methods: HashMap<String, LoxFunction>) -> Self {
		Self { name, methods }
	}

	pub fn find_method(&self, name: &str) -> Option<LoxFunction> {
		self.methods.get(name)
			.map(|m| m.clone())
	}
}

impl Callable for LoxClass {
	fn arity(&self) -> usize {
		self.find_method("init")
			.map(|m| m.arity())
			.unwrap_or(0)
	}

	fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> ValueResult<Value> {
		let instance = LoxInstance::new(self.clone());

		if let Some(initializer) = self.methods.get_mut("init") {
			return initializer.bind(instance.clone()).call(interpreter, arguments)
		}

		// if let Some(mut initializer) = self.find_method("init") {
		// 	initializer.bind(instance.clone()).call(interpreter, arguments)?;
		// }

		return Ok(Value::Instance(instance))
	}

	fn to_string(&self) -> String {
		self.name.to_string()
	}
}

#[derive(PartialEq, Clone)]
pub struct LoxInstance {
	class: LoxClass,
	fields: HashMap<String, Value>
}

impl LoxInstance {
	pub fn new(class: LoxClass) -> Self {
		Self { class, fields: HashMap::new() }
	}

	pub fn get(&self, name: Token) -> ValueResult<Value> {
		let l = name.lexeme.clone();

		match self.fields.get(&l) {
			Some(v) => return Ok(v.clone()),
			_ => {
				if let Some(mut method) = self.class.find_method(&name.lexeme) {
					let v = method.bind(self.clone());
					return Ok(Value::Function(v));
				}

				Err(ValueError::new(name, &format!("Undefined property '{}'.", l)))
			
			}
		}


	}

	pub fn set(&mut self, name: &Token, value: Value){
		let l = name.lexeme.clone();
		self.fields.insert(l, value);
	}
}

impl ToString for LoxInstance {
	fn to_string(&self) -> String {
		format!("{} instance", self.class.name)
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
			Value::Class(x) => &x.to_string(),
			Value::Instance(x) => &x.to_string(),
			Value::String(x) => &x,
		};

		write!(f, "{}", as_str)
	}
}