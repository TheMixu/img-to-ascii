use std::{fmt::Error, path::PathBuf, str::FromStr};

use clap::Parser;
use image::{
    imageops::FilterType::Nearest, io::Reader as ImageReader, DynamicImage, GenericImageView,
};
type ArgMode = Mode;
/// Simple program to greet a person
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap()]
    filename: PathBuf,

    #[clap(short, long)]
    /// Scaling percent as a integer
    scale: Option<u32>,

    #[clap(short, long)]
    /// Width of the output ASCII in characters
    width: Option<u32>,

    #[clap(short, long)]
    /// Modes are lightness, average, luminosity
    mode: Option<ArgMode>,
}
enum Mode {
    Average,
    Lightess,
    Luminosity,
}
impl FromStr for Mode {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "average" => Ok(Mode::Average),
            "lightness" => Ok(Mode::Lightess),
            "luminosity" => Ok(Mode::Luminosity),
            _ => Err(Error),
        }
    }
}

fn main() {
    let (image, mode) = parse_args(Args::parse());
    turn_to_ascii(image, mode)
}
fn parse_args(args: Args) -> (DynamicImage, Mode) {
    let image = ImageReader::open(args.filename);

    if let Err(e) = image {
        eprintln!("{}", e);
        std::process::exit(0)
    }
    let image = image.unwrap().decode().unwrap();
    let scale = if let Some(scale) = args.scale {
        scale as f32 / 100.0
    } else {
        1.0
    };
    let width = image.width();
    let height = image.height();
    let aspect_ratio = (height as f32 / width as f32).ceil();
    let new_width = if let Some(w) = args.width { w } else { 60 };
    let new_height = aspect_ratio * new_width as f32 * scale;
    let image = image.resize(new_width, new_height as u32, Nearest);
    let mode = if let Some(mode) = args.mode {
        mode
    } else {
        Mode::Luminosity
    };
    (image, mode)
}

fn turn_to_ascii(image: DynamicImage, mode: Mode) {
    let pixels = image.pixels();
    let mut string = String::with_capacity(image.width() as usize);
    let width = image.width();
    for (x, _, rgba) in pixels {
        let [r, g, b, _] = rgba.0;
        let luma = match mode {
            Mode::Average => get_average(r, g, b),
            Mode::Lightess => get_lightness(r, g, b),
            Mode::Luminosity => get_luminosity(r, g, b),
        };
        let ascii = get_ascii(luma);
        string.push(ascii);
        if x == width - 1 {
            string.push('\n');
        }
    }
    terminal_clipboard::set_string(&string).unwrap();
    println!("{string}")
}

fn get_average(r: u8, g: u8, b: u8) -> u8 {
    ((r as u32 + g as u32 + b as u32) / 3) as u8
}

fn min_max(list: Vec<u8>) -> (u8, u8) {
    let mut min = u8::MAX;
    let mut max = u8::MIN;
    for item in list {
        if item > max {
            max = item;
        } else if item < min {
            min = item;
        }
    }
    (min, max)
}

fn get_lightness(r: u8, g: u8, b: u8) -> u8 {
    let (min, max) = min_max(vec![r, g, b]);
    let res = (min as u32 + max as u32) / 2;
    res as u8
}

fn get_luminosity(r: u8, g: u8, b: u8) -> u8 {
    (0.21 * r as f32 + 0.72 * g as f32 + 0.07 * b as f32) as u8
}

fn get_ascii(num: u8) -> char {
    let letters = " .\"`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
    let index = ((letters.len() - 1) * num as usize) / 255;
    letters.as_bytes()[index].into()
}
