use std::error::Error;

use image::{DynamicImage, GenericImageView};

use crate::cli::{Mode, Options};

pub fn turn_to_ascii(image: DynamicImage, mode: Mode) -> String {
    let pixels = image.pixels();
    let mut result = String::with_capacity(image.width() as usize);
    let width = image.width();
    for (x, _, rgba) in pixels {
        let [r, g, b, _] = rgba.0;
        let luma = match mode {
            Mode::Average => get_average(r, g, b),
            Mode::Lightess => get_lightness(r, g, b),
            Mode::Luminosity => get_luminosity(r, g, b),
        };
        let ascii = get_ascii(luma);
        result.push(ascii);
        if x == width - 1 {
            result.push('\n');
        }
    }
    result
}

pub fn output_ascii(ascii: String, args: &Options) -> Result<(), Box<dyn Error>> {
    if let Some(path) = args.save_to_file.as_ref() {
        std::fs::write(path, &ascii)?;
    };
    if !args.silent {
        println!("{}", &ascii);
    }
    if args.copy {
        let res = terminal_clipboard::set_string(&ascii);
        if res.is_err() {
            return Err(Box::from("Could not copy text to clipboard."));
        }
    }
    Ok(())
}

fn get_average(r: u8, g: u8, b: u8) -> u8 {
    let sum: u32 = u32::from(r) + u32::from(g) + u32::from(b);
    u8::try_from(sum / 3).unwrap()
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
    u8::try_from(res).unwrap()
}

fn get_luminosity(r: u8, g: u8, b: u8) -> u8 {
    let r = 0.21 * f32::from(r);
    let g = 0.72 * f32::from(g);
    let b = 0.07 * f32::from(b);
    (r + g + b) as u8
}

fn get_ascii(num: u8) -> char {
    let letters = " .\"`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
    let index = ((letters.len() - 1) * num as usize) / 255;
    letters.as_bytes()[index].into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_max_test() {
        let res = min_max(vec![12, 230, 3]);
        assert_eq!(res, (3, 230));
    }
    #[test]
    fn luminosity() {
        let res = get_luminosity(102, 25, 45);
        assert_eq!(res, 42);
    }
    #[test]
    fn light() {
        let res = get_lightness(102, 25, 45);
        assert_eq!(res, 63);
    }
    #[test]
    fn avg() {
        let res = get_average(23, 192, 73);
        assert_eq!(res, 96);
    }
    #[test]
    fn ascii() {
        let res = get_ascii(255);
        assert_eq!(res, '$');
    }
    #[test]
    fn ascii_min() {
        let res = get_ascii(0);
        assert_eq!(res, ' ');
    }
}
