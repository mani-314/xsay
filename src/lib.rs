use textwrap::fill;
use unicode_width::UnicodeWidthStr;

pub fn maketext(args: &[String]) -> String {
    let mut text = String::from("");
    for arg in args.into_iter() {
        text = text + arg + " ";
    }
    return text;
}

pub fn bubble(input: &String) -> String {
    const DASHES: &str = "\n   \\ \n    \\";
    const LINE_LIMIT: usize = 50;

    let wrapped = fill(input, LINE_LIMIT);
    let lines: Vec<&str> = wrapped.lines().collect();

    let num_lines = lines.len();
    let line_width = widest_line(&lines);

    //Top Border
    let mut out = String::from(" ");
    out = out + &("—".repeat(line_width + 2)) + " \n";

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

    //Bottom Border
    out = out + " " + &("—".repeat(line_width + 2));
    //Line to bubble
    out = out + DASHES;

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
