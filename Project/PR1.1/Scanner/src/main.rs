mod scanner;
use std::io::BufRead;


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
        output.push_str(&format!("Line: {}", line));
        let tokens = scanner::parse_token(line);
        for mut token in tokens {
            if !scanner::match_num_or_identifier(&mut token, &mut output) {
                break;
            }
        }
        output = format!("{}\n\n", output);
    }
    
    write_output(output, args.nth(0).expect("No output file detected"));
}

fn write_output(output: String, path: String) {
    std::fs::write(std::path::Path::new(&path), output).unwrap();
}