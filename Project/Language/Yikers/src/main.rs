mod scanner;
mod parser;
mod evaluator;

use std::io::BufRead;
use std::collections::VecDeque;
use parser::Parser;

fn main() {
    let mut args = std::env::args();
    if args.len() != 3 {
        println!("Usage: Scanner INPUT_FILE OUTPUT_FILE");
        return;
    }
    let in_file = std::fs::File::open(args.nth(1).expect("No input file specified")).expect("Could not find file");
    
    let infile_lines = std::io::BufReader::new(in_file).lines();
    
    let mut output = String::new();
    

    
    for line in infile_lines {
        let line = line.unwrap();
        if line.replace(" ", "").is_empty() {
            continue;
        }
        
        // Scanner
        let mut scanner_results = String::new();
        scanner_results.push_str(&format!("Line: {}", line));
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
        output = format!("{}{}\n\n", output, scanner_results);
        if !valid {
            break;
        }
        
        // Regenerate Tokens Properly
        input_tokens = tmp_tokens;
        
        // Parser
        let mut parser_results = String::new();
        let tree = parser::parse_tokens(&mut input_tokens);
        parser::print_tree(&tree, &mut parser_results, &mut 0);
        
        if let Parser::PlaceHolder = tree {
            valid = false;
        }
        
        output = format!("{}Parser:\n\n{}\n\n", output, parser_results);
        if !valid {
            break;
        }
        
        let mut eval_stack = VecDeque::new();
        evaluator::evaluate_tree(&tree, &mut eval_stack);
        output = format!("{}Output: {}\n\n", output, eval_stack.get(0).unwrap().variable);
    }
    
    write_output(output, args.nth(0).expect("No output file detected"));
}

fn write_output(output: String, path: String) {
    std::fs::write(std::path::Path::new(&path), output).unwrap();
}