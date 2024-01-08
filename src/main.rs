mod lib;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 2 || args[1].eq("-h") {
        println!(
            "Usage: 'xsay <asciiart-file> your text here'.\nAsciiart files must be located at '$HOME/.config/xsay/asciiart/<file>'"
        );
        process::exit(0);
    }

    let file = &args[1];

    if let Some(home_dir) = env::var_os("HOME") {
        let path = PathBuf::from(home_dir);
        let config_path = path.join(".config/xsay/asciiart");
        let asciiart_path = config_path.join(file);
        let text = lib::maketext(&args[2..]);
        match fs::read_to_string(asciiart_path) {
            Ok(asciiart) => {
                println!("{}", lib::bubble(&text));
                println!("{}", asciiart);
            }
            Err(err) => {
                eprintln!("Error reading file: {} \n asciiart files must be located at '$HOME/.config/xsay/asciiart/<file>'", err);
                process::exit(1);
            }
        }
    } else {
        eprintln!("Path to home directory not found");
        process::exit(1);
    }
}
