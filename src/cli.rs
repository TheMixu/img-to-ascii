use std::path::PathBuf;

use clap::{clap_derive::ArgEnum, Parser};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Options {
    #[clap()]
    pub filename: PathBuf,

    #[clap(long)]
    /// Scaling percent as a integer
    pub scale: Option<u32>,

    #[clap(short, long)]
    /// Width of the output ASCII in characters
    pub width: Option<u32>,

    #[clap(short, long)]
    /// Height of the output ASCII in characters
    pub height: Option<u32>,

    #[clap(short, long, arg_enum)]
    /// Mode to use to determine character
    pub mode: Option<Mode>,

    #[clap(short, long, action)]
    pub grayscale: bool,

    #[clap(short, long, action)]
    /// Keep original image dimensions
    pub original: bool,

    #[clap(short, long, action)]
    /// Copy result to clipboard
    pub copy: bool,

    #[clap(long, action)]
    /// Don't print the result
    pub silent: bool,

    #[clap(long, action)]
    /// Save to file
    pub save_to_file: Option<PathBuf>,
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Mode {
    Average,
    Lightess,
    Luminosity,
}

impl Options {
    pub fn new() -> Self {
        Self::parse()
    }
}
