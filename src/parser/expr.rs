use crate::scanner::token::Token;

pub enum Expr {
	Literal(ExprLiteral),
	Unary{operator: Token, right: Box<Expr>},
	Binary{left: Box<Expr>, operator: Token, right: Box<Expr>},
	Grouping(Box<Expr>),
}


impl Expr {
	/// Method to accept expressions for printing
	fn accept(self) -> String {
		match self {
			Expr::Literal(x) => x.to_string(),
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
	/// Parenthesize a given expression
	pub fn parenthesize(name: String, exprs: Vec<Expr>) -> String
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

/// Storage for literal expressions
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

/// `NEW` methods for ExprVariants
impl Expr {
	/// Create a new binary expression
	pub fn new_binary(left: Expr, operator: Token, right: Expr) -> Self {
		Expr::Binary{ left: Box::new(left), operator, right: Box::new(right) }
	}
	
	/// Create a new unary expression
	pub fn new_unary(operator: Token, right: Expr) -> Self {
		Expr::Unary { operator, right: Box::new(right) }
	}
	
	/// Create a new grouping expression
	pub fn new_grouping(grouping: Expr) -> Self {
		Expr::Grouping(Box::new(grouping))
	}
	
	/// Create a new literal expression
	pub fn new_literal(literal: ExprLiteral) -> Self {
		Expr::Literal(literal)
	}
}

/// In charge of printing expressions to stdout
pub struct AstPrinter;

impl AstPrinter {
	/// Print an expression to stdout
	pub fn print(expr: Expr) -> String{
		return expr.accept()
	}
}