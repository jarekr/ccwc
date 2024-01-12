use clap::Parser;
use colored::*;
use std::fs;
use std::fmt;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Parser)]
struct Args {

    #[arg(short, long, default_value_t = false)]
    count: bool,

    #[arg(short, long, default_value_t = false)]
    lines: bool,

    #[arg(short, long, default_value_t = false)]
    words: bool,

    filepath: String
}


fn main() {
    let args = Args::parse();

    let text = match fs::read_to_string(&args.filepath) {
        Ok(txt) => txt,
        Err(e) => panic!("couldn't open file: {}", e)
    };

    let any_flags = args.lines || args.count || args.words;
    let flagsum = if args.lines { 1 } else {0}
                        + if args.count {1} else {0}
                        + if args.words {1} else {0};

    let mut show_lines = true;
    let mut show_chars = true;
    let mut show_words: bool = true;

    if flagsum > 0 {
        show_lines = args.lines;
        show_chars = args.count;
        show_words = args.words;
    }

    let mut output = String::new();
    let char_count = text.as_bytes().len();
    let char_count_str = char_count.to_string();

    let padding = if flagsum == 1 { 0 } else { char_count_str.len() };

    if show_lines {
        let line_count = text.lines().count();
        let line_count_str = format!("{:>padding$}", line_count.to_string().as_str());
        output.push_str(&line_count_str);
    }

    if show_words {
        let mut word_count: usize = 0;
        for line in text.lines() {
            word_count += line.split_whitespace().count();
        }

        let wc_str = format!("{:>padding$}", word_count.to_string().as_str());
        output.push_str(&wc_str);
    }

    if show_chars {
        let chars_str = format!("{:>pd$}", char_count_str, pd = padding + 1);
        output.push_str(&chars_str);

    }

    println!("{} {}", output, &args.filepath);

}
