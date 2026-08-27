use std::{collections::HashMap, hash::Hash};

use macroquad::prelude::*;

pub mod component;
pub mod helper;
pub mod render_q;
mod system;
pub mod table;
pub mod ui;
pub mod world;

#[derive(Clone, Debug)]
pub struct Gradient {
    pub colors: Vec<Color>,
    pub angle: f32,
}

pub struct FontRegistry<U> {
    pub fonts: HashMap<U, Font>,
}

impl<U> FontRegistry<U>
where
    U: Eq + Hash,
{
    pub fn new() -> FontRegistry<U> {
        FontRegistry {
            fonts: HashMap::new(),
        }
    }

    pub fn get(&self, font: &U) -> Option<&Font> {
        self.fonts.get(font)
    }
}

pub type UIElementId = usize;
