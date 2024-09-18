use crate::scanner::token::Token;

pub enum Expr {
	Literal(ExprLiteral),
	Unary{operator: Token, right: Box<Expr>},
	Binary{left: Box<Expr>, operator: Token, right: Box<Expr>},
	Grouping(Box<Expr>),
}


impl ExprAccept for Expr {
	fn accept(self) -> String {
		match self {
			Expr::Literal(x) => x.accept(),
			Expr::Unary{operator, right} => {
				Self::parenthesize(operator.lexeme, vec![*right])
			},
			Expr::Binary{left, operator, right} => {
				Expr::parenthesize(operator.lexeme, vec![*left, *right])
			},
			Expr::Grouping(g) => {
				Self::parenthesize("group".to_string(), vec![*g])
			},
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

pub enum ExprLiteral {
	NUMBER(f64),
	STRING(String),
	True,
	False,
	Null
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
pub trait ExprAccept {
	fn accept(self) -> String;
}

impl ExprAccept for ExprLiteral {
	fn accept(self) -> String {
		self.to_string()
	}
}

/// `NEW` methods for ExprVariants
impl Expr {
	pub fn new_binary(left: Expr, operator: Token, right: Expr) -> Self {
		Expr::Binary{ left: Box::new(left), operator, right: Box::new(right) }
	}

	pub fn new_unary(operator: Token, right: Expr) -> Self {
		Expr::Unary { operator, right: Box::new(right) }
	}

	pub fn new_grouping(grouping: Expr) -> Self {
		Expr::Grouping(Box::new(grouping))
	}

	pub fn new_literal(literal: ExprLiteral) -> Self {
		Expr::Literal(literal)
	}
}

pub struct AstPrinter;

impl AstPrinter {
	pub fn print(expr: Expr) -> String{
		return expr.accept()
	}
}