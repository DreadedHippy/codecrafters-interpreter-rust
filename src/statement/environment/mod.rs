use std::{cell::RefCell, collections::HashMap, rc::Rc};

use error::{EnvironmentError, EnvironmentResult};

use crate::{interpreter::values::Value, scanner::token::Token};

pub mod error;


/// A struct representing an interpreter's environment
#[derive(Default, Clone)]
pub struct Environment {
	pub values: HashMap<String, Value>,
	pub enclosing: Option<EnvCell>
}

/// A struct holding references to environments, this is to allow for self-references
#[derive(Clone)]
pub struct EnvCell(Rc<RefCell<Environment>>);

impl EnvCell {
	pub fn new() -> Self {
		let e = Environment::default();
		Self(Rc::new(RefCell::new(e)))
	}

	pub fn define(&mut self, name: String, value: Value) {
		self.0.borrow_mut().define(name, value);
	}

	pub fn assign(&mut self, name: Token, value: Value) -> EnvironmentResult<()> {
		self.0.borrow_mut().assign(name, value)
	}

	pub fn get(&self, name: Token) -> EnvironmentResult<Value> {
		return self.0.borrow().get(name)
	}

	pub fn with_enclosing(enclosing: &EnvCell) -> Self {
		let enclosing_environment = enclosing.clone();

		let new_environment = Environment::with_enclosing(enclosing_environment);

		let new_env_cell = EnvCell(Rc::new(RefCell::new(new_environment)));

		return new_env_cell;

	}
}


impl Environment {
	/// Takes a given environment, mutates it changing it into its own child
	pub fn with_enclosing(enclosing: EnvCell) -> Self {
		Self {
			values: HashMap::new(),
			enclosing: Some(enclosing)
		}
	}

	// pub fn nest_self(&mut self) {
	// 	let mut d = Environment::default();
	// 	std::mem::swap(&mut d, self);
		
	// 	*self = Environment {
	// 		values: HashMap::new(),
	// 		enclosing: Some(Rc::new(RefCell::new(d)))
	// 	};
	// }

	// /// Clones a given environment, nesting it as a parent of a new environment, and returns the nested environment
	// pub fn create_cloned_inner(&mut self) -> Self {
	// 	let c = self.clone();
	// 	Environment {
	// 		values: HashMap::new(),
	// 		enclosing: Some(Rc::new(RefCell::new(c)))
	// 	}
	// }

	/// Mutates a given environment, setting it to it's own parent, discarding the child
	// pub fn set_as_own_parent(&mut self){
	// 	let mut d = Environment::default();
	// 	std::mem::swap(&mut d, self);
		
	// 	*self = d.enclosing.unwrap().take();
	// }

	/// Defines/Overwrites values for a new entry
	pub fn define(&mut self, name: String, value: Value) {
		self.values.insert(name, value);
	}

	/// Get's the value for a given entry
	pub fn get(&self, name: Token) -> EnvironmentResult<Value> {
		// Check current scope
		if let Some(v) = self.values.get(&name.lexeme) {
			return Ok(v.clone())
		}
		
		// Check enclosing scope
		if let Some(EnvCell(s)) = &self.enclosing {
			return s.borrow().get(name)
		}
		
		let l = name.lexeme.clone();
		Err(EnvironmentError::new(name, &format!("Undefined variable '{}'.", l)))
	}

	/// Overwrites value for a given entry. Panics if entry is not found
	pub fn assign(&mut self, name: Token, value: Value) -> EnvironmentResult<()> {
		if let Some(v) = self.values.get_mut(&name.lexeme) {
			*v = value;
			return Ok(())
		}

		if let Some(EnvCell(s)) = &mut self.enclosing {
			return s.borrow_mut().assign(name, value)
		}

		let l = name.lexeme.clone();
		Err(EnvironmentError::new(name, &format!("Undefined variable '{}'.", l)))
	}
}
