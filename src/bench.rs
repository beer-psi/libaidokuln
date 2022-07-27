extern crate test;
use super::*;

#[bench]
fn lorem_ipsum_5_paragraphs(bench: &mut test::Bencher) {
    bench.iter(|| {
        write_text(
            include_str!("./lorem.txt"),
            fonts::times::TIMES36,
            ImageOptions {
                text_color: 0,
                background_color: 0xFFFFFF,
                padding: Padding(40.0, 40.0),
                width: 1080.0,
                constant_width: false,
            },
        )
    })
}
