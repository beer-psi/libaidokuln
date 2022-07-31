#[allow(dead_code)]
#[cfg_attr(not(test), cfg(feature = "fonts"))]
#[allow(clippy::all, unknown_lints)]
mod data {
    pub mod arial;
    pub mod atkinson;
    pub mod fs_brabo;
    pub mod georgia;
    pub mod palatino;
    pub mod times;
}

#[cfg_attr(not(test), cfg(feature = "fonts"))]
pub use data::*;

pub struct Font {
    pub height: f32,
    pub font: [&'static [u8]; 95],
}

impl AsRef<Font> for Font {
    fn as_ref(&self) -> &Font {
        self
    }
}

impl Font {
    /// Get a font from its name.
    #[cfg_attr(not(test), cfg(feature = "fonts"))]
    pub fn from_name<T: AsRef<str>>(name: T) -> Self {
        match name.as_ref() {
            "arial18" => arial::ARIAL18,
            "arial24" => arial::ARIAL24,
            "arial30" => arial::ARIAL30,
            "arial36" => arial::ARIAL36,
            "atkinson18" => atkinson::ATKINSON18,
            "atkinson24" => atkinson::ATKINSON24,
            "atkinson30" => atkinson::ATKINSON30,
            "atkinson36" => atkinson::ATKINSON36,
            "fs_brabo18" => fs_brabo::FS_BRABO18,
            "fs_brabo24" => fs_brabo::FS_BRABO24,
            "fs_brabo30" => fs_brabo::FS_BRABO30,
            "fs_brabo36" => fs_brabo::FS_BRABO36,
            "georgia18" => georgia::GEORGIA18,
            "georgia24" => georgia::GEORGIA24,
            "georgia30" => georgia::GEORGIA30,
            "georgia36" => georgia::GEORGIA36,
            "palatino18" => palatino::PALATINO18,
            "palatino24" => palatino::PALATINO24,
            "palatino30" => palatino::PALATINO30,
            "palatino36" => palatino::PALATINO36,
            "times18" => times::TIMES18,
            "times24" => times::TIMES24,
            "times30" => times::TIMES30,
            "times36" => times::TIMES36,
            _ => times::TIMES18,
        }
    }
}
