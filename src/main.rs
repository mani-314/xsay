use std::env;
use std::fs;
use textwrap::fill;
use unicode_width::UnicodeWidthStr;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = "asciiart/"; //planned to be changeable via "--setpath" flag
    let file = &args[1];
    let text = &args[2];
    let path_abs = path.to_owned() + &file + ".txt";

    let asciiart = fs::read_to_string(path_abs).expect("Should have been able to read file");

    println!("{}", bubble(text));
    println!("{}", asciiart);
}

fn bubble(input: &str) -> String {
    let dashes = "\n   \\ \n    \\";
    let line_limit = 50;

    let wrapped = fill(input, line_limit);
    let lines: Vec<&str> = wrapped.lines().collect();

    let num_lines = lines.len();
    let line_width = widest_line(&lines);

    //Top Border
    let mut out: String = " ".to_string();
    out = out + &("_".repeat(line_width + 2)) + " \n";

    //Text
    for (i, line) in lines.into_iter().enumerate() {
        let diff = line_width - UnicodeWidthStr::width(line);
        if num_lines == 1 {
            out = out + "< " + line + " >\n";
        } else if i == 0 {
            out = out + "/ " + line + &(" ".repeat(diff)) + " \\\n";
        } else if i == num_lines - 1 {
            out = out + "\\ " + line + &(" ".repeat(diff)) + " /\n";
        } else {
            out = out + "| " + line + &(" ".repeat(diff)) + " |\n";
        }
    }
    out = out + " " + &("-".repeat(line_width + 2));
    out = out + dashes;

    return out;
}

fn widest_line(lines: &[&str]) -> usize {
    let mut wideboy = UnicodeWidthStr::width(lines[0]);
    let mut line_width;
    for line in lines {
        line_width = UnicodeWidthStr::width(line.to_owned());
        if line_width > wideboy {
            wideboy = line_width;
        }
    }
    return wideboy;
}
