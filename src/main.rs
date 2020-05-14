use std::io::{self, Read};
use std::env::args;
use std::process::exit;
use std::fs::File;
struct Options {
    exclude: bool
}
impl Default for Options {

    fn default() -> Options {
        Options {
            exclude: false
        }
    }
}
impl Options {
    fn new(e: bool) -> Options {
        Options {
            exclude: e,
        }
    }
}
fn search_input(pattern: usize, list: Vec<&str>, args: Vec<String>, options: Options) -> io::Result<()> {

    for i in list {
        if options.exclude == false {
            if i.contains(args[pattern].as_str()) {
                println!("{}", i);
            }
        } else {
            if !i.contains(args[pattern].as_str()) {
                println!("{}", i);
            }
        }
    }
    Ok(())
}
fn process_args(option_arg: usize, file_arg: usize, buf: &mut String, args: Vec<String>) -> Options {
    let mut options = Options::default();
    let help_dialog = "RGREP: 0.1.0\nUsage: rgrep [options] pattern [file]\nDescription: A tool to find lines that contain the given pattern\nOptions:\n\n--help, -h: Prints this Message\n--exclude, -e: Displays all lines that do not contain the pattern";
    let mut option = 0;
    while option <= option_arg {
        match args[option].as_str() {
            "--help" | "-h" => {
                println!("{}", help_dialog);
                exit(0);
            }
            "--exclude" | "-e" => {
                options.exclude = true;
            }
            _ => {}
        }
        option+=1;
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
    let mut buf = String::new();

    let mut option_arg =  0;
    let mut pattern_arg = 1;
    let mut file_arg = 2;
    if args[pattern_arg].starts_with("--") || args[pattern_arg].starts_with("-") {
        option_arg+=1;
        pattern_arg+=1;
        file_arg+=1;
    }
    let options = process_args(option_arg,file_arg, &mut buf, args.clone());


    if buf.is_empty() {
        let _  = io::stdin().read_to_string(&mut buf);
    }
    let list: Vec<&str> = buf.split("\n").collect();
    search_input(pattern_arg, list, args.clone(), options)
}
