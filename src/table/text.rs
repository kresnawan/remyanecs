use crate::{
    UIElementId,
    component::{
        Dimension, GlobalPosition, Parent, Position, PositionType, Style, UIColor, ZIndex,
    },
    helper::draw_text_extended,
    render_q::{UIRender, UIVisual},
};

use macroquad::prelude::*;

pub enum TextAlignment {
    Left,
    Center,
    Right,
}

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
    pub lines: Vec<Vec<String>>,

    pub is_dirty: Vec<bool>
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
            lines: Vec::new(),
            is_dirty: Vec::new()
        }
    }
}

pub fn render_text(render_data: &UIRender) {
    let UIVisual::UIText(config, max_width, text) = render_data.vis else {
        return;
    };

    let pos = render_data.global_pos;

    let color = if let UIColor::Fill(color) = config.color {
        color
    } else {
        BLACK
    };

    let mut y = pos.y;
    let line_dimension = measure_text(&text[0], None, config.font_size, 1.);

    y += line_dimension.height;

    for line in text {

        let current_line_dimension = measure_text(line, None, config.font_size, 1.);

        // let cx = render_data.dim.w / 2. - current_line_dimension.width / 2.;
        let cx = render_data.dim.w - current_line_dimension.width;

        draw_text_extended(
            line,
            cx + pos.x,
            y,
            max_width,
            TextParams {
                font: None,
                font_size: config.font_size,
                color: color,
                ..Default::default()
            },
        );

        y += line_dimension.height;
        y += config.line_spacing;
    }
}
