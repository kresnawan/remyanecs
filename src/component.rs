use macroquad::prelude::*;

use crate::{Gradient, UIElementId};

#[derive(Debug)]
pub struct ButtonConfig {
    pub text: String,
    pub style: Style,
    pub hover_style: Option<Style>,
}

#[derive(Clone, Debug)]
pub enum DynPos {
    Start,
    Center,
    End,
    Custom,
}

#[derive(Debug)]
pub enum DynDim {
    Full,
    Percent(f32),
    Custom,
}

#[derive(Debug)]
pub struct GlobalPosition {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub dyn_x: Option<DynPos>,
    pub dyn_y: Option<DynPos>,
}

impl Position {
    pub fn new() -> Position {
        Position {
            x: 0.,
            y: 0.,
            dyn_x: None,
            dyn_y: None,
        }
    }
    pub fn from(x: f32, y: f32) -> Position {
        Position {
            x,
            y,
            dyn_x: None,
            dyn_y: None,
        }
    }

    pub fn x(mut self, value: f32) -> Position {
        self.x = value;
        self
    }

    pub fn y(mut self, value: f32) -> Position {
        self.y = value;
        self
    }

    pub fn dyn_x(mut self, value: DynPos) -> Position {
        self.dyn_x = Some(value);
        self
    }

    pub fn dyn_y(mut self, value: DynPos) -> Position {
        self.dyn_y = Some(value);
        self
    }

    pub fn set_x(&mut self, value: f32) {
        self.x = value;
    }

    pub fn set_y(&mut self, value: f32) {
        self.y = value;
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Direction {
    Vertical,
    Horizontal,
}

#[derive(Debug)]
pub enum Display {
    Normal,
    Grid { direction: Direction, gap: f32 },
    Flex,
}

#[derive(Debug)]
pub enum PositionType {
    Absolute,
    Relative,
}

#[derive(Clone, Debug)]
pub enum UIColor {
    Fill(Color),
    Gradient(Gradient),
}

#[derive(Clone, Debug)]
pub enum UIEvent {
    CreateRoom,
    OpenDialogueBox(UIElementId),
    CloseDialogueBox(UIElementId),
}

#[derive(Clone, Debug)]
pub struct Style {
    pub bg_color: UIColor,
    pub color: UIColor,
    pub font: u32,
    pub font_size: u16,
    pub outline: Option<f32>,
    pub outline_color: Option<Color>,
}

pub type ZIndex = u32;

#[derive(Debug)]
pub struct OnClickEvent(pub UIEvent);

pub struct Visible;

pub type Parent = usize;

#[derive(Debug)]
pub struct Button;

#[derive(Debug)]
pub struct Div;
