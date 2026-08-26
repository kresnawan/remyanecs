use std::collections::HashMap;

use macroquad::prelude::*;

pub mod component;
pub mod helper;
pub mod render_q;
pub mod system;
pub mod table;
pub mod ui;
pub mod world;

#[derive(Clone, Debug)]
pub struct Gradient {
    pub colors: Vec<Color>,
    pub angle: f32,
}

pub struct FontRegistry {
    pub fonts: HashMap<FontKind, Font>,
}

impl FontRegistry {
    pub fn new() -> FontRegistry {
        FontRegistry { fonts: HashMap::new() }
    }

    pub fn get(&self, font: &FontKind) -> Option<&Font> {
        self.fonts.get(font)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FontKind {
    NunitoBlack,
    NunitoBold,
    NunitoRegular,
}

pub type UIElementId = usize;
