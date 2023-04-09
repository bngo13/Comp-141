use super::scanner;

use std::boxed::Box;

#[derive(Debug, Clone)]
pub struct Data {
	pub variable: String,
	pub datatype: String
}

#[derive(Debug, Clone)]
pub enum Parser {
	Tree(Box<Parser>, Data, Box<Parser>),
	Value(Data),
	PlaceHolder
}

pub fn parse_tokens(tokens: &mut Vec<String>) -> Parser {
	let mut parse_tree = parse_expression(tokens);
	
	// If tree was made but there are still tokens left to process, input is not valid
	if !tokens.is_empty() { parse_tree = Parser::PlaceHolder;}
	
	return parse_tree;
}

pub fn print_tree(tree: &Parser, output: &mut String, tab: &mut i32) {
	match tree {
		Parser::Tree(left, symbol, right) => {
			// Print with tabs in mind
			for _ in 0..*tab {
				*output = format!("{}	", output);
			}
			*output = format!("{}{} : {}\n", output, symbol.variable, symbol.datatype);
			
			// Increment tab count
			*tab = *tab + 1;
			
			// Handle Left
			match left.as_ref() {
				Parser::Value(val) => {
					for _ in 0..*tab {
						*output = format!("{}	", output);
					}
					*output = format!("{}{} : {}\n", output, val.variable, val.datatype);
				}
				Parser::Tree(_, _, _) => {
					print_tree(left, output, tab);
					*tab = *tab - 1;
				}
				Parser::PlaceHolder => {
					*output = format!("Invalid Parse Tree!")
				}
			}
			
			// Handle Right
			match right.as_ref() {
				Parser::Value(val) => {
					for _ in 0..*tab {
						*output = format!("{}	", output);
					}
					*output = format!("{}{} : {}\n", output, val.variable, val.datatype);
				}
				Parser::Tree(_, _, _) => {
					print_tree(right, output, tab);
					*tab = *tab - 1;
				}
				Parser::PlaceHolder => {
					*output = format!("Invalid Parse Tree!")
				}
			}
		}
		Parser::PlaceHolder | Parser::Value(_) => {
			*output = format!("Invalid Parse Tree!")
		}
	}
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
		
		let data = Data {
			variable: "+".to_string(),
			datatype: "SYMBOL".to_string(),
		};
		
		tree = Parser::Tree(Box::new(tree), data, Box::new(right));
		
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
		
		let data = Data {
			variable: "-".to_string(),
			datatype: "SYMBOL".to_string(),
		};
		
		tree = Parser::Tree(Box::new(tree), data, Box::new(right));
		
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
		
		let data = Data {
			variable: "/".to_string(),
			datatype: "SYMBOL".to_string(),
		};
	
		tree = Parser::Tree(Box::new(tree), data, Box::new(right));
		
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
	
		let data = Data {
			variable: "*".to_string(),
			datatype: "SYMBOL".to_string(),
		};
	
		tree = Parser::Tree(Box::new(tree), data, Box::new(right));
		
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
		if tokens.get(0).is_some() && tokens.get(0).unwrap() == ")" {
			tokens.remove(0);
		}
		return expr
	}
	
	let mut scan_num = current_token.clone();
	scanner::scan_num(&mut scan_num);
	
	let mut scan_ident = current_token.clone();
	scanner::scan_identifier(&mut scan_ident);
	
	if scan_ident == "" {
		// Consume token
		tokens.remove(0);
		
		let data = Data {
			variable: current_token.to_string(),
			datatype: "IDENTIFIER".to_string()
		};
		
		return Parser::Value(data)
		
	}
	
	if scan_num == "" {
		tokens.remove(0);
		
		let data = Data {
			variable: current_token.to_string(),
			datatype: "NUMBER".to_string()
		};
		
		return Parser::Value(data)
	}
	
	
	
	return Parser::PlaceHolder;
}