use macroquad::prelude::*;

use crate::{Entity, Gradient};

pub struct ButtonStyle {
    pub text: String,
    pub style: Style,
    pub hover_style: Option<Style>,
}

pub struct ZIndex(pub u32);

pub struct Button;
pub struct Rectangle;

#[derive(Clone)]
pub enum DynPos {
    Start,
    Center,
    End,
    Custom,
}

pub enum DynDim {
    Full,
    Percent(f32),
    Custom,
}

pub struct GlobalPosition {
    pub x: f32,
    pub y: f32,
}

pub struct Position {
    pub x: f32,
    pub y: f32,
    pub dyn_x: Option<DynPos>,
    pub dyn_y: Option<DynPos>,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Position {
        Position {
            x,
            y,
            dyn_x: None,
            dyn_y: None,
        }
    }
    pub fn set_x(&mut self, value: f32) {
        self.x = value;
    }

    pub fn set_y(&mut self, value: f32) {
        self.y = value;
    }
}

pub struct Dimension {
    pub w: f32,
    pub h: f32,
    pub dyn_w: Option<DynDim>,
    pub dyn_h: Option<DynDim>,
}

impl Dimension {
    pub fn new(w: f32, h: f32) -> Dimension {
        Dimension {
            w,
            h,
            dyn_w: None,
            dyn_h: None,
        }
    }
    pub fn set_width(&mut self, value: f32) {
        self.w = value;
    }

    pub fn set_height(&mut self, value: f32) {
        self.h = value;
    }
}

pub enum Direction {
    Vertical,
    Horizontal,
}

pub enum Display {
    Normal,
    Grid { direction: Direction, gap: f32 },
    Flex,
}

pub enum PositionType {
    Absolute,
    Relative,
}

pub struct Div;

#[derive(Clone)]
pub enum UIColor {
    Fill(Color),
    Gradient(Gradient),
}

#[derive(Clone)]
pub enum UIEvent {
    CreateRoom,
    OpenDialogueBox(Entity),
    CloseDialogueBox(Entity),
}

pub struct OnClickEvent(pub UIEvent);
pub struct Visible;

pub struct Parent(pub Entity);

#[derive(Clone)]
pub struct Style {
    pub bg_color: UIColor,
    pub color: UIColor,
    pub font: u32,
    pub font_size: u16,
    pub outline: Option<f32>,
    pub outline_color: Option<Color>,
}
