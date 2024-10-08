use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyErrors {
    #[error("The indent value must be in the range of 0-7 inclusive. Current value: {0}")]
    InvalidIndent(i8),

    #[error("You can have either colored or monochrome output")]
    ColoredAndMonochromeError,

    #[error("You can have either compact or indented output")]
    CompactAndIndentedError,
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
    indent: i8,

    #[clap(short = 'o', long)]
    compact_output: bool,

    needle: String,

    file: PathBuf,
}

fn main() -> Result<(), MyErrors> {
    let args = Args::parse();

    if args.color_output && args.monochrome_output {
        return Err(MyErrors::ColoredAndMonochromeError);
    } else if args.indent > 0 && args.compact_output {
        return Err(MyErrors::CompactAndIndentedError);
    } else if args.indent > 7 || args.indent < 0 {
        return Err(MyErrors::InvalidIndent(args.indent));
    }

    println!("{:?}", args);

    Ok(())
}
