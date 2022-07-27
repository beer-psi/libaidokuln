#![allow(dead_code)]
pub mod arial;
pub mod georgia;
pub mod times;
pub mod palatino;
pub struct Font {
    pub height: f32,
    pub font: [&'static [u8]; 95],
}
