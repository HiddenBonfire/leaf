use std::env;
use std::fs;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    processing_arguments(&args);
}

fn processing_arguments(args:& Vec<String>) {
    let mut prev_was_p = false;
    let mut contents: Result<String, io::Error> = Ok(String::from("NONE"));
    for arg in args.iter() {
        if prev_was_p {
           contents = fs::read_to_string(arg); 
           prev_was_p = false;
        }

        match arg.as_str() {
            "p" => prev_was_p = true,
            _ => (),
            
        }
    }
    if let Ok(c) = contents {
        println!("{}", c);
    } else {
        println!("File not found.");
    }
}
