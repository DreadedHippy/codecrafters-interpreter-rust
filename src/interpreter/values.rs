#[derive(PartialEq)]
pub enum Values {
	Double(f64),
	Nil,
	Boolean(bool),
	String(String)
}

impl Values {
	pub fn is_truthy(&self) -> bool {
		match self {
			Values::Boolean(false) | Values::Nil => false,
			_ => true
		}
	}
}

impl std::fmt::Display for Values {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let as_str = match self {
			Values::Boolean(x) => &x.to_string(),
			Values::Double(x) => &format!("{:?}", x),
			Values::Nil => "nil",
			Values::String(x) => &x
		};

		write!(f, "{}", as_str)
	}
}