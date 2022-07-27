#![allow(dead_code)]
pub mod arial;
pub mod georgia;
pub mod times;
pub mod palatino;

pub struct Font {
    pub height: f32,
    pub font: [&'static [u8]; 95],
}

impl Font {
    /// Get a font from its name.
    pub fn from_name<T: AsRef<str>>(name: T) -> Self {
        match name.as_ref() {
            "arial18" => arial::ARIAL18,
            "arial24" => arial::ARIAL24,
            "arial30" => arial::ARIAL30,
            "arial36" => arial::ARIAL36,
            "times18" => times::TIMES18,
            "times24" => times::TIMES24,
            "times30" => times::TIMES30,
            "times36" => times::TIMES36,
            "georgia18" => georgia::GEORGIA18,
            "georgia24" => georgia::GEORGIA24,
            "georgia30" => georgia::GEORGIA30,
            "georgia36" => georgia::GEORGIA36,
            "palatino18" => palatino::PALATINO18,
            "palatino24" => palatino::PALATINO24,
            "palatino30" => palatino::PALATINO30,
            "palatino36" => palatino::PALATINO36,
            _ => times::TIMES18,
        }
    }
}
