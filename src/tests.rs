extern crate std;
use super::*;
use std::fs::File;
use std::io::Write;

#[test]
fn length() {
    assert_eq!(
        calculate_text_length("Hello World", &fonts::times::TIMES36),
        177.0
    );
}

#[test]
fn spliterate() {
    assert_eq!(
        break_apart("Hello World", 200.0, &fonts::times::TIMES36),
        Spliterated {
            split: vec![String::from("Hello World")],
            width: 177.0,
        }
    );
}

#[test]
fn it_works() {
    let img = write_text(
        include_str!("./lorem.txt"),
        0,
        fonts::palatino::PALATINO18,
        ImageOptions::default(),
    );
    let mut file = File::create("test.bmp").unwrap();
    file.write_all(&img).unwrap();
}

#[test]
fn pagination() {
    let img = write_text(
        include_str!("./lorem.txt"),
        1,
        fonts::palatino::PALATINO18,
        ImageOptions {
            text_color: 0xFF0000,
            ..Default::default()
        },
    );
    let mut file = File::create("test2.bmp").unwrap();
    file.write_all(&img).unwrap();
}
