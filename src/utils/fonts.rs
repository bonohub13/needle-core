extern crate font_loader as fonts;

use crate::{NeedleConfig, NeedleErr, NeedleError};
use fonts::system_fonts;
use glyphon::fontdb::Source;
use regex::Regex;
use std::{fs, path::PathBuf, sync::Arc};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FontType {
    System,
    Needle,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Font {
    pub font: Box<str>,
    pub font_type: FontType,
}

#[derive(Debug, Default)]
pub struct Fonts {
    available_fonts: Option<Box<[Font]>>,
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
    const FONT_SUBDIR: &'static str = "fonts/";
    #[cfg(target_os = "windows")]
    const DIRECTORY_DELIMITER: &'static str = "\\";
    #[cfg(target_os = "linux")]
    const DIRECTORY_DELIMITER: &'static str = "/";

    pub fn new() -> Self {
        Self {
            available_fonts: None,
        }
    }

    #[inline]
    pub fn available_fonts(&self) -> Box<[Font]> {
        self.available_fonts.clone().unwrap_or([].into())
    }

    pub fn query_fonts(&mut self, font_type: Option<FontTypes>) -> NeedleErr<()> {
        let mut fonts = vec![];
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
        let sysfonts = if let Some(mut property) = property {
            system_fonts::query_specific(&mut property)
        } else {
            system_fonts::query_all()
        };
        let config_fonts = Self::search_fonts()?;

        sysfonts.iter().for_each(|font| {
            fonts.push(Font {
                font: font.as_str().into(),
                font_type: FontType::System,
            })
        });
        config_fonts.iter().for_each(|font| {
            fonts.push(Font {
                font: font.display().to_string().into(),
                font_type: FontType::Needle,
            })
        });

        if self.available_fonts.is_none() {
            self.available_fonts = Some(fonts.clone().into());
        }

        Ok(())
    }

    pub fn read(&mut self, font: &str) -> NeedleErr<Source> {
        let available_fonts = if let Some(ref available_fonts) = self.available_fonts {
            available_fonts.clone()
        } else {
            self.query_fonts(None)?;

            /* Fonts have been queried, so something should be in here */
            self.available_fonts.clone().unwrap_or([].into())
        };
        let font = available_fonts.iter().find(|f| f.font == font.into());

        if let Some(font) = font {
            match font.font_type {
                FontType::System => {
                    let property = system_fonts::FontPropertyBuilder::new()
                        .family(&font.font)
                        .build();

                    #[cfg(debug_assertions)]
                    log::debug!("{}", font.font);

                    if let Some((font, _)) = system_fonts::get(&property) {
                        Ok(Source::Binary(Arc::new(font)))
                    } else {
                        Err(NeedleError::FailedToReadFile)
                    }
                }
                FontType::Needle => {
                    let config_path = Self::find_font(&font.font)?;

                    Ok(Source::File(config_path))
                }
            }
        } else {
            Err(NeedleError::FailedToReadFile)
        }
    }

    pub fn font_names(&self) -> Option<Box<[String]>> {
        if let Some(ref available_fonts) = self.available_fonts {
            let mut output = vec![];

            available_fonts
                .iter()
                .for_each(|font| match font.font_type {
                    FontType::Needle => {
                        if let Some(file) = font.font.split(Self::DIRECTORY_DELIMITER).last() {
                            if let Some(font) = file.split(".").next() {
                                output.push(font.to_string());
                            }
                        }
                    }
                    FontType::System => {
                        output.push(font.font.to_string());
                    }
                });

            Some(output.into())
        } else {
            None
        }
    }

    fn query_files(path: PathBuf) -> NeedleErr<Box<[PathBuf]>> {
        match fs::read_dir(path) {
            Ok(paths) => {
                let mut files = vec![];
                for path in paths {
                    let path = match path {
                        Ok(path) => Ok(path.path()),
                        Err(err) => Err(NeedleError::FailedToSearchDir(err.into())),
                    }?;

                    files.push(path);
                }

                Ok(files.into())
            }
            Err(err) => Err(NeedleError::FailedToReadDir(err.into())),
        }
    }

    fn search_fonts() -> NeedleErr<Box<[PathBuf]>> {
        match NeedleConfig::config_path(true, Some(Self::FONT_SUBDIR)) {
            Ok(path) => Self::query_files(path),
            Err(err) => Err(NeedleError::FailedToSearchDir(err.into())),
        }
    }

    fn find_font(font_name: &str) -> NeedleErr<PathBuf> {
        let fonts_path = NeedleConfig::config_path(true, Some(Self::FONT_SUBDIR))?;

        if fonts_path.exists() {
            match Self::query_files(fonts_path) {
                Ok(paths) => {
                    let re = match Regex::new(&format!(r"/{}\.[a-z]*$", font_name)) {
                        Ok(regex) => Ok(regex),
                        Err(err) => Err(NeedleError::InvalidRegex(err.into())),
                    }?;

                    for path in paths {
                        if re.find(font_name).is_some() {
                            return Ok(path);
                        }
                    }

                    Err(NeedleError::InvalidPath)
                }
                Err(err) => Err(NeedleError::FailedToSearchDir(err.into())),
            }?;
        }
        Err(NeedleError::InvalidPath)
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
    let result = fonts.query_fonts(None);

    assert!(result.is_ok());
    if result.is_ok() {
        assert!(fonts.available_fonts.is_some())
    }
}

#[test]
fn test_fonts_0003() {
    let mut fonts = Fonts::new();
    let result = fonts.query_fonts(Some(FontTypes::Bold));

    assert!(result.is_ok());
    if result.is_ok() {
        assert!(fonts.available_fonts.is_some())
    }
}

#[test]
fn test_fonts_0004() {
    let mut fonts = Fonts::new();
    let result = fonts.query_fonts(Some(FontTypes::Italic));

    assert!(result.is_ok());
    if result.is_ok() {
        assert!(fonts.available_fonts.is_some())
    }
}

#[test]
fn test_fonts_0005() {
    let mut fonts = Fonts::new();
    let result = fonts.query_fonts(Some(FontTypes::Monospace));

    assert!(result.is_ok());
    if result.is_ok() {
        assert!(fonts.available_fonts.is_some())
    }
}

#[test]
fn test_fonts_0006() {
    let mut fonts = Fonts::new();
    let result = fonts.query_fonts(Some(FontTypes::Oblique));

    assert!(result.is_ok());
    if result.is_ok() {
        assert!(fonts.available_fonts.is_some())
    }
}

#[test]
fn test_fonts_0007() {
    let mut fonts = Fonts::new();
    let result = fonts.query_fonts(None);
    assert!(result.is_ok());
    let result = if let Some(ref available_fonts) = fonts.available_fonts.clone() {
        fonts.read(&available_fonts[0])
    } else {
        Err(NeedleError::FailedToReadFile)
    };

    assert!(result.is_ok())
}

#[test]
fn test_fonts_0008() {
    let mut fonts = Fonts::new();
    let font = Font {
        font: "This doesn't exist".into(),
        font_type: FontType::System,
    };
    let result = fonts.read(&font);

    assert!(result.is_err())
}
