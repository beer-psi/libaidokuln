extern crate test;
use super::*;

#[bench]
fn lorem_ipsum_5_paragraphs(bench: &mut test::Bencher) {
    bench.iter(|| {
        write_text(
            include_str!("./lorem.txt"),
            0,
            fonts::times::TIMES36,        
            ImageOptions::default(),
        )
    })
}
