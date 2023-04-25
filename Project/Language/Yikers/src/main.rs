mod scanner;
mod parser;
mod evaluator;

use std::io::BufRead;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    let mut args = std::env::args();
    if args.len() != 3 {
        println!("Usage: Scanner INPUT_FILE OUTPUT_FILE");
        return;
    }
    let in_file = std::fs::File::open(args.nth(1).expect("No input file specified")).expect("Could not find file");
    
    let infile_lines = std::io::BufReader::new(in_file).lines();
    
    let mut output = String::new();
    
    let mut full_input_tokens = Vec::new();
    
    // Scanner and Token list
    output = format!("{}~~~Scanner~~~\n\n", output);
    for line in infile_lines {
        let line = line.unwrap();
        if line.replace(" ", "").is_empty() {
            continue;
        }
        
        let mut scanner_results = String::new();
        scanner_results.push_str(&format!("Line: {}", line.trim()));
        let mut valid = true;
        let mut tmp_tokens = Vec::new();
        let mut input_tokens = scanner::parse_token(line);
        for mut token in input_tokens.clone() {
            let scan = scanner::match_num_or_identifier(&mut token, &mut scanner_results);
            if !scan.0 {
                valid = false;
                break;
            }
            
            for token in scan.1 {
                tmp_tokens.push(token);
            }
            
        }
        
        if !valid {
            output = format!("{}{}\n", output, scanner_results);
            break;
        }
        
        // Regenerate Tokens Properly
        input_tokens = tmp_tokens;
        full_input_tokens.append(&mut input_tokens);
        
        // Write Scanner Results to output
        output = format!("{}{}\n\n", output, scanner_results);
    }
    
    // Parser
    let mut parser_results = String::new();
    let mut tree = parser::parse_tokens(&mut full_input_tokens);
    if !full_input_tokens.is_empty() {
        tree = parser::Parser::PlaceHolder;
    }
    parser::printTree(&tree, &mut parser_results, &mut 0);
    
    
    output = format!("{}~~~Parser~~~\n\n{}\n", output, parser_results);
    
    // Evaluator
    let mut value_map: HashMap<String, i32> = HashMap::new();
    let mut value_stack: VecDeque<parser::Data> = VecDeque::new();
    
    evaluator::eval_tree(&tree, &mut value_stack, &mut value_map);
    
    output = format!("{}~~~Evaluator~~~\n\nOutput:\n", output);
    
    for value in value_map {
        output = format!("{}{}: {}\n", output, value.0, value.1)
    }
    
    // Write Output
    write_output(output, args.nth(0).expect("No output file detected"));
}

fn write_output(output: String, path: String) {
    std::fs::write(std::path::Path::new(&path), output).unwrap();
}