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
			
			/* Start Evaluation */
			
			stack.push_front(symbol.clone());
			
			/* Evaluate Left */
			
			eval_tree(left, stack, valuemap);
			
			// Check for error
			if stack.get(0).is_some() && stack.get(0).unwrap().variable == "Error".to_string() {
				return
			}
			
			/* Result of left side */
			
			{ // Do Math Stuff if math stuff applies
				if 
					(stack.get(0).is_some() && stack.get(0).unwrap().datatype.clone().is_some() && (stack.get(0).unwrap().datatype.clone().unwrap() == "IDENTIFIER" || stack.get(0).unwrap().datatype.clone().unwrap() == "NUMBER")) && 
					(stack.get(1).is_some() && stack.get(1).unwrap().datatype.clone().is_some() && (stack.get(1).unwrap().datatype.clone().unwrap() == "IDENTIFIER" || stack.get(1).unwrap().datatype.clone().unwrap() == "NUMBER")) && 
					(symbol.variable == "+" || symbol.variable == "-" || symbol.variable == "*" || symbol.variable == "/") {
					handle_math(stack, valuemap);
				}
			}
			
			/* Evaluate Right */
			eval_tree(right, stack, valuemap);
			
			// Check for error
			if stack.get(0).is_some() && stack.get(0).unwrap().variable == "Error".to_string() {
				return
			}
			
			{ // Do Math Stuff
				if (stack.get(0).is_some() && stack.get(0).unwrap().datatype.clone().is_some() && (stack.get(0).unwrap().datatype.clone().unwrap() == "IDENTIFIER" || stack.get(0).unwrap().datatype.clone().unwrap() == "NUMBER")) && (stack.get(1).is_some() && stack.get(1).unwrap().datatype.clone().is_some() && (stack.get(1).unwrap().datatype.clone().unwrap() == "IDENTIFIER" || stack.get(1).unwrap().datatype.clone().unwrap() == "NUMBER")) && (symbol.variable == "+" || symbol.variable == "-" || symbol.variable == "*" || symbol.variable == "/") {
					handle_math(stack, valuemap);
				}
			}
			
			/* Evaluate Setters */
			
			if symbol.variable == ":=".to_string() {
				handle_setter(stack, valuemap)
			}
		}
		Parser::IfValue(ifcondition, ifstatement, elsestatement) => {
			let mut ifstack: VecDeque<Data> = VecDeque::new();
			
			// Evaluate if condition
			eval_tree(&ifcondition, &mut ifstack, valuemap);
			
			// Run whichever tree corresponds to condition
			if ifstack.pop_front().unwrap().variable == String::from("0") {
				// If condition is met
				eval_tree(ifstatement, stack, valuemap)
			} else {
				// If condition is not met (else is enacted)
				eval_tree(elsestatement, stack, valuemap)
			}
		}
		Parser::Value(terminal) => {
			if stack.get(0).is_some() && stack.get(0).unwrap().variable == "Error".to_string() {
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
	
	if loop_stack.get(0).is_some() && loop_stack.get(0).unwrap().datatype.is_some() && loop_stack.get(0).unwrap().clone().datatype.unwrap() == "IDENTIFIER".to_string() {
		let value = valuemap.get(&loop_stack.get(0).unwrap().clone().variable).unwrap();
		loop_stack.pop_front().unwrap();
		
		let data = Data {
			variable: value.to_string(),
			datatype: Some("NUMBER".to_string())
		};
		
		loop_stack.push_front(data);
	}
	
	while loop_stack.get(0).is_some() && loop_stack.get(0).unwrap().variable.parse::<i32>().unwrap() != 0 
		{
			// Eval while statement tree
			let mut result_queue = VecDeque::new();
			eval_tree(whilestatement, &mut result_queue, valuemap);
			
			if result_queue.get(0).is_some() && result_queue.get(0).unwrap().variable == "SKIP".to_string() {
				break
			}
			
			// Test for looping condition
			loop_stack.clear();
			eval_tree(checkstatement, &mut loop_stack, &mut valuemap.clone());
			
			if loop_stack.get(0).is_some() && loop_stack.get(0).unwrap().datatype.is_some() && loop_stack.get(0).unwrap().clone().datatype.unwrap() == "IDENTIFIER".to_string() {
				let value = valuemap.get(&loop_stack.get(0).unwrap().clone().variable).unwrap();
				loop_stack.pop_front().unwrap();
				
				let data = Data {
					variable: value.to_string(),
					datatype: Some("NUMBER".to_string())
				};
				
				loop_stack.push_front(data);
			}
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
			print!("
main::evaluator::handle_math
Code Portion: Evaluate First Value
Stack : {:?}
ValMap: {:?}", 
					stack, valuemap);
			// If val is not found in expression, error out
			stack.clear();
			let error = Data {
				variable: "Error".to_string(),
				datatype: None
			};
			stack.push_front(error);
			return
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
			print!("
main::evaluator::handle_math
Code Portion: Evaluate Second Value
Stack : {:?}
ValMap: {:?}", 
					stack, valuemap);
			// If val is not found in expression, error out
			stack.clear();
			let error = Data {
				variable: "Error".to_string(),
				datatype: None
			};
			stack.push_front(error);
			return
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
	let identifier = {
		let identifier = stack.get(1);
		
		if identifier.is_none() {
			print!("
main::evaluator::handle_setter
Code Portion: Grab Identifier from stack
Stack : {:?}
ValMap: {:?}", 
					stack, valuemap);
			
			stack.clear();
			let error = Data {
				variable: "Error".to_string(),
				datatype: None
			};
			stack.push_front(error);
			return
		}
		identifier.unwrap().clone().variable
	};
	
	let value;
	
	// Get value of value
	if stack.get(0).unwrap().datatype.clone().unwrap() == "IDENTIFIER" {
		let value1 = valuemap.get(&stack.get(0).unwrap().variable);
		
		// Return invalid some
		if value1.is_none() {
			print!("
main::evaluator::handle_setter
Code Portion: Grab value of identifier
Stack : {:?}
ValMap: {:?}", 
					stack, valuemap);
			
			stack.clear();
			let error = Data {
				variable: "Error".to_string(),
				datatype: None
			};
			stack.push_front(error);
			return
		}	
		
		value = *value1.unwrap();
	} else {
		value = stack.get(0).unwrap().variable.parse().unwrap();
	}
	
	valuemap.entry(identifier).and_modify(|val| *val = value).or_insert(value);
}