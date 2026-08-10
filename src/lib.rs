use macroquad::prelude::*;

pub mod component;
pub mod page;
pub mod system;
pub mod world;
pub mod ui;

#[derive(Clone)]
pub struct Gradient {
    pub colors: Vec<Color>,
    pub angle: f32,
}

pub type Entity = usize;
