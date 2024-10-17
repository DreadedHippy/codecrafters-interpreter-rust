use std::collections::HashMap;

use error::{EnvironmentError, EnvironmentResult};

use crate::{interpreter::values::Value, scanner::token::Token};

pub mod error;


/// A struct representing an interpreter's environment
#[derive(Default, Clone)]
pub struct Environment {
	pub values: HashMap<String, Value>,
	pub enclosing: Option<Box<Environment>>
}


impl Environment {
	/// Takes a given environment, mutates it changing it into its own child
	pub fn with_enclosing(enclosing: Self) -> Self {
		Self {
			values: HashMap::new(),
			enclosing: Some(Box::new(enclosing))
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
		if let Some(s) = &self.enclosing {
			return s.get(name)
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

		if let Some(s) = &mut self.enclosing {
			return s.assign(name, value)
		}

		let l = name.lexeme.clone();
		Err(EnvironmentError::new(name, &format!("Undefined variable '{}'.", l)))
	}
}
