use macroquad::prelude::*;

pub mod component;
pub mod helper;
pub mod page;
pub mod system;
pub mod ui;
pub mod world;
pub mod table;
pub mod render_q;

#[derive(Clone, Debug)]
pub struct Gradient {
    pub colors: Vec<Color>,
    pub angle: f32,
}

pub type UIElementId = usize;
