use super::scanner;

use std::boxed::Box;

#[derive(Debug, Clone)]
pub struct Data {
	variable: String,
	datatype: Option<String>
}

#[derive(Debug, Clone)]
pub enum Parser {
	Tree(Box<Parser>, Data, Box<Parser>),
	IfValue(Box<Parser>, Box<Parser>, Box<Parser>),
	Value(Data),
	PlaceHolder
}

pub fn parse_tokens(tokens: &mut Vec<String>) -> Parser {
	parse_statement(tokens)
}

pub fn printTree(tree: &Parser, output: &mut String, tab: &mut i32) {
	/* 
	// Print with tabs in mind
	for _ in 0..*tab {
		*output = format!("{}	", output);
	}
	
	match tree {
		Parser::Tree(left, symbol, right) => {
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
				Parser::IfValue(expr, s1, s2) => {
					for _ in 0..*tab {
						*output = format!("{}	", output);
					}
					
					*output = format!("{}IF\n", output);
					
					printTree(&expr, output, tab);
					*tab = *tab - 1;
					printTree(&s1, output, tab);
					*tab = *tab - 1;
					printTree(&s2, output, tab);
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
				Parser::IfValue(expr, s1, s2) => {
					for _ in 0..*tab {
						*output = format!("{}	", output);
					}
					
					*output = format!("{}IF\n", output);
					
					*tab = *tab + 1;
					
					printTree(&expr, output, tab);
					*tab = *tab - 1;
					printTree(&s1, output, tab);
					*tab = *tab - 1;
					printTree(&s2, output, tab);
					*tab = *tab - 1;
					
				}
				_ => ()
			}
		}
		
		Parser::IfValue(expr, s1, s2) => {
			*output = format!("{}IF\n", output);
			
			*tab = *tab + 1;
			
			println!("{:?}", expr);
			printTree(&expr, output, tab);
			*tab = *tab - 1;
			printTree(&s1, output, tab);
			*tab = *tab - 1;
			printTree(&s2, output, tab);
			
		}
		_ => ()
	}
	*/
	// Perform tabbing on output
	for _ in 0..*tab {
		*output = format!("{}	", output);
	}
	
	match tree {
		Parser::Tree(left, symbol, right) => {
			// Push Symbol into tree and incremement tabbing
			if symbol.datatype.is_some() {
				*output = format!("{}{} : {}\n", output, symbol.variable, symbol.datatype.clone().unwrap());
			} else {
				*output = format!("{}{}\n", output, symbol.variable);
			}
			
			*tab += 1;
			
			// Recursively Parse Left Tree
			printTree(&left, output, tab);
			
			// Recursively Parse Right Tree
			printTree(&right, output, tab);
			
			*tab -= 1;
		}
		
		Parser::IfValue(expr, s1, s2) => {
			*output = format!("{}IF\n", output);
			
			*tab += 1;
			
			printTree(&expr, output, tab);
			printTree(&s1, output, tab);
			printTree(&s2, output, tab);
			
			*tab -= 1;
		}
		
		Parser::Value(value) => {
			if value.datatype.is_some() {
				*output = format!("{}{} : {}\n", output, value.variable, value.datatype.clone().unwrap());
			} else {
				*output = format!("{}{}\n", output, value.variable);
			}
		}
		
		_ => ()
	}
}

fn parse_statement(tokens: &mut Vec<String>) -> Parser {
	let left = parse_basestatement(tokens);
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
	while current_token == String::from(";") {
		tokens.remove(0);
		let right = parse_basestatement(tokens);
		
		match right {
			Parser::PlaceHolder => {
				return tree
				
			}
			_ => ()
		}
		let data = Data {
			variable: ";".to_string(),
			datatype: Some("SYMBOL".to_string()),
		};
		
		tree = Parser::Tree(Box::new(tree), data, Box::new(right));
		
		if tokens.get(0).is_none() {
			break;
		}
		
		current_token = tokens.get(0).unwrap().to_string().clone();
	}
	tree
}

