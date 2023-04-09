use super::parser::*;
use std::collections::VecDeque;

pub fn evaluate_tree(tree: &Parser, stack: &mut VecDeque<Data>) {
	match tree {
		Parser::Tree(left, symbol, right) => {
			stack.push_front(symbol.clone());
			
			// Eval Left Hand Side
			evaluate_tree(left, stack);
			if stack.get(0).is_some() && stack.get(1).is_some() && stack.get(2).is_some() && stack.get(0).unwrap().clone().datatype == "NUMBER" && stack.get(1).unwrap().clone().datatype == "NUMBER" && stack.get(2).unwrap().clone().datatype == "SYMBOL" {
				let num1: i32 = stack.get(0).unwrap().clone().variable.parse().unwrap();
				let num2: i32 = stack.get(1).unwrap().clone().variable.parse().unwrap();
				let symbol: String = stack.get(2).unwrap().clone().variable.parse().unwrap();
				
				stack.remove(0).unwrap();
				stack.remove(0).unwrap();
				stack.remove(0).unwrap();
				
				let result_data = perform_calculation(num1, symbol, num2);
				
				if result_data.variable == "Error".to_string() {
					stack.clear();
					stack.push_front(Data {variable: "Error".to_string(), datatype: "Error".to_string()})
				}
				
				stack.push_front(result_data);
			}
			
			// Eval Right Hand Side
			evaluate_tree(right, stack);
			if stack.get(0).is_some() && stack.get(1).is_some() && stack.get(2).is_some() && stack.get(0).unwrap().clone().datatype == "NUMBER" && stack.get(1).unwrap().clone().datatype == "NUMBER" && stack.get(2).unwrap().clone().datatype == "SYMBOL" {
				let num1: i32 = stack.get(0).unwrap().clone().variable.parse().unwrap();
				let num2: i32 = stack.get(1).unwrap().clone().variable.parse().unwrap();
				let symbol: String = stack.get(2).unwrap().clone().variable.parse().unwrap();
				
				stack.remove(0).unwrap();
				stack.remove(0).unwrap();
				stack.remove(0).unwrap();
				
				let result_data = perform_calculation(num1, symbol, num2);
				
				if result_data.variable == "Error".to_string() {
					stack.clear();
					stack.push_front(Data {variable: "Error".to_string(), datatype: "Error".to_string()})
				}
				
				stack.push_front(result_data);
				
			}
		}
		Parser::Value(val) => {
			if stack.get(0).unwrap().variable == "Error".to_string() {
				return
			}
			
			stack.push_front(val.clone());
		}
		Parser::PlaceHolder => ()
	}
}

fn perform_calculation(num1: i32, symbol: String, num2: i32) -> Data {
	let mut result = 0;
	match symbol.as_str() {
		"-" => {
			result = num2 - num1;
		}
		"+" => {
			result = num2 + num1;
		}
		"*" => {
			result = num2 * num1;
		}
		"/" => {
			if num1 == 0 {
				return Data {
					variable: "Error".to_string(), 
					datatype: "Error".to_string()
				}
			}
			result = num2 / num1;
		}
		_ => ()
	}
	
	Data {
		variable: result.to_string(),
		datatype: "NUMBER".to_string()
	}
}