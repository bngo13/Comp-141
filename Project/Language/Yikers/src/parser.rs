use super::scanner;

use std::boxed::Box;

#[derive(Debug, Clone)]
pub enum Parser {
	Tree(Box<Parser>, String, Box<Parser>),
	Value(String),
	PlaceHolder
}

pub fn parse_tokens(tokens: &mut Vec<String>) -> Parser {
	parse_expression(tokens)
}

fn parse_expression(tokens: &mut Vec<String>) -> Parser {
	// Term {+ Term}
	let left = parse_term(tokens);
	match left {
		Parser::PlaceHolder => {return left}
		_ => ()
	}
	
	
	let mut tree = left.clone();
	
	
	let current_token = tokens.get(0);
	if current_token.is_none() {
		return left;
	}
	
	let mut current_token = current_token.unwrap().clone();
	
	
	
	
	while current_token == "+" {
		tokens.remove(0);
		let right = parse_term(tokens);
		match right {
			Parser::PlaceHolder => {return right}
			_ => ()
		}
	
		tree = Parser::Tree(Box::new(tree), "+".to_string(), Box::new(right));
		
		if tokens.get(0).is_none() {
			break;
		}
		
		current_token = tokens.get(0).unwrap().to_string().clone();
		
	}
	tree
}

fn parse_term(tokens: &mut Vec<String>) -> Parser {
	// Factor {- Factor}
	let left = parse_factor(tokens);
	match left {
		Parser::PlaceHolder => {return left}
		_ => ()
	}
	
	
	let mut tree = left.clone();
	
	
	let current_token = tokens.get(0);
	if current_token.is_none() {
		return left;
	}
	
	let mut current_token = current_token.unwrap().clone();
	
	
	
	
	while current_token == "-" {
		tokens.remove(0);
		let right = parse_factor(tokens);
		match right {
			Parser::PlaceHolder => {return right}
			_ => ()
		}
	
		tree = Parser::Tree(Box::new(tree), "-".to_string(), Box::new(right));
		
		if tokens.get(0).is_none() {
			break;
		}
		
		current_token = tokens.get(0).unwrap().to_string().clone();
		
	}
	tree
}

fn parse_factor(tokens: &mut Vec<String>) -> Parser {
	// Piece {/ Piece}
	let left = parse_piece(tokens);
	match left {
		Parser::PlaceHolder => {return left}
		_ => ()
	}
	
	
	let mut tree = left.clone();
	
	
	let current_token = tokens.get(0);
	if current_token.is_none() {
		return left;
	}
	
	let mut current_token = current_token.unwrap().clone();
	
	
	
	
	while current_token == "/" {
		tokens.remove(0);
		let right = parse_piece(tokens);
		match right {
			Parser::PlaceHolder => {return right}
			_ => ()
		}
	
		tree = Parser::Tree(Box::new(tree), "/".to_string(), Box::new(right));
		
		if tokens.get(0).is_none() {
			break;
		}
		
		current_token = tokens.get(0).unwrap().to_string().clone();
		
	}
	tree
}

fn parse_piece(tokens: &mut Vec<String>) -> Parser{
	// Element {* Element}
	let left = parse_element(tokens);
	match left {
		Parser::PlaceHolder => {return left}
		_ => ()
	}
	
	let mut tree = left.clone();
	
	
	let current_token = tokens.get(0);
	if current_token.is_none() {
		return left;
	}
	
	let mut current_token = current_token.unwrap().clone();
	
	
	
	
	while current_token == "*" {
		tokens.remove(0);
		let right = parse_element(tokens);
		match right {
			Parser::PlaceHolder => {return right}
			_ => ()
		}
	
		tree = Parser::Tree(Box::new(tree), "*".to_string(), Box::new(right));
		
		if tokens.get(0).is_none() {
			break;
		}
		
		current_token = tokens.get(0).unwrap().to_string().clone();
		
	}
	tree
	
}

fn parse_element(tokens: &mut Vec<String>) -> Parser {
	// (expression) | NUMBER | IDENTIFIER
	
	let current_token = tokens.get(0);
	if current_token.is_none() {
		return Parser::PlaceHolder;
	}
	
	let current_token = current_token.unwrap().clone();
	if current_token.contains("(") || current_token.contains(")") {
		// Consume token
		tokens.remove(0);
		let expr = parse_expression(tokens);
		tokens.remove(0);
		return expr
	}
	
	let mut scan_num = current_token.clone();
	scanner::scan_num(&mut scan_num);
	
	let mut scan_ident = current_token.clone();
	scanner::scan_identifier(&mut scan_ident);
	
	if scan_ident == "" || scan_num == "" {
		// Consume token
		tokens.remove(0);
		return Parser::Value(current_token.to_string())
		
	}
	
	return Parser::PlaceHolder;
}