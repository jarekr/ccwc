use clap::Parser;
use colored::*;
use std::fs;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    count: bool,
    #[arg(short, long, default_value_t = false)]
    lines: bool,
    filepath: String
}


fn main() {
    let args = Args::parse();

    let text = match fs::read_to_string(&args.filepath) {
        Ok(txt) => txt,
        Err(e) => panic!("couldn't open file: {}", e)
    };

    let any_flags = args.lines || args.count;

    let mut show_lines = true;
    let mut show_chars = true;

    if any_flags {
        show_lines = args.lines;
        show_chars = args.count;
    }

    let mut result = Vec::<&str>::new();
    let mut output = String::new();

    if show_chars {
        let char_count = text.as_bytes().len();
        let char_count_str = format!(" {:>7}", char_count.to_string());
        output.push_str(&char_count_str);

    }

    if show_lines {
        let line_count = text.lines().count();
        let line_count_str = format!(" {:>7}", line_count.to_string().as_str());
        output.push_str(&line_count_str);
    }

    println!("{} {}", output, &args.filepath);

}
