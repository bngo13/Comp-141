use super::scanner;

use std::boxed::Box;

#[derive(Debug, Clone)]
pub struct Data {
	variable: String,
	datatype: String
}

#[derive(Debug, Clone)]
pub enum Parser {
	Tree(Box<Parser>, Data, Box<Parser>),
	Value(Data),
	PlaceHolder
}

pub fn parse_tokens(tokens: &mut Vec<String>) -> Parser {
	parse_expression(tokens)
}

pub fn printTree(tree: &Parser, output: &mut String, tab: &mut i32) {
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
					printTree(left, output, tab);
					*tab = *tab - 1;
				}
				_ => ()
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
					printTree(right, output, tab);
					*tab = *tab - 1;
				}
				_ => ()
			}
		}
		_ => ()
	}
}

fn parse_expression(tokens: &mut Vec<String>) -> Parser {
	// Term {+ Term}
	// Structure is Tree(left, symbol, right)
	let left = parse_term(tokens);
	if let Parser::PlaceHolder = left {return left}
	
	
	let mut tree = left.clone();
	
	
	let current_token = tokens.get(0);
	if current_token.is_none() {
		return left;
	}
	
	let mut current_token = current_token.unwrap().clone();
	
	
	
	
	while current_token == "+" {
		tokens.remove(0);
		let right = parse_term(tokens);
<<<<<<< HEAD
		match right {
			Parser::PlaceHolder => {return right}
			_ => ()
		}
		
		let data = Data {
			variable: "+".to_string(),
			datatype: "SYMBOL".to_string(),
		};
		
		tree = Parser::Tree(Box::new(tree), data, Box::new(right));
=======
		if let Parser::PlaceHolder = right {return right}
	
		tree = Parser::Tree(Box::new(tree), "+".to_string(), Box::new(right));
>>>>>>> a65fa40 (Finished Lab12)
		
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
<<<<<<< HEAD
	match left {
		Parser::PlaceHolder => {return left}
		_ => ()
	}
=======
	if let Parser::PlaceHolder = left {return left}
	
	
>>>>>>> a65fa40 (Finished Lab12)
	let mut tree = left.clone();
	
	let current_token = tokens.get(0);
	if current_token.is_none() {
		return left;
	}
	
	let mut current_token = current_token.unwrap().clone();
	
	while current_token == "-" {
		tokens.remove(0);
		let right = parse_factor(tokens);
<<<<<<< HEAD
		match right {
			Parser::PlaceHolder => {return right}
			_ => ()
		}
		
		let data = Data {
			variable: "-".to_string(),
			datatype: "SYMBOL".to_string(),
		};
		
		tree = Parser::Tree(Box::new(tree), data, Box::new(right));
=======
		if let Parser::PlaceHolder = right {return right}
	
		tree = Parser::Tree(Box::new(tree), "-".to_string(), Box::new(right));
>>>>>>> a65fa40 (Finished Lab12)
		
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
	if let Parser::PlaceHolder = left {return left}
	
	
	let mut tree = left.clone();
	
	
	let current_token = tokens.get(0);
	if current_token.is_none() {
		return left;
	}
	
	let mut current_token = current_token.unwrap().clone();
	
	
	
	
	while current_token == "/" {
		tokens.remove(0);
		let right = parse_piece(tokens);
<<<<<<< HEAD
		match right {
			Parser::PlaceHolder => {return right}
			_ => ()
		}
		
		let data = Data {
			variable: "/".to_string(),
			datatype: "SYMBOL".to_string(),
		};
=======
		if let Parser::PlaceHolder = right {return right}
>>>>>>> a65fa40 (Finished Lab12)
	
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
	if let Parser::PlaceHolder = left {return left}
	
	let mut tree = left.clone();
	
	
	let current_token = tokens.get(0);
	if current_token.is_none() {
		return left;
	}
	
	let mut current_token = current_token.unwrap().clone();
	
	
	
	
	while current_token == "*" {
		tokens.remove(0);
		let right = parse_element(tokens);
		if let Parser::PlaceHolder = right {return right}
	
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