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
    let mut format_str = Vec::<&str>::new();

    if show_chars {
        let char_count = text.as_bytes().len();
        result.push(char_count.to_string().as_str());
        format_str.push("{}");
    }

    if show_lines {
        let line_count = text.lines().count();
        result.push(line_count.to_string().as_str());
        format_str.push("{}");
    }


    println!("{} {}", char_count, &args.filepath);

}