fn parse_basestatement(tokens: &mut Vec<String>) -> Parser  {
	
	// Each check will return PlaceHolder if it doesn't apply
	
	// parse_assignment
	
	let assignment_result = parse_assignment(tokens);
	
	
	match assignment_result {
		Parser::PlaceHolder => (),
		_ => {
			return assignment_result;
		}
	}
	
	// parse_ifstatement
	let if_result = parse_ifstatement(tokens);
	
	match if_result {
		Parser::PlaceHolder => (),
		_ => {
			return if_result;
		}
	}
	
	// parse_whilestatement
	let while_result = parse_whilestatement(tokens);
	
	match while_result {
		Parser::PlaceHolder => (),
		_ => {
			return while_result;
		}
	}
	
	// skip
	let current_token = tokens.get(0);
	
	if current_token.is_none() {
		// edge case for empty string
		return Parser::PlaceHolder
	}
	
	if current_token.unwrap() == "skip" {
		let data = Data {
			variable: String::from("SKIP"),
			datatype: Some(String::from("KEYWORD"))
		};
		
		return Parser::Value(data);
	}
	
	// if none are true return PlaceHolder
	return Parser::PlaceHolder
}

fn parse_assignment(tokens: &mut Vec<String>) -> Parser  {
	// IDENTIFIER := expression
	
	/****** Identifier ******/
	
	let identifier = {
		let identifier_token = tokens.get(0);
		
		if identifier_token.is_none() {
			return Parser::PlaceHolder
		}
		
		// replace identifier_token with unwrapped version since above confirmed not empty
		let identifier_token = identifier_token.unwrap();
		
		// Check for identifier on the front
		
		let mut scan_keyword = identifier_token.clone();
		scanner::scan_keyword(&mut scan_keyword);
		
		if scan_keyword == "" {
			return Parser::PlaceHolder;
		}
		
		let mut scan_ident = identifier_token.clone();
		scanner::scan_identifier(&mut scan_ident);
		
		// Make sure first token is an identifier
		if scan_ident != "" {
			return Parser::PlaceHolder;
		}
		
		let identifier = Parser::Value (Data {
			variable: identifier_token.to_string(),
			datatype: Some("IDENTIFIER".to_string())
		});
		
		// Remove idenfiier from token list
		tokens.remove(0);
		identifier
	};
	
	/****** Symbol ******/
	
	let symbol = {
		let symbol = tokens.get(0).unwrap();
	
	// Check for the middle symbol to be :=
		if symbol != ":=" {
			return Parser::PlaceHolder;
		}
		
		// Convert symbol to the Data enum
		let symbol = Data {
			variable: ":=".to_string(),
			datatype: Some("SYMBOL".to_string())
		};
		
		// Remove Symbol from token list
		tokens.remove(0);
		
		symbol
	};
	
	/****** Expression ******/
	
	let expression = parse_expression(tokens);
	
	
	return Parser::Tree(Box::new(identifier), symbol, Box::new(expression));
}

fn parse_ifstatement(tokens: &mut Vec<String>) -> Parser  {
	// If
	{
		let current_token = tokens.get(0);
		
		if current_token.is_none() || current_token.unwrap() != "if" {
			
			return Parser::PlaceHolder
		}
		
		tokens.remove(0);
	}
	
	let expression = parse_expression(tokens);
	
	// Then
	{
		let current_token = tokens.get(0);
		
		if current_token.is_none() || current_token.unwrap() != "then" {
			
			return Parser::PlaceHolder
		}
		
		tokens.remove(0);
	}
	
	let statement1 = parse_statement(tokens);
	
	// Else
	{
		let current_token = tokens.get(0);
		
		if current_token.is_none() || current_token.unwrap() != "else" {
			
			return Parser::PlaceHolder
		}
		
		tokens.remove(0);
	}
	
	let statement2 = parse_statement(tokens);
	
	// Endif
	{
		let current_token = tokens.get(0);
		
		if current_token.is_none() || current_token.unwrap() != "endif" {
			return Parser::PlaceHolder
		}
		
		tokens.remove(0);
	}
	
	return Parser::IfValue(Box::new(expression), Box::new(statement1), Box::new(statement2));
}

fn parse_whilestatement(tokens: &mut Vec<String>) -> Parser  {
	return Parser::PlaceHolder
	
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
			datatype: Some("SYMBOL".to_string()),
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
			datatype: Some("SYMBOL".to_string()),
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
			datatype: Some("SYMBOL".to_string()),
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
			datatype: Some("SYMBOL".to_string()),
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
			datatype: Some("IDENTIFIER".to_string())
		};
		
		return Parser::Value(data)
		
	}
	
	if scan_num == "" {
		tokens.remove(0);
		
		let data = Data {
			variable: current_token.to_string(),
			datatype: Some("NUMBER".to_string())
		};
		
		return Parser::Value(data)
	}
	
	
	
	return Parser::PlaceHolder;
}