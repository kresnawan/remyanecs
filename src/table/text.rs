use crate::{
    UIElementId,
    component::{
        Button, ButtonConfig, Dimension, GlobalPosition, OnClickEvent, Parent, Position,
        PositionType, Style, UIColor, ZIndex,
    },
    helper::{draw_rectangle_extended, draw_text_extended, draw_text_extended_experimental},
    render_q::{UIRender, UIVisual},
    world::World,
};

use macroquad::prelude::*;

#[derive(Debug)]
pub struct UITextTable {
    pub ids: Vec<UIElementId>,
    pub position: Vec<Position>,
    pub z_index: Vec<ZIndex>,
    pub global_pos: Vec<GlobalPosition>,
    pub position_type: Vec<PositionType>,
    pub dimension: Vec<Dimension>,
    pub parent: Vec<Option<Parent>>,

    pub visible: Vec<bool>,
    pub max_width: Vec<Option<f32>>,
    pub style: Vec<Style>,
    pub value: Vec<String>,
}

impl UITextTable {
    pub fn new() -> UITextTable {
        UITextTable {
            ids: Vec::new(),
            position: Vec::new(),
            z_index: Vec::new(),
            global_pos: Vec::new(),
            position_type: Vec::new(),
            dimension: Vec::new(),
            parent: Vec::new(),
            visible: Vec::new(),
            max_width: Vec::new(),
            style: Vec::new(),
            value: Vec::new(),
        }
    }
}

pub fn render_text(render_data: &UIRender) {
    let UIVisual::UIText(config, max_width, text) = render_data.vis else {
        return;
    };

    let pos = render_data.global_pos;
    let dim = render_data.dim;

    let color = if let UIColor::Fill(color) = config.color {
        color
    } else {
        BLACK
    };

    draw_text_extended_experimental(
        text,
        pos.x,
        pos.y + dim.h,
        max_width,
        TextParams {
            font: None,
            font_size: config.font_size,
            color: color,
            ..Default::default()
        },
    );
}
