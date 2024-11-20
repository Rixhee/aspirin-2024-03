use anyhow::{Context, Result};
use clap::Parser;
use serde_json::Value;
use std::{
    env,
    io::{stdout, BufReader},
    num::ParseIntError,
    path::PathBuf,
};
use thiserror::Error;

mod filters;
use filters::pipe;

mod functions;
mod output;
use output::print_result;

#[derive(Error, Debug)]
enum MyErrors {
    #[error("The indent value must be in the range of 0-7 inclusive. Current value: {0}")]
    InvalidIndent(usize),

    #[error("You can have either colored or monochrome output")]
    ColoredAndMonochromeError,

    #[error("You can have either compact or indented output")]
    CompactAndIndentedError,

    #[error("Failed to read the provided JSON file: {0}")]
    JSONError(#[from] serde_json::Error),

    #[error("You need an integer")]
    ParseError(#[from] ParseIntError),
}

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, default_value_t = true)]
    color_output: bool,

    #[clap(short, long)]
    monochrome_output: bool,

    #[clap(short, long)]
    sort_keys: bool,

    #[clap(short, long, default_value_t = 2)]
    indent: usize,

    #[clap(short = 'o', long)]
    compact_output: bool,

    #[clap(required = true)]
    needle: String,

    #[clap(required = true)]
    file: PathBuf,
}

pub fn file_path(path: PathBuf) -> Result<Value> {
    let file =
        std::fs::File::open(&path).with_context(|| format!("Failed to open file: {:?}", path))?;
    let reader = BufReader::new(file);
    let json_value: Value = serde_json::from_reader(reader)?;

    Ok(json_value)
}

fn main() -> Result<()> {
    let args = Args::parse();

    let jq_colors = env::var("JQ_COLORS").unwrap_or_else(|_| {
        if !args.color_output || args.monochrome_output {
            "0;0:0;0:0;0:0;0:0;0:0;0:0;0:0;0".to_string()
        } else {
            "0;90:0;37:0;37:0;37:0;32:1;37:1;37:1;34".to_string()
        }
    });

    // Validate color and indent options
    let color_output_provided = std::env::args().any(|arg| arg == "--color-output" || arg == "-c");
    let indent_provided = std::env::args().any(|arg| arg == "--indent" || arg == "-i");

    if color_output_provided && args.monochrome_output {
        return Err(MyErrors::ColoredAndMonochromeError.into());
    }

    if indent_provided && args.indent > 0 && args.compact_output {
        return Err(MyErrors::CompactAndIndentedError.into());
    }

    if !(0..=7).contains(&args.indent) {
        return Err(MyErrors::InvalidIndent(args.indent).into());
    }

    let json_input = file_path(args.file)?;
    let filtered_input = pipe(&json_input, &args.needle)?;

    let mut writer = stdout();

    let _ = print_result(
        filtered_input,
        &jq_colors,
        &args.sort_keys,
        &args.indent,
        &args.compact_output,
        &mut writer,
    );

    Ok(())
}
