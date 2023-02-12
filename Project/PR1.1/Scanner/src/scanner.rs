use regex::Regex;

pub fn parse_token(input: String) -> Vec<String> {
    let collect = input.split(' ').collect();
    let tokens: Vec<&str> = collect;
	let tokens: Vec<String> = tokens.iter().map(|&x| x.into()).collect();
	return tokens;
}

pub fn match_num_or_identifier(input: &mut String, output: &mut String) -> bool {
	let found_num = scan_num(input);
	if found_num != "" {
		*output = format!("{}\n{}: NUMBER", output, found_num);
	}
	
	let found_identifier = scan_identifier(input);
	if found_identifier != "" {
		*output = format!("{}\n{} : IDENTIFIER", output, found_identifier);
	}
	
	let found_symbol = scan_symbol(input);
	if found_symbol != "" {
		*output = format!("{}\n{} : SYMBOL", output, found_symbol);
	}
	
	if input != "" {
		*output = format!("{}\n{} : ERROR", output, input);
		return false;
	}
	
	
	return true;
}

fn parse_regex(input: &mut String, regex: Regex) -> String {
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

fn scan_num(input: &mut String) -> String {
	let re_num = Regex::new(r"^[0-9]+").unwrap();
	
	return parse_regex(input, re_num);
}

fn scan_identifier(input: &mut String) -> String {
	let re_identifier = Regex::new(r"^([a-z]|[A-Z])([a-z]|[A-Z]|[0-9])*$").unwrap();
	
	return parse_regex(input, re_identifier);
}

fn scan_symbol(input: &mut String) -> String {
	let re_symbol = Regex::new(r"\+|\-|\*|/|\(|\)").unwrap();
	
	return parse_regex(input, re_symbol);
}