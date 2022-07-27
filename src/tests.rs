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
        fonts::palatino::PALATINO36,
        ImageOptions {
            text_color: 0,
            background_color: 0xFFFFFF,
            padding: Padding(40.0, 40.0),
            width: 1080.0,
            constant_width: false,
        },
    );
    let mut file = File::create("test.bmp").unwrap();
    file.write_all(&img).unwrap();
}
