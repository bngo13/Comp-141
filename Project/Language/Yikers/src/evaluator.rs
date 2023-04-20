use super::parser::*;
use std::collections::VecDeque;
use std::collections::HashMap;

pub fn eval_tree(tree: &Parser, stack: &mut VecDeque<Data>, valuemap: &mut HashMap<String, i32>) {
	match tree {
		Parser::Tree(left, symbol, right) => {
			// While Handling
			if symbol.datatype.is_none() {
				handle_while(&left, &right, valuemap);
				return;
			}
			
			stack.push_front(symbol.clone());
			
			// Parse Left
			eval_tree(left, stack, valuemap);
			
			/* Result of left side */
			
			{ // Do Math Stuff if math stuff applies
				if 
					(stack.get(0).is_some() && stack.get(0).unwrap().datatype.clone().is_some() && (stack.get(0).unwrap().datatype.clone().unwrap() == "IDENTIFIER" || stack.get(0).unwrap().datatype.clone().unwrap() == "NUMBER")) && 
					(stack.get(1).is_some() && stack.get(1).unwrap().datatype.clone().is_some() && (stack.get(1).unwrap().datatype.clone().unwrap() == "IDENTIFIER" || stack.get(1).unwrap().datatype.clone().unwrap() == "NUMBER")) && 
					(symbol.variable == "+" || symbol.variable == "-" || symbol.variable == "*" || symbol.variable == "/") {
					handle_math(stack, valuemap);
				}
			}
			
			
			
			// Parse Right
			eval_tree(right, stack, valuemap);
			
			{ // Do Math Stuff
				if (stack.get(0).is_some() && stack.get(0).unwrap().datatype.clone().is_some() && (stack.get(0).unwrap().datatype.clone().unwrap() == "IDENTIFIER" || stack.get(0).unwrap().datatype.clone().unwrap() == "NUMBER")) && (stack.get(1).is_some() && stack.get(1).unwrap().datatype.clone().is_some() && (stack.get(1).unwrap().datatype.clone().unwrap() == "IDENTIFIER" || stack.get(1).unwrap().datatype.clone().unwrap() == "NUMBER")) && (symbol.variable == "+" || symbol.variable == "-" || symbol.variable == "*" || symbol.variable == "/") {
					handle_math(stack, valuemap);
				}
			}
			
			if symbol.variable == ":=".to_string() {
				handle_setter(stack, valuemap)
			}
			// Result of right side
		}
		Parser::IfValue(ifcondition, ifstatement, elsestatement) => {
			let mut ifstack: VecDeque<Data> = VecDeque::new();
			
			// Evaluate if condition
			eval_tree(&ifcondition, &mut ifstack, valuemap);
			
			if ifstack.pop_front().unwrap().variable == String::from("0") {
				eval_tree(ifstatement, stack, valuemap)
			} else {
				eval_tree(elsestatement, stack, valuemap)
			}
		}
		Parser::Value(terminal) => {
			if stack.get(0).unwrap().variable == "Error".to_string() {
				return
			}
			
			stack.push_front(terminal.clone());
		}
		Parser::PlaceHolder => ()
	}
}

fn handle_while(checkstatement: &Box<Parser>, whilestatement: &Box<Parser>, valuemap: &mut HashMap<String, i32>) {
	// Condition Checking
	let mut loop_stack: VecDeque<Data> = VecDeque::new();
	eval_tree(checkstatement, &mut loop_stack, &mut valuemap.clone());
	
	while 
		loop_stack.get(0).is_some() && loop_stack.get(0).unwrap().variable.parse::<i32>().unwrap() != 0 
		{
			
			println!("{:?}", loop_stack);
			// Eval while statement tree
			eval_tree(whilestatement, &mut VecDeque::new(), valuemap);
			
			// Test for looping condition
			loop_stack.clear();
			eval_tree(checkstatement, &mut loop_stack, &mut valuemap.clone());
		}
		
}

fn handle_math(stack: &mut VecDeque<Data>, valuemap: &mut HashMap<String, i32>) {
	// Get numbers
	let num1;
	let num2;
	// Replace first value with actual value
	//println!("{:?}", stack);
	let elem = stack.get(0);
	if elem.is_some() && elem.unwrap().datatype.clone().unwrap() == "IDENTIFIER".to_string() {
		let val = valuemap.get(&elem.unwrap().variable.clone());
		
		if val.is_none() {
			println!("TODO1")
		}
		let val = val.unwrap();
		
		num1 = *val;
	} else {
		num1 = elem.unwrap().variable.parse().unwrap();
	}
	stack.pop_front();
	//println!("{:?}", stack);
	
	// Replace second value with actual value
	let elem = stack.get(0);
	if elem.is_some() && elem.unwrap().datatype.clone().unwrap() == "IDENTIFIER".to_string() {
		let val = valuemap.get(&elem.unwrap().variable.clone());
		
		if val.is_none() {
			println!("{:?}", elem);
			println!("TODO2")
		}
		let val = val.unwrap();
		
		num2 = *val;
	} else {
		num2 = elem.unwrap().variable.parse().unwrap();
	}
	stack.pop_front();
	//println!("{:?}", stack);
	
	let mut result = 0;
	
	let elem = stack.get(0);
	match elem.unwrap().variable.clone().as_str() {
		"+" => {
			result = num2 + num1;
		}
		"-" => {
			result = num2 - num1;
		}
		"*" => {
			result = num2 * num1;
		}
		"/" => {
			result = num2 / num1;
		}
		_ => ()
	}
	
	stack.pop_front();
	stack.push_front(Data { variable: result.to_string(), datatype: Some("NUMBER".to_string()) });
}

fn handle_setter(stack: &mut VecDeque<Data>, valuemap: &mut HashMap<String, i32>) {
	let identifier = stack.get(1).unwrap().clone().variable;
	
	let value;
	
	// Get value of value
	if stack.get(0).unwrap().datatype.clone().unwrap() == "IDENTIFIER" {
		let value1 = valuemap.get(&stack.get(0).unwrap().variable);
		
		// Return invalid some
		if value1.is_none() {
			println!("TODO")
		}
		
		value = *value1.unwrap();
	} else {
		value = stack.get(0).unwrap().variable.parse().unwrap();
	}
	
	valuemap.entry(identifier).and_modify(|val| *val = value).or_insert(value);
}