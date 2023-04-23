mod lib;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = Path::new("~/.config/xsay/asciiart/tux.txt");
    let file = &args[1];
    let text = lib::maketext(&args[2..]);
    // let path_abs = path + &file + ".txt";
    // println!("{}", path_abs);
    let asciiart = fs::read_to_string(path).expect("Should have been able to read file");

    println!("{}", lib::bubble(&text));
    println!("{}", asciiart);
}
