//! # libaidokuln
//! 
//! A WASM/no_std library for generating bitmap images from text. As the name implies, it
//! is geared toward usage in [Aidoku](https://aidoku.app), but can be used anywhere.
//! 
//! ## Usage
//! ```
//! use libaidokuln::{write_text, write_image_data, fonts, ImageOptions, Padding};
//! 
//! let mut data = write_text(
//!     include_str!("./lorem.txt"),
//!     fonts::times::TIMES36,
//!     ImageOptions {
//!         text_color: 0,
//!         background_color: 0xFFFFFF,
//!         padding: Padding(40.0, 40.0),
//!         width: 1080.0,
//!         constant_width: false,
//!     },
//! );
//! 
//! let img = write_image_data(&mut data);
//! ```
//! 
//! ## Caveats
//! * Does not support Unicode characters. Any characters between ASCII 32 and 126 will
//! be converted, and the rest will be spaces.
//! * Fonts need to be bundled with the final binary, and it can accumulate a lot of
//! binary data.
//! * Fonts follow a specific format. To generate a font, check the FontToJson.java file
//! in the `fonts` module.
//! 
//! ## Credits
//! * [JimIsWayTooEpic](https://github.com/phiefferj24) for creating the original TypeScript
//! library which this is based on.
#![no_std]
#![feature(test)]
#![allow(clippy::needless_range_loop)]
#[cfg(test)]
mod tests;

#[cfg(test)]
mod bench;

/// Module containing a few built-in fonts for text rendering.
#[cfg_attr(not(test), cfg(feature = "fonts"))]
pub mod fonts;
use fonts::Font;

extern crate alloc;
use alloc::{borrow::ToOwned, string::String, vec, vec::Vec};

const BMP_HEADER1: [u8; 2] = [0x42, 0x4D];
const BMP_HEADER2: [u8; 12] = [
    0x00, 0x00, 0x00, 0x00, 0x36, 0x00, 0x00, 0x00, 0x28, 0x00, 0x00, 0x00,
];
const BMP_HEADER3: [u8; 8] = [0x01, 0x00, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00];
const BMP_HEADER4: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

/// Struct representing text padding, used to tell the library to add margins
/// to the text.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Padding(
    /// The horizontal padding.
    pub f32, 

    /// The vertical padding.
    pub f32
);

/// Rendering options.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImageOptions {
    /// Text color. It is an usize, but can be written in an RGB-like format
    /// using hex notation, e.g., `0x1F1E33`.
    pub text_color: usize,

    /// Background color. Can be written in an RGB-like format simular to text
    /// color.
    pub background_color: usize,

    /// The margins for the generated page.
    pub padding: Padding,

    /// Maximum page width.
    pub width: f32,
    
    /// Whether the renderer should force the given max width or not.
    pub constant_width: bool,
}

/// Struct representing split text.
#[derive(Debug, Clone, PartialEq)]
pub struct Spliterated {
    /// The split text.
    pub split: Vec<String>,

    /// The maximum width of the split text.
    pub width: f32,
}

fn ceil(num: f32) -> f32 {
    (num as i32 + 1) as f32
}

fn calculate_text_length<T: AsRef<str>>(text: T, font: &Font) -> f32 {
    let mut ret = 0.0;
    for c in text.as_ref().as_bytes() {
        let curr = *c;
        let idx = if (b' '..=127).contains(&curr) {
            (curr - 32) as usize
        } else {
            0
        };
        ret += (font.font[idx].len() as f32) / font.height;
    }
    ret
}

/// Split the text into multiple lines based on a given maximum width and font.
pub fn break_apart<T: AsRef<str>>(text: T, max_width: f32, font: &Font) -> Spliterated {
    let width = calculate_text_length(&text, font);
    if width <= max_width {
        return Spliterated {
            split: vec![String::from(text.as_ref())],
            width,
        };
    }

    let text = text.as_ref().replace('\n', "\n ");
    let fullsplit = text
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    let mut split = Vec::new();

    let mut base = 0;
    let mut maxlen = 0.0;
    #[allow(unused_assignments)]
    let mut prevlen = 0.0;
    let mut curlen = 0.0;

    for i in 0..fullsplit.len() {
        prevlen = curlen;
        curlen = calculate_text_length(fullsplit[base..i + 1].join(" "), font);
        if curlen > max_width || (i >= 1 && fullsplit[i - 1].contains('\n')) {
            split.push(fullsplit[base..i].join(" ").replace('\n', ""));
            if prevlen > maxlen {
                maxlen = prevlen;
            }
            base = i;
        }
    }

    split.push(fullsplit[base..].join(" "));
    if curlen > maxlen {
        maxlen = curlen;
    }
    Spliterated {
        split,
        width: maxlen,
    }
}

