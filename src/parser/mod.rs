use expr::{Expr, ExprLiteral};
use error::{ParserError, ParserResult};

use crate::scanner::token::{Literal, Token, TokenType};

pub mod expr;
pub mod error;

pub struct Parser {
	pub tokens: Vec<Token>,
	current: usize,
	had_error: bool
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		Parser {tokens, current: 0, had_error: false}
	}
}

impl Parser {
	pub fn parse(&mut self) -> Option<Expr> {
		self.expression().ok()
	}

	pub fn expression(&mut self) -> ParserResult<Expr> {
		return self.assignment()
	}

	pub fn assignment(&mut self) -> ParserResult<Expr> {
		let expr = self.equality()?;

		if self.match_next(vec![TokenType::EQUAL]) {
			let equals = self.previous();
			let value = self.assignment()?;

			match expr {
				Expr::Variable(v) => {
					let name = v.name;
					return Ok(Expr::new_assignment(name, value))
				},
				_ => return Err(ParserError::new(equals, "Invalid assignment target".to_string()))
			}
		}

		Ok(expr)
	}

	pub fn equality(&mut self) -> ParserResult<Expr> {
		let mut expr = self.comparison()?;

		while self.match_next(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
			let operator = self.previous();
			let right = self.comparison()?;

			expr = Expr::new_binary(expr, operator, right);
		}

		return Ok(expr);
	}

	pub fn match_next(&mut self, token_types: Vec<TokenType>) -> bool {
		for token_type in token_types {
			if self.check(token_type) {
				self.advance();
				return true
			}
		}

		return false
	}

	pub fn advance(&mut self) -> Token {
		if !self.is_at_end() {
			self.current += 1;
		}
		return self.previous();
	}


	pub fn check(&self, token_type: TokenType) -> bool {
		if self.is_at_end() {return false}
		return self.peek().token_type == token_type
	}

	pub fn is_at_end(&self) -> bool {
		self.peek().token_type == TokenType::EOF
	}

	pub fn peek(&self) -> Token {
		return self.tokens.get(self.current).unwrap().clone()
	}

	pub fn previous(&self) -> Token {
		return self.tokens.get(self.current - 1).unwrap().clone()
	}


	// Comparison
	pub fn comparison(&mut self) -> ParserResult<Expr> {
		let mut expr = self.term()?;

		while self.match_next(vec![TokenType::GREATER, TokenType::GREATER_EQUAL, TokenType::LESS, TokenType::LESS_EQUAL]) {
			let operator = self.previous();
			let right = self.term()?;

			expr = Expr::new_binary(expr, operator, right);
		}

		Ok(expr)
	}

	pub fn term(&mut self) -> ParserResult<Expr> {
		let mut expr = self.factor()?;

		while self.match_next(vec![TokenType::MINUS, TokenType::PLUS]) {
			// If invalid LHS
			match expr {
				Expr::Literal(ExprLiteral::Null) => {return Err(self.error(self.previous(), format!("Invalid LHS for binary expression")))},
				_ => {}
			}

			let operator = self.previous();
			let right = self.factor()?;

			match right {
				Expr::Literal(ExprLiteral::Null) => {return Err(self.error(self.peek(), format!("Invalid RHS for binary expression")))},
				_ => {}
			}

			expr = Expr::new_binary(expr, operator, right);
		}

		Ok(expr)
	}

	pub fn factor(&mut self) -> ParserResult<Expr> {
		let mut expr = self.unary()?;

		while self.match_next(vec![TokenType::SLASH, TokenType::STAR]) {
			let operator = self.previous();
			let right = self.unary()?;
			
			expr = Expr::new_binary(expr, operator, right);
		}

		Ok(expr)
	}

	pub fn unary(&mut self) -> ParserResult<Expr> {
		if self.match_next(vec![TokenType::BANG, TokenType::MINUS]) {
			let operator = self.previous();
			let right = self.unary()?;
			return Ok(Expr::new_unary(operator, right))
		}


		return self.primary()
	}

	pub fn primary(&mut self) -> ParserResult<Expr> {
		if self.match_next(vec![TokenType::FALSE]) {return Ok(Expr::Literal(ExprLiteral::False))}
		if self.match_next(vec![TokenType::TRUE]) {return Ok(Expr::Literal(ExprLiteral::True))}
		if self.match_next(vec![TokenType::NIL]) {return Ok(Expr::Literal(ExprLiteral::Null))}

		if self.match_next(vec![TokenType::NUMBER]) {
			let v = match self.previous().literal {
				Literal::Float(x) => x,
				_ => 0.0
			};
			return Ok(Expr::Literal(ExprLiteral::NUMBER(v)))
		}

		if self.match_next(vec![TokenType::STRING]) {
			let v = match self.previous().literal {
				Literal::String(x) => x,
				_ => "".to_string()
			};
			return Ok(Expr::Literal(ExprLiteral::STRING(v)))
		}

		if self.match_next(vec![TokenType::IDENTIFIER]) {
			return Ok(Expr::new_variable(self.previous()))
		}

		if self.match_next(vec![TokenType::LEFT_PAREN]) {
			let expr = self.expression()?;
			self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression".to_string())?;
			return Ok(Expr::new_grouping(expr));
		}

		return Ok(Expr::Literal(ExprLiteral::Null));

	}

	pub fn consume(&mut self, token_type: TokenType, message: String) -> ParserResult<Token> {
		if self.check(token_type) {
			return Ok(self.advance())
		}

		return Err(self.error(self.peek(), message))
	}

	pub fn error(&mut self, token: Token, message: String) -> ParserError {
		self.had_error = true;
		let error = ParserError::new(token, message);
		error.error();
		error
	}

	pub fn synchronize(&mut self) {
		self.advance();

		while !self.is_at_end() {
			if self.previous().token_type == TokenType::SEMICOLON {return}

			match self.peek().token_type {
				TokenType::CLASS | TokenType::FUN | TokenType::VAR
				| TokenType::FOR | TokenType::IF | TokenType::WHILE
				| TokenType::PRINT | TokenType::RETURN  => return,
				_ => {}
			}

			self.advance();
		}
	}

}
