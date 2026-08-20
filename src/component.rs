use macroquad::prelude::*;

use crate::{FontKind, Gradient, UIElementId};

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

    pub fn center() -> Position {
        Position {
            x: 0.,
            y: 0.,
            dyn_x: Some(DynPos::Center),
            dyn_y: Some(DynPos::Center),
        }
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
    pub fn new() -> Dimension {
        Dimension {
            w: 0.,
            h: 0.,
            dyn_w: None,
            dyn_h: None,
        }
    }

    pub fn w(mut self, value: f32) -> Dimension {
        self.w = value;
        self
    }

    pub fn h(mut self, value: f32) -> Dimension {
        self.h = value;
        self
    }

    pub fn dyn_w(mut self, value: DynDim) -> Dimension {
        self.dyn_w = Some(value);
        self
    }

    pub fn dyn_h(mut self, value: DynDim) -> Dimension {
        self.dyn_h = Some(value);
        self
    }

    pub fn from(w: f32, h: f32) -> Dimension {
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

impl UIColor {
    pub fn as_fill(&self) -> Color {
        match self {
            UIColor::Fill(color) => *color,
            UIColor::Gradient(grad) => grad.colors[0],
        }
    }
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
    pub font: FontKind,
    pub font_size: u16,
    pub line_spacing: f32,
    pub outline: f32,
    pub outline_color: Color,
    pub corner_radius: f32,
}

impl Style {
    pub fn new(
        bg_color: UIColor,
        color: UIColor,
        font: FontKind,
        font_size: u16,
        outline: f32,
        outline_color: Color,
        corner_radius: f32,
        line_spacing: f32
    ) -> Style {
        Style {
            bg_color,
            color,
            font,
            font_size,
            outline,
            outline_color,
            corner_radius,
            line_spacing
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            bg_color: UIColor::Fill(WHITE),
            color: UIColor::Fill(BLACK),
            font: FontKind::NunitoRegular,
            font_size: 24,
            outline: 0.,
            outline_color: BLACK,
            corner_radius: 0.,
            line_spacing: 0.
        }
    }
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
