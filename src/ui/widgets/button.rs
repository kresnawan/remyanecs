use crate::{
    component::UIColor, render_q::{UIRender, UIVisual}, table::UIElementTable, ui::UIElement, world::World,
};
use macroquad::prelude::*;

pub fn render_button(render_data: &UIRender) {
    if !render_data.visible {
        return;
    }

    let UIVisual::UIButton(config) = render_data.vis else {
        return;
    };

    let pos = render_data.global_pos;
    let dim = render_data.dim;

    let active_style = if render_data.is_hovered {
        config.hover_style.as_ref().unwrap_or(&config.style)
    } else {
        &config.style
    };

    let text_dimension = measure_text(&config.text, None, active_style.font_size, 1.0);
    let text_pos = (
        pos.x + dim.w / 2. - text_dimension.width / 2.,
        pos.y + text_dimension.height + dim.h / 2. - text_dimension.height / 2.,
    );

    if let UIColor::Fill(color) = active_style.bg_color {
        draw_rectangle(pos.x, pos.y, dim.w, dim.h, color);
    }

    draw_text(
        &config.text,
        text_pos.0,
        text_pos.1,
        active_style.font_size as f32,
        WHITE,
    );
}
