use crate::scanner::token::Token;

#[derive(Clone)]
pub enum Expr {
	Literal(ExprLiteral),
	Unary(ExprUnary),
	Call(ExprCall),
	Binary(ExprBinary),
	Grouping(ExprGrouping),
	Variable(ExprVariable),
	Assignment(ExprAssignment),
	Logical(ExprLogical),
}

impl ExprAccept for Expr {
	fn accept(self) -> String {
		match self {
			Expr::Literal(x) => x.accept(),
			Expr::Unary(u) => u.accept(),
			Expr::Call(c) => c.accept(),
			Expr::Binary(b) => b.accept(),
			Expr::Grouping(g) => g.accept(),
			Expr::Variable(v) => v.accept(),
			Expr::Assignment(a) => a.accept(),
			Expr::Logical(l) => l.accept(),
		}
	}
}

impl Expr {
	pub fn parenthesize<T>(name: String, exprs: Vec<T>) -> String
		where T: ExprAccept
	{
		let mut builder = String::new();

		builder.push_str("(");
		builder.push_str(&name);

		for expr in exprs {
			builder.push_str(" ");
			builder.push_str(&expr.accept())
		}

		builder.push_str(")");
		
		builder

	}
}

#[derive(Clone)]
pub enum ExprLiteral {
	NUMBER(f64),
	STRING(String),
	True,
	False,
	Null
}

impl Expr {
	pub fn new_binary(left: Expr, operator: Token, right: Expr) -> Expr {
		Expr::Binary(ExprBinary {left: Box::new(left), operator, right: Box::new(right)})
	}

	pub fn new_unary(operator: Token, right: Expr) -> Expr {
		Expr::Unary(ExprUnary { operator, right: Box::new(right) })
	}

	pub fn new_grouping(expr: Expr) -> Expr {
		Expr::Grouping(ExprGrouping(Box::new(expr)))
	}

	pub fn new_variable(name: Token) -> Expr {
		Expr::Variable(ExprVariable {name})
	}

	pub fn new_assignment(name: Token, value: Expr) -> Expr {
		Expr::Assignment(ExprAssignment {name, value: Box::new(value)})
	}

}

impl ToString for ExprLiteral {

	fn to_string(&self) -> String {
		match self {
			ExprLiteral::NUMBER(n) => {format!("{:?}", n)},
			ExprLiteral::STRING(s) => {s.clone()},
			ExprLiteral::True => {"true".to_string()},
			ExprLiteral::False => {"false".to_string()},
			ExprLiteral::Null => {"nil".to_string()},
		}
	}
}

#[derive(Clone)]
pub struct ExprGrouping(pub Box<Expr>);

#[derive(Clone)]
pub struct ExprUnary {
	pub operator: Token,
	pub right: Box<Expr>
}

#[derive(Clone)]
pub struct ExprCall {
	pub callee: Box<Expr>,
	pub paren: Token,
	pub arguments: Vec<Expr>
}


#[derive(Clone)]
pub struct ExprBinary {
	pub left: Box<Expr>,
	pub operator: Token,
	pub right: Box<Expr>
}

#[derive(Clone)]
pub struct ExprLogical {
	pub left: Box<Expr>,
	pub operator: Token,
	pub right: Box<Expr>
}

#[derive(Clone)]
pub struct ExprVariable {
	pub name: Token
}

#[derive(Clone)]
pub struct ExprAssignment {
	pub name: Token,
	pub value: Box<Expr>
}

impl ExprBinary {
		pub fn new(left: Expr, operator: Token, right: Expr) -> Self {
			Self { left: Box::new(left), operator, right: Box::new(right) }
		}
}

pub trait ExprAccept {
	fn accept(self) -> String;
}

impl ExprAccept for ExprBinary {
	fn accept(self) -> String {
		Expr::parenthesize(self.operator.lexeme, vec![*self.left, *self.right])
	}
}

impl ExprAccept for ExprUnary {
	fn accept(self) -> String {
		Expr::parenthesize(self.operator.lexeme, vec![*self.right])
	}
}

impl ExprAccept for ExprCall {
	fn accept(self) -> String {
		self.paren.lexeme
	}
}

impl ExprAccept for ExprLiteral {
	fn accept(self) -> String {
		self.to_string()
	}
}

impl ExprAccept for ExprLogical {
	fn accept(self) -> String {
		Expr::parenthesize(self.operator.lexeme, vec![*self.left, *self.right])
	}
}

impl ExprAccept for ExprGrouping {
	fn accept(self) -> String {
		Expr::parenthesize("group".to_string(), vec![*self.0])
	}
}

impl ExprAccept for ExprVariable {
	fn accept(self) -> String {
		self.name.to_string()
	}
}

impl ExprAccept for ExprAssignment {
	fn accept(self) -> String {
		String::new()
	}
}

pub struct AstPrinter;

impl AstPrinter {
	pub fn print(expr: Expr) -> String{
		return expr.accept()
	}
}