use clap::Parser;
use colored::*;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Parser)]
struct Args {
    paths: Vec<PathBuf>,
}


fn main() {
    println!("Hello, world!");
}
