use std::{cell::RefCell, collections::HashMap, rc::Rc};

use error::{EnvironmentError, EnvironmentResult};

use crate::{interpreter::values::Value, scanner::token::Token};

pub mod error;

#[derive(Default)]
pub struct Environment {
	values: HashMap<String, Value>,
	enclosing: Option<Rc<RefCell<Environment>>>
}

impl Environment {

	pub fn create_inner(&mut self) -> Self {
		let mut d = Environment::default();
		std::mem::swap(&mut d, self);
		
		Environment {
			values: HashMap::new(),
			enclosing: Some(Rc::new(RefCell::new(d)))
		}
	}

	pub fn get_enclosing(&mut self) -> Self{
		let mut d = Environment::default();
		std::mem::swap(&mut d, self);

		d = d.enclosing.unwrap().take();

		d
	}

	pub fn define(&mut self, name: String, value: Value) {
		self.values.insert(name, value);
	}

	pub fn get(&self, name: Token) -> EnvironmentResult<Value> {
		// Check current scope
		if let Some(v) = self.values.get(&name.lexeme) {
			return Ok(v.clone())
		}
		
		// Check enclosing scope
		if let Some(s) = &self.enclosing {
			return s.borrow().get(name)
		}
		
		let l = name.lexeme.clone();
		Err(EnvironmentError::new(name, &format!("Undefined variable '{}'.", l)))
	}

	pub fn assign(&mut self, name: Token, value: Value) -> EnvironmentResult<()> {
		if let Some(v) = self.values.get_mut(&name.lexeme) {
			*v = value;
			return Ok(())
		}

		if let Some(s) = &self.enclosing {
			return s.borrow_mut().assign(name, value)
		}

		let l = name.lexeme.clone();
		Err(EnvironmentError::new(name, &format!("Undefined variable '{}'.", l)))
	}
}
