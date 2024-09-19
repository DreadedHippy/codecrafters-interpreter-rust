#[derive(PartialEq, Clone)]
pub enum Value {
	Double(f64),
	Nil,
	Boolean(bool),
	String(String)
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
			Value::String(x) => &x
		};

		write!(f, "{}", as_str)
	}
}