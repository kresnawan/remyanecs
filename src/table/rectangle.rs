use crate::{
    UIElementId,
    component::{
        Dimension, GlobalPosition, Parent, Position, PositionType, Style, UIColor, ZIndex,
    },
    helper::draw_rectangle_extended,
    render_q::{UIRender, UIVisual},
};

use macroquad::prelude::*;

#[derive(Debug)]
pub struct UIRectangleTable<U> {
    pub ids: Vec<UIElementId>,
    pub position: Vec<Position>,
    pub z_index: Vec<ZIndex>,
    pub global_pos: Vec<GlobalPosition>,
    pub position_type: Vec<PositionType>,
    pub dimension: Vec<Dimension>,
    pub parent: Vec<Option<Parent>>,

    pub visible: Vec<bool>,
    pub style: Vec<Style<U>>,
}

impl<U> UIRectangleTable<U> {
    pub fn new() -> UIRectangleTable<U> {
        UIRectangleTable {
            ids: Vec::new(),
            position: Vec::new(),
            z_index: Vec::new(),
            global_pos: Vec::new(),
            position_type: Vec::new(),
            dimension: Vec::new(),
            parent: Vec::new(),
            visible: Vec::new(),
            style: Vec::new(),
        }
    }
}

pub fn render_rectangle<U>(render_data: &UIRender<U>) {
    let UIVisual::UIRectangle(style) = render_data.vis else {
        return;
    };

    let pos = render_data.global_pos;
    let dim = render_data.dim;

    let color_1 = if let UIColor::Fill(color) = style.bg_color {
        color
    } else if let UIColor::Gradient(grad) = &style.bg_color {
        grad.colors[0]
    } else {
        BLACK
    };

    let color_2 = if let UIColor::Fill(color) = style.bg_color {
        color
    } else if let UIColor::Gradient(grad) = &style.bg_color {
        grad.colors[1]
    } else {
        BLACK
    };

    let gradient_angle = if let UIColor::Gradient(grad) = &style.bg_color {
        grad.angle
    } else {
        0.
    };

    draw_rectangle_extended(
        pos.x,
        pos.y,
        dim.w,
        dim.h,
        style.corner_radius,
        color_1,
        color_2,
        gradient_angle,
        style.outline,
        style.outline_color,
    );
}
