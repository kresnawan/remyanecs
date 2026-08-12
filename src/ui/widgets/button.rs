use crate::{
    UIElementId,
    component::{
        ButtonConfig, Dimension, OnClickEvent, Parent, Position, PositionType, Style, UIColor,
    },
    helper::draw_rectangle_extended,
    render_q::{UIRender, UIVisual},
    world::World,
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

    match &active_style.bg_color {
        UIColor::Fill(color) => {
            draw_rectangle_extended(
                pos.x,
                pos.y,
                dim.w,
                dim.h,
                active_style.corner_radius,
                *color,
                *color,
                0.,
                active_style.outline,
                active_style.outline_color,
            );
        }
        UIColor::Gradient(colors) => {
            draw_rectangle_extended(
                pos.x,
                pos.y,
                dim.w,
                dim.h,
                active_style.corner_radius,
                colors.colors[0],
                colors.colors[1],
                colors.angle,
                active_style.outline,
                active_style.outline_color,
            );
        }
    }

    draw_text(
        &config.text,
        text_pos.0,
        text_pos.1,
        active_style.font_size as f32,
        WHITE,
    );
}

pub fn spawn_std_button(
    world: &mut World,
    pos: Position,
    dim: Dimension,
    pos_type: PositionType,
    text: &str,
    parent: Option<Parent>,
    on_click: Option<OnClickEvent>,
) -> UIElementId {
    world.spawn_button(
        (pos, pos_type),
        dim,
        ButtonConfig {
            text: text.to_owned(),
            style: Style::new(
                UIColor::Fill(GREEN),
                UIColor::Fill(WHITE),
                1,
                36,
                0.,
                BLANK,
                5.,
            ),
            hover_style: Some(Style::new(
                UIColor::Fill(PURPLE),
                UIColor::Fill(WHITE),
                1,
                36,
                0.,
                BLANK,
                5.,
            )),
        },
        parent,
        on_click,
    )
}
