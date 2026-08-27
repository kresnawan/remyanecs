use std::{hash::Hash, sync::Arc};

use crate::{
    FontRegistry, UIElementId,
    component::{Dimension, GlobalPosition, Parent, Position, PositionType, Style, ZIndex},
    helper::{draw_rectangle_extended, draw_text_extended},
    render_q::{UIRender, UIVisual},
};

use macroquad::prelude::*;

#[derive(Debug)]
pub struct TextInputConfig<U> {
    pub on_focus_style: Style<U>,
    pub style: Style<U>,
    pub on_hover_style: Option<Style<U>>,
}

#[derive(Debug)]
pub struct UITextInputTable<U> {
    pub ids: Vec<UIElementId>,
    pub position: Vec<Position>,
    pub z_index: Vec<ZIndex>,
    pub global_pos: Vec<GlobalPosition>,
    pub position_type: Vec<PositionType>,
    pub dimension: Vec<Dimension>,
    pub parent: Vec<Option<Parent>>,

    pub visible: Vec<bool>,
    pub max_length: Vec<Option<usize>>,
    pub config: Vec<TextInputConfig<U>>,
    pub value: Vec<String>,

    pub is_dirty: Vec<bool>,
}

impl<U> UITextInputTable<U> {
    pub fn new() -> UITextInputTable<U> {
        UITextInputTable {
            ids: Vec::new(),
            position: Vec::new(),
            z_index: Vec::new(),
            global_pos: Vec::new(),
            position_type: Vec::new(),
            dimension: Vec::new(),
            parent: Vec::new(),
            visible: Vec::new(),
            max_length: Vec::new(),
            config: Vec::new(),
            value: Vec::new(),
            is_dirty: Vec::new(),
        }
    }
}

pub fn render_text_input<U>(render_data: &UIRender<U>, font: Arc<FontRegistry<U>>)
where
    U: Eq + Hash,
{
    if let UIVisual::UITextInput(a, b) = render_data.vis {
        let used_style = if render_data.is_focused {
            &a.on_focus_style
        } else {
            if render_data.is_hovered {
                &a.on_hover_style.as_ref().unwrap_or(&a.style)
            } else {
                &a.style
            }
        };

        let pos = render_data.global_pos;
        let dim = render_data.dim;

        draw_rectangle_extended(
            pos.x,
            pos.y,
            dim.w,
            dim.h,
            used_style.corner_radius,
            used_style.bg_color.as_fill(),
            used_style.bg_color.as_fill(),
            0.,
            used_style.outline,
            used_style.outline_color,
        );

        let text_dimension = measure_text(
            b,
            font.fonts.get(&used_style.font),
            used_style.font_size,
            1.,
        );

        let text_pos = (
            pos.x,
            pos.y + text_dimension.height + dim.h / 2. - text_dimension.height / 2.,
        );

        draw_text_extended(
            b,
            text_pos.0,
            text_pos.1,
            None,
            TextParams {
                font: font.fonts.get(&used_style.font),
                font_size: used_style.font_size,
                color: used_style.color.as_fill(),
                ..Default::default()
            },
        );
    }
}
