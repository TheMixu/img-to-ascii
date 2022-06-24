use image::DynamicImage;
use image::{imageops::FilterType::Nearest, io::Reader};
use std::error::Error;

use crate::cli::{Mode, Options};

pub fn parse_args(args: &Options) -> Result<(DynamicImage, Mode), Box<dyn Error>> {
    let path = args.filename.as_path();
    let image = Reader::open(path);
    let mut image = image?.decode()?;
    if args.grayscale {
        image = image.grayscale()
    };
    // Only modifying dimensions if not keeping original image scale
    if !args.original {
        let scale = if let Some(scale) = args.scale {
            scale as f32 / 100.0
        } else {
            1.0
        };
        let aspect_ratio = (image.width() as f32 / image.height() as f32).ceil();
        let new_width = if let Some(w) = args.width { w } else { 120 };
        let new_height = if let Some(h) = args.height {
            h
        } else {
            (aspect_ratio * new_width as f32 * scale) as u32
        };
        image = image.resize(new_width, new_height as u32, Nearest);
    }
    let mode = if let Some(mode) = args.mode {
        mode
    } else {
        Mode::Average
    };
    println!("Width: {} Height: {}", image.width(), image.height());
    Ok((image, mode))
}
