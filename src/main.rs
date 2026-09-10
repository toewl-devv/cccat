use std::env;
use std::fs;
use std::process;

fn main() {
    let mut number_lines = false;
    let mut skip_blanks = false;

    //let mut contents_vec: Vec<String> = vec![];
    let args: Vec<String> = env::args().collect();
    let arg_number = args.len();
    let file_path = &args[arg_number - 1];

    for arg in args.iter() {
        if arg == "-n" {
            number_lines = true;
        }
        else if arg == "-b" {
            skip_blanks = true;
        }
        /*
        else {
            // check if is a file
            // if so append file contents to the output
            contents_vec.push(
                    match fs::read_to_string(&arg) {
                        Ok(contents) => contents,
                        Err(_) => {
                            eprintln!("Could not open file: {}", file_path);
                            process::exit(1);
                        }
                    }
            );
        }
        */
    }

    let contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(_) => {
            eprintln!("Could not open file: {}", file_path);
            process::exit(1);
        }
    };

    let lines: Vec<String> = contents.split('\n').map(String::from).collect();

    let mut lines_printed = 0;
    for i in 0..lines.len() {
        if skip_blanks {
            if lines[i] != "" {
                lines_printed += 1;
                println!("{} {}", lines_printed, lines[i]);
            } else {
                println!("");
            }
        } else if number_lines {
            println!("{} {}", i+1, lines[i]);
        } else {
            println!("{}", lines[i]);
        }
    }

}
