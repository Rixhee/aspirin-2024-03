use anyhow::Result;
use clap::Parser;
use colored::Color;
use std::io::stdout;
use std::path::PathBuf;

mod colored_output;
mod find_match;
mod input;

use colored_output::colored_output;
use find_match::filter_lines;
use input::get_lines_from_input;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    ignore_case: bool,

    #[clap(short = 'v', long)]
    invert_match: bool,

    #[clap(short, long)]
    regex: bool,

    #[clap(short, long)]
    color: Option<Color>,

    needle: String,

    file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("{:?}", args);

    let lines = get_lines_from_input(args.file)?;
    let filter_lines = filter_lines(args.needle, lines, args.ignore_case, args.invert_match)?;
    let _ = colored_output(filter_lines, &mut stdout(), args.color);

    Ok(())
}