fn split_color(color: usize) -> Vec<u8> {
    vec![
        (color & 0xFF) as u8,
        ((color & 0xFF00) >> 8) as u8,
        ((color & 0xFF0000) >> 16) as u8,
    ]
}

/// Turns text into a 3-dimensional array containing the color data for each pixel
pub fn write_text<T: AsRef<str>>(text: T, font: Font, options: ImageOptions) -> Vec<Vec<Vec<u8>>> {
    let text = text.as_ref().replace(|c| matches!(c, '\0'..='\x7F'), "");
    let spliterated = break_apart(text, options.width - options.padding.0 * 2.0, &font);
    let split = spliterated.split;
    let width = if options.constant_width {
        options.width
    } else {
        spliterated.width + 2.0 * options.padding.0
    };
    let height = (split.len() as f32) * font.height + options.padding.1 * 2.0;

    let bg = split_color(options.background_color);
    let mut img = vec![vec![bg; ceil(width) as usize]; ceil(height) as usize];
    let mut line_at: usize = 0;

    for i in 0..(ceil(height) as usize) {
        if (i as f32) < options.padding.1 || (i as f32) >= height - options.padding.1 {
            continue;
        }

        if (i as f32 - options.padding.1) % font.height == 0.0 {
            line_at += 1;
        }

        let mut letter_on: usize = 0;
        let mut letter = Vec::new();
        let mut letter_base = options.padding.0;
        let bytes = split[line_at - 1].as_bytes();
        for j in 0..(ceil(width) as usize) {
            if (j as f32) < options.padding.0 || (j as f32) >= width - options.padding.0 {
                continue;
            }

            if (j as f32) >= letter_base + (letter.len() as f32) / font.height {
                letter_on += 1;
                if letter_on > bytes.len() {
                    continue;
                }
                letter_base = j as f32;
                let mut char = bytes[letter_on - 1].saturating_sub(32);
                if char >= 95 {
                    char = 0;
                }
                letter = font.font[char as usize].to_owned();
            }

            let thing = ((i as f32 - options.padding.1) - ((line_at - 1) as f32) * font.height)
                * ((letter.len() as f32) / font.height)
                + (j as f32 - letter_base);
            let alpha = letter[thing as usize];

            if alpha != 0 {
                let colors = split_color(options.text_color);
                img[i][j] = vec![
                    core::cmp::min(255, colors[0] * alpha / 255 + colors[0] * (1 - alpha / 255)),
                    core::cmp::min(255, colors[1] * alpha / 255 + colors[1] * (1 - alpha / 255)),
                    core::cmp::min(255, colors[2] * alpha / 255 + colors[2] * (1 - alpha / 255)),
                ];
            }
        }
    }

    img
}

fn little_endian(size: usize, data: usize) -> Vec<u8> {
    let mut ret = Vec::new();
    for i in 0..size {
        ret.push((data >> (8 * i) & 0x000000ff) as u8);
    }
    ret
}

/// Turns an array of pixels into a bitmap image.
pub fn write_image_data(data: &mut Vec<Vec<Vec<u8>>>) -> Vec<u8> {
    let mut imgdata: Vec<u8> = Vec::new();
    let width = data[0].len();
    let bytewidth = (((width as f32) * 3.0 / 4.0) + 0.5) as usize * 4;
    let height = data.len();
    let size = bytewidth * height;
    let file_size = size + 54;
    for i in (0..height).rev() {
        for j in 0..width {
            imgdata.append(&mut data[i][j]);
        }
        imgdata.append(&mut vec![0; bytewidth - width * 3]);
    }
    let mut ret = Vec::with_capacity(file_size);
    ret.append(&mut BMP_HEADER1.to_vec());
    ret.append(&mut little_endian(4, file_size));
    ret.append(&mut BMP_HEADER2.to_vec());
    ret.append(&mut little_endian(4, width));
    ret.append(&mut little_endian(4, height));
    ret.append(&mut BMP_HEADER3.to_vec());
    ret.append(&mut little_endian(4, size));
    ret.append(&mut vec![0x13, 0x0b, 0x00, 0x00, 0x13, 0x0b, 0x00, 0x00]);
    ret.append(&mut BMP_HEADER4.to_vec());
    ret.append(&mut imgdata);
    ret
}
