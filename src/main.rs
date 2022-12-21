use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = "asciiart/";                     //planned to be changeable via "--setpath" flag
    let file = &args[1];
    let text = &args[2];
    let path_abs = path.to_owned() + &file + ".txt";

    let asciiart = fs::read_to_string(path_abs)
        .expect("Should have been able to read file");
    
    println!("{}",speechbubblecreator(text.to_string()));
    println!("{}",asciiart);
}

fn speechbubblecreator(input: String) -> String  {
    let out = input.to_owned() + "\n   \\ \n    \\";
    return out;
}