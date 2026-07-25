/**
* Copyright (c) AWildDevAppears
*/
use std::collections::HashMap;

use raylib::{
    RaylibHandle, RaylibThread,
    text::{Font, RaylibFont},
    texture::RaylibTexture2D,
};

pub struct FontManager {
    fonts: HashMap<String, Font>,
}

impl FontManager {
    pub fn new(
        game: &mut RaylibHandle,
        thread: &RaylibThread,
        font_refs: &[FontReference],
    ) -> Self {
        let mut fonts: HashMap<String, Font> = HashMap::new();

        for font in font_refs {
            let file_data = std::fs::read(&font.path).expect("Failed to read font file.");
            let format = match font.format {
                FontFormat::TTF => ".ttf",
            };

            let font_data = game
                .load_font_from_memory(&thread, format, &file_data, 16, None)
                .expect("Failed to load default font data");

            font_data
                .texture()
                .set_texture_filter(&thread, raylib::ffi::TextureFilter::TEXTURE_FILTER_BILINEAR);

            fonts.insert(font.name.clone(), font_data);
        }

        Self { fonts }
    }

    pub fn get_font(&self, name: &str) -> &Font {
        self.fonts
            .get(&name.to_string())
            .expect("Could not get font by name")
    }
}

pub struct FontReference {
    pub name: String,
    pub path: &'static str,
    pub format: FontFormat,
}

pub enum FontFormat {
    TTF,
}
