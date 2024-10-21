use expr::{Expr, ExprCall, ExprGet, ExprLiteral, ExprLogical, ExprThis};
use error::{ParserError, ParserResult};

use crate::scanner::token::{Literal, Token, TokenType};

pub mod expr;
pub mod error;

/// A struct representing the parser, moving token by token
pub struct Parser {
	pub tokens: Vec<Token>,
	current: usize,
	had_error: bool,
	pub loop_depth: usize,
}

impl Parser {
	/// Initialize a new parser
	pub fn new(tokens: Vec<Token>) -> Self {
		Parser {tokens, current: 0, had_error: false, loop_depth: 0}
	}
}

impl Parser {
	/// Begin parsing
	pub fn parse(&mut self) -> Option<Expr> {
		self.expression().ok()
	}

	/// Parse an expression
	pub fn expression(&mut self) -> ParserResult<Expr> {
		return self.assignment()
	}

	/// Parse an assignment
	pub fn assignment(&mut self) -> ParserResult<Expr> {
		let expr = self.or()?;

		if self.match_next(vec![TokenType::EQUAL]) {
			let equals = self.previous();
			let value = self.assignment()?;

			match expr {
				Expr::Variable(v) => {
					let name = v.name;
					return Ok(Expr::new_assignment(name, value))
				},
				Expr::Get(g) => {
					return Ok(Expr::new_set(*g.object, g.name, value))
				}
				_ => return Err(ParserError::new(equals, "Invalid assignment target"))
			}
		}

		Ok(expr)
	}

	/// Parse a logical or
	pub fn or(&mut self) -> ParserResult<Expr> {
		let mut expr = self.and()?;

		while self.match_next(vec![TokenType::OR]) {
			let operator = self.previous();
			let right = Box::new(self.and()?);

			expr = Expr::Logical(ExprLogical { left: Box::new(expr), operator, right});
		}

		Ok(expr)
	}

	/// Parse a Logical and
	pub fn and(&mut self) -> ParserResult<Expr> {
		let mut expr = self.equality()?;

		while self.match_next(vec![TokenType::AND]) {
			let operator = self.previous();
			let right = Box::new(self.equality()?);

			expr = Expr::Logical(ExprLogical {left: Box::new(expr), operator, right});
		}

		Ok(expr)
	}

	/// Parse equality
	pub fn equality(&mut self) -> ParserResult<Expr> {
		let mut expr = self.comparison()?;

		while self.match_next(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
			let operator = self.previous();
			let right = self.comparison()?;

			expr = Expr::new_binary(expr, operator, right);
		}

		return Ok(expr);
	}

	/// Check if the current token matches at least one in a given token. If true, it advances "current"
	/// and returns true, returns false otherwise
	pub fn match_next(&mut self, token_types: Vec<TokenType>) -> bool {
		for token_type in token_types {
			if self.check(token_type) {
				self.advance();
				return true
			}
		}

		return false
	}

	/// Moves "current" one step forward if not at end of file
	pub fn advance(&mut self) -> Token {
		if !self.is_at_end() {
			self.current += 1;
		}
		return self.previous();
	}


	/// Checks if the current token's type matches the given token's type
	pub fn check(&self, token_type: TokenType) -> bool {
		if self.is_at_end() {return false}
		return self.peek().token_type == token_type
	}

	/// Checks if the end of the file has been reached;
	pub fn is_at_end(&self) -> bool {
		self.peek().token_type == TokenType::EOF
	}

	/// Gets the current token;
	pub fn peek(&self) -> Token {
		return self.tokens.get(self.current).unwrap().clone()
	}

	/// Gets the previous token
	pub fn previous(&self) -> Token {
		return self.tokens.get(self.current - 1).unwrap().clone()
	}


	/// Parse comparison
	pub fn comparison(&mut self) -> ParserResult<Expr> {
		let mut expr = self.term()?;

		while self.match_next(vec![TokenType::GREATER, TokenType::GREATER_EQUAL, TokenType::LESS, TokenType::LESS_EQUAL]) {
			let operator = self.previous();
			let right = self.term()?;

			expr = Expr::new_binary(expr, operator, right);
		}

		Ok(expr)
	}

	/// Parse a term
	pub fn term(&mut self) -> ParserResult<Expr> {
		let mut expr = self.factor()?;

		while self.match_next(vec![TokenType::MINUS, TokenType::PLUS]) {
			// If invalid LHS
			match expr {
				Expr::Literal(ExprLiteral::Null) => {return Err(self.error(self.previous(), "Invalid LHS for binary expression"))},
				_ => {}
			}

			let operator = self.previous();
			let right = self.factor()?;

			match right {
				Expr::Literal(ExprLiteral::Null) => {return Err(self.error(self.peek(), "Invalid RHS for binary expression"))},
				_ => {}
			}

			expr = Expr::new_binary(expr, operator, right);
		}

		Ok(expr)
	}

	/// Parse a factor
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


		return self.call()
	}
	
	/// Begin parsing a call
	pub fn call(&mut self) -> ParserResult<Expr> {
		let mut expr = self.primary()?;

		loop {
			if self.match_next(vec![TokenType::LEFT_PAREN]) {
				expr = self.finish_call(expr)?;
			} else if self.match_next(vec![TokenType::DOT]) {
				let name = self.consume(TokenType::IDENTIFIER, "Expect property name after '.'")?;
				expr = Expr::Get(ExprGet {name, object: Box::new(expr)})
			} else {
				break
			}
		}

		return Ok(expr)
	}

	/// Finish parsing a call
	pub fn finish_call(&mut self, callee: Expr) -> ParserResult<Expr> {
		let mut arguments = Vec::new();

		if !self.check(TokenType::RIGHT_PAREN) {
			loop {
				if arguments.len() >= 255 {
					self.error(self.peek(), "Can't have more than 255 arguments");
				}
				arguments.push(self.expression()?);
				if !self.match_next(vec![TokenType::COMMA]) {
					break
				}
			}
		}

		let paren = self.consume(TokenType::RIGHT_PAREN, "Expect ')' after arguments")?;

		Ok(Expr::Call(ExprCall {callee: Box::new(callee), arguments, paren}))


	}

	/// Parse a primary expression
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

		if self.match_next(vec![TokenType::THIS]) {
			return Ok(Expr::This(ExprThis {keyword:  self.previous()}))
		}

		if self.match_next(vec![TokenType::IDENTIFIER]) {
			return Ok(Expr::new_variable(self.previous()))
		}

		if self.match_next(vec![TokenType::LEFT_PAREN]) {
			let expr = self.expression()?;
			self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression")?;
			return Ok(Expr::new_grouping(expr));
		}

		return Ok(Expr::Literal(ExprLiteral::Null));

	}

	/// Expect a given token to be at the current position, throws an error otherwise
	pub fn consume(&mut self, token_type: TokenType, message: &str) -> ParserResult<Token> {
		if self.check(token_type) {
			return Ok(self.advance())
		}

		return Err(self.error(self.peek(), message))
	}

	/// Generate a ParseeError
	pub fn error(&mut self, token: Token, message: &str) -> ParserError {
		self.had_error = true;
		let error = ParserError::new(token, message);
		error.error();
		error
	}

	/// Synchronize the curr in the event of bad syntax
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
