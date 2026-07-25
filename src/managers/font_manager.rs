/**
* Copyright (c) AWildDevAppears
*/
use std::collections::HashMap;

use raylib::{RaylibHandle, RaylibThread, text::Font};

pub struct FontManager {
    fonts: HashMap<String, Font>,
}

impl FontManager {
    pub fn new() -> Self {
        Self {
            fonts: HashMap::new(),
        }
    }

    pub fn preload(
        &mut self,
        game: &mut RaylibHandle,
        thread: &RaylibThread,
        fonts: &[FontReference],
    ) {
        for font in fonts {
            let file_data = std::fs::read(&font.path).expect("Failed to read font file.");
            let format = match font.format {
                FontFormat::TTF => ".ttf",
            };

            let font_data = game
                .load_font_from_memory(&thread, format, &file_data, 16, None)
                .expect("Failed to load default font data");

            self.fonts.insert(font.name.clone(), font_data);
        }
    }

    pub fn get_font(&self, name: &str) -> &Font {
        self.fonts.get(name).expect("Could not get font by name")
    }
}

pub struct FontReference {
    pub name: String,
    pub path: String,
    pub format: FontFormat,
}

pub enum FontFormat {
    TTF,
}
