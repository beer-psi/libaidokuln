//! # libaidokuln
//!
//! A WASM/no_std library for generating bitmap images from text. As the name implies, it
//! is geared toward usage in [Aidoku](https://aidoku.app), but can be used anywhere.
//!
//! ## Usage
//! ```
//! use libaidokuln::{write_text, fonts, ImageOptions, Padding};
//!
//! // Write this image anywhere you want
//! let img = write_text(
//!     "Hello World",
//!     0,
//!     fonts::times::TIMES36,
//!     ImageOptions::default(),
//! );
//!
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
pub mod fonts;
use fonts::Font;

extern crate alloc;
use alloc::{string::String, vec, vec::Vec};

/// Struct representing text padding, used to tell the library to add margins
/// to the text.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Padding(
    /// The horizontal padding.
    pub f32,
    /// The vertical padding.
    pub f32,
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitmapPixel(pub u8, pub u8, pub u8);

/// Rendering options.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImageOptions {
    /// Text color. It is an usize, but can be written in an BGR-like format
    /// using hex notation, e.g., `0x1F1E33`.
    pub text_color: usize,

    /// Background color. Can be written in an BGR-like format simular to text
    /// color.
    pub background_color: usize,

    /// The margins for the generated page.
    pub padding: Padding,

    /// Maximum page width.
    pub width: f32,

    /// Whether the renderer should force the given max width or not.
    pub constant_width: bool,

    /// The number of lines in a given page.
    pub lines: usize,
}

impl Default for ImageOptions {
    fn default() -> Self {
        Self {
            text_color: 0,
            background_color: 0xFFFFFF,
            padding: Padding(20.0, 20.0),
            width: 800.0,
            constant_width: true,
            lines: 60,
        }
    }
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

fn split_color(color: usize) -> BitmapPixel {
    BitmapPixel(
        (color & 0xFF) as u8,
        ((color & 0xFF00) >> 8) as u8,
        ((color & 0xFF0000) >> 16) as u8,
    )
}

/// Turns text into a 3-dimensional array containing the color data for each pixel
///
/// Set the page parameter to 0 to generate an image containing all text.
/// 
/// This assumes that the given text is ASCII. Anything not ASCII will be filtered out.
/// You may want to preserve them by using a crate like [deunicode](https://lib.rs/crates/deunicode).
pub fn write_text<T: AsRef<str>>(
    text: T,
    page: usize,
    font: Font,
    options: ImageOptions,
) -> Vec<u8> {
    let text = text.as_ref().chars().filter(|&c| (c as u8) < 0x7F).collect::<String>();

    let spliterated = break_apart(text, options.width - options.padding.0 * 2.0, &font);
    let split = if page >= 1 {
        spliterated.split[(page - 1) * options.lines
            ..core::cmp::min(spliterated.split.len(), page * options.lines)]
            .to_vec()
    } else {
        spliterated.split
    };

    let width = if options.constant_width {
        options.width
    } else {
        spliterated.width + 2.0 * options.padding.0
    };
    let height = (split.len() as f32) * font.height + options.padding.1 * 2.0;
    let ceil_width = ceil(width) as usize;
    let ceil_height = ceil(height) as usize;

    let mut img = vec![split_color(options.background_color); ceil_width * ceil_height];
    let mut line_at: usize = 0;

    for i in (options.padding.1 as usize)..((height - options.padding.1) as usize) {
        if (i as f32) < options.padding.1 {
            continue;
        }

        if (i as f32 - options.padding.1) % font.height == 0.0 {
            line_at += 1;
        }

        let mut letter: &[u8] = &[];
        let mut letter_on: usize = 0;
        let mut letter_base = options.padding.0;
        let bytes = split[line_at - 1].as_bytes();
        for j in (ceil(options.padding.0) as usize)..((width - options.padding.0) as usize) {
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
                letter = font.font[char as usize];
            }

            let alpha = letter[(((i as f32 - options.padding.1)
                - ((line_at - 1) as f32) * font.height)
                * ((letter.len() as f32) / font.height)
                + (j as f32 - letter_base)) as usize];

            if alpha != 0 {
                let colors = split_color(options.text_color);
                img[i * ceil_width + j] = BitmapPixel(
                    core::cmp::min(
                        255,
                        ((colors.0 as f32) * (alpha as f32) / 255.0
                            + (colors.0 as f32) * (1.0 - alpha as f32 / 255.0))
                            as u8,
                    ),
                    core::cmp::min(
                        255,
                        ((colors.1 as f32) * (alpha as f32) / 255.0
                            + (colors.1 as f32) * (1.0 - alpha as f32 / 255.0))
                            as u8,
                    ),
                    core::cmp::min(
                        255,
                        ((colors.2 as f32) * (alpha as f32) / 255.0
                            + (colors.2 as f32) * (1.0 - alpha as f32 / 255.0))
                            as u8,
                    ),
                );
            }
        }
    }

    let bytewidth = (((ceil_width as f32) * 3.0 / 4.0) + 0.5) as usize * 4;
    let size = bytewidth * ceil_height;
    let file_size = size + 54;

    let mut ret = Vec::with_capacity(file_size);
    ret.extend([0x42, 0x4D]); // bmp header 1
    ret.append(&mut little_endian(4, file_size));
    ret.extend([
        0x00, 0x00, 0x00, 0x00, 0x36, 0x00, 0x00, 0x00, 0x28, 0x00, 0x00, 0x00,
    ]); // bmp header 2
    ret.append(&mut little_endian(4, ceil_width));
    ret.append(&mut little_endian(4, ceil_height));
    ret.extend([0x01, 0x00, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00]); // bmp header 3
    ret.append(&mut little_endian(4, size));
    ret.extend([0x13, 0x0b, 0x00, 0x00, 0x13, 0x0b, 0x00, 0x00]);
    ret.extend([0x00; 8]); // bmp header 4
    for i in (0..ceil_height).rev() {
        for j in 0..ceil_width {
            let idx = i * ceil_width + j;
            ret.push(img[idx].0);
            ret.push(img[idx].1);
            ret.push(img[idx].2);
        }
        ret.append(&mut vec![0; bytewidth - ceil_width * 3]);
    }
    ret
}

fn little_endian(size: usize, data: usize) -> Vec<u8> {
    let mut ret = Vec::new();
    for i in 0..size {
        ret.push((data >> (8 * i) & 0x000000ff) as u8);
    }
    ret
}
