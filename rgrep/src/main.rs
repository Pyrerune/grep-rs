use std::io::{self, Read};
use std::env::args;
use std::process::exit;
use std::fs::File;
use librgrep::Options;
use librgrep::Searcher;

fn process_args(o: Vec<String>, file_arg: usize, buf: &mut String, args: Vec<String>) -> Options {
    let mut options = Options::default();
    let help_dialog = "RGREP: 0.1.0\nUsage: rgrep [options] pattern [file]\nDescription: A tool to find lines that contain the given pattern\nOptions:\n\n--help, -h: Prints this Message\n--exclude, -e: Displays all lines that do not contain the pattern\n--include-before, -I: Displays all lines before the line including the pattern\n--include-after, -i: Displays all lines after the line including the pattern";
    
    for i in o {
        println!("{}", i);
        match i.as_str() {
            "--help" | "-h" => {
                println!("{}", help_dialog);
                exit(0);
            }
            "--exclude" | "-e" => {
                options.exclude = true;
            }
            "--include-before" | "-I" => {
                options.include_before = true;
            }
            "--include-after" | "-i" => {
                options.include_after = true;
            }
            _ => {}
        }
    }
    
    if args.len() > file_arg && !args[file_arg].is_empty() {
        let file = File::open(args[file_arg].clone());
        if file.is_ok() {
            let _ = file.unwrap().read_to_string(buf);
        }
    }
    options
}
fn main() -> io::Result<()>{
    let args: Vec<String> = args().collect();
    let mut options: Vec<String> = vec![];
    let mut buf = String::new();

    let mut pattern_arg = 1;
    let mut file_arg = 2;
    while args[pattern_arg].starts_with("--") || args[pattern_arg].starts_with("-") {
        options.push(args[pattern_arg].clone());
        pattern_arg+=1;
        file_arg+=1;
    }
    let options = process_args(options,file_arg, &mut buf, args.clone());


    if buf.is_empty() {
        let _  = io::stdin().read_to_string(&mut buf);
    }
    let searcher = Searcher::new(args[pattern_arg].clone(), buf.clone(), options.clone());
    let output = searcher.search();
    println!("{}", output);
    //let list: Vec<&str> = buf.split("\n").collect();
   // search_input(pattern_arg, list, args.clone(), options)
   Ok(())
}
