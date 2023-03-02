use regex::Regex;

pub fn parse_token(input: String) -> Vec<String> {
    let collect = input.split([' ']).collect();
    let tokens: Vec<&str> = collect;
	let tokens: Vec<String> = tokens.iter().map(|&x| x.into()).collect();
	
	return tokens;
}

pub fn match_num_or_identifier(input: &mut String, output: &mut String) -> (bool, Vec<String>) {
	let mut symbol_vec: Vec<String> = Vec::new();
	// Keep looping until scanning fails. If fail, return true
	loop {
		// Trim input each time remove training white spaces on both ends
		*input = input.trim().to_string();
		
		let found_keyword = scan_keyword(input);
		
		if found_keyword != "" {
			symbol_vec.push(found_keyword.clone());
			*output = format!("{}\n{} : KEYWORD", *output, found_keyword);
			continue;
		}
		
		let found_num = scan_num(input);
		if found_num != "" {
			symbol_vec.push(found_num.clone());
			*output = format!("{}\n{} : NUMBER", *output, found_num);
			continue;
		}
		
		let found_identifier = scan_identifier(input);
		if found_identifier != "" {
			symbol_vec.push(found_identifier.clone());
			*output = format!("{}\n{} : IDENTIFIER", *output, found_identifier);
			continue;
		}
		
		let found_symbol = scan_symbol(input);
		if found_symbol != "" {
			symbol_vec.push(found_symbol.clone());
			*output = format!("{}\n{} : SYMBOL", *output, found_symbol);
			continue;
		}
		
		if input != "" {
			*output = format!("{}\n{} : ERROR READING INPUT", *output, input.chars().next().unwrap());
			return (false, symbol_vec);
		} else {
			return (true, symbol_vec)
		}
	}
}

pub fn parse_regex(input: &mut String, regex: Regex) -> String {
	let capture = regex.find(input);
	if capture.is_none() {
		return "".to_string();
	}
	// Generate split capture
	let split_capture = input.split_at(capture.unwrap().end());
	let found_string = String::from(split_capture.0);
	
	// Modify input to contain leftover string
	*input = split_capture.1.clone().to_string();
	
	// Return string found if any
	return found_string;
}

pub fn scan_num(input: &mut String) -> String {
	let regex = Regex::new(r"^[0-9]+").unwrap();
	
	return parse_regex(input, regex);
}

pub fn scan_identifier(input: &mut String) -> String {
	let regex = Regex::new(r"^([a-z]|[A-Z])([a-z]|[A-Z]|[0-9])*").unwrap();
	
	return parse_regex(input, regex);
}

pub fn scan_symbol(input: &mut String) -> String {
	let regex = Regex::new(format!(r"^(\{}|\{}|\{}|{}|\{}|\{}|{}|{})", 
								   "+", "-", "*", "/", "(", ")", ":=", ";"
								   ).as_str()).unwrap();
	return parse_regex(input, regex);
}

pub fn scan_keyword(input: &mut String) -> String {
	let regex = Regex::new(r"^(if|then|else|endif|while|do|endwhile|skip)$").unwrap();
	
	return parse_regex(input, regex);
}