extern crate font_loader as fonts;

use crate::{NeedleErr, NeedleError};
use fonts::system_fonts;

#[derive(Debug, Default)]
pub struct Fonts {
    available_fonts: Option<Box<[String]>>,
}

#[derive(Debug)]
pub enum FontTypes {
    Italic,
    Bold,
    Oblique,
    Monospace,
    Family(String),
}

impl Fonts {
    pub fn new() -> Self {
        Self {
            available_fonts: None,
        }
    }

    pub fn query_fonts(&mut self, font_type: Option<FontTypes>) -> Box<[String]> {
        let property = if let Some(font_type) = font_type {
            let property = match font_type {
                FontTypes::Bold => system_fonts::FontPropertyBuilder::new().bold().build(),
                FontTypes::Italic => system_fonts::FontPropertyBuilder::new().italic().build(),
                FontTypes::Monospace => {
                    system_fonts::FontPropertyBuilder::new().monospace().build()
                }
                FontTypes::Oblique => system_fonts::FontPropertyBuilder::new().oblique().build(),
                FontTypes::Family(family) => system_fonts::FontPropertyBuilder::new()
                    .family(&family)
                    .build(),
            };

            Some(property)
        } else {
            None
        };
        let sysfonts: Box<[String]> = if let Some(mut property) = property {
            system_fonts::query_specific(&mut property)
        } else {
            system_fonts::query_all()
        }
        .into();

        if self.available_fonts.is_none() {
            self.available_fonts = Some(sysfonts.clone());
        }

        sysfonts
    }

    pub fn read(&mut self, font: &String) -> NeedleErr<Vec<u8>> {
        let available_fonts = if let Some(ref available_fonts) = self.available_fonts {
            available_fonts.clone()
        } else {
            self.query_fonts(None)
        };

        if available_fonts.contains(font) {
            let property = system_fonts::FontPropertyBuilder::new()
                .family(font)
                .build();

            if let Some((font, _)) = system_fonts::get(&property) {
                return Ok(font);
            }
        }

        Err(NeedleError::Other)
    }
}

#[test]
fn test_fonts_0001() {
    let fonts = Fonts::new();

    assert_eq!(None, fonts.available_fonts)
}

#[test]
fn test_fonts_0002() {
    let mut fonts = Fonts::new();
    let available_fonts = fonts.query_fonts(None);

    assert_eq!(Some(available_fonts), fonts.available_fonts)
}

#[test]
fn test_fonts_0003() {
    let mut fonts = Fonts::new();
    let available_fonts = fonts.query_fonts(Some(FontTypes::Bold));

    assert_eq!(Some(available_fonts), fonts.available_fonts)
}

#[test]
fn test_fonts_0004() {
    let mut fonts = Fonts::new();
    let available_fonts = fonts.query_fonts(Some(FontTypes::Italic));

    assert_eq!(Some(available_fonts), fonts.available_fonts)
}

#[test]
fn test_fonts_0005() {
    let mut fonts = Fonts::new();
    let available_fonts = fonts.query_fonts(Some(FontTypes::Monospace));

    assert_eq!(Some(available_fonts), fonts.available_fonts)
}

#[test]
fn test_fonts_0006() {
    let mut fonts = Fonts::new();
    let available_fonts = fonts.query_fonts(Some(FontTypes::Oblique));

    assert_eq!(Some(available_fonts), fonts.available_fonts)
}

#[test]
fn test_fonts_0007() {
    let mut fonts = Fonts::new();
    let available_fonts = fonts.query_fonts(None);
    let result = fonts.read(&available_fonts[0]);

    if let Ok(ref path) = result {
        println!("{:?}", &path[..50]);
    }

    assert!(result.is_ok())
}

#[test]
fn test_fonts_0008() {
    let mut fonts = Fonts::new();
    let result = fonts.read(&"This doesn't exist".to_string());

    assert!(result.is_err())
}
