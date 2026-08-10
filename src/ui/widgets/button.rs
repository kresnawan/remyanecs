use crate::{
    component::{
        Button, ButtonStyle, Dimension, OnClickEvent, Parent, Position, PositionType, UIColor,
        Visible,
    },
    world::World,
};
use macroquad::prelude::*;

pub fn spawn_button(
    world: &mut World,
    pos: (Position, PositionType),
    dim: Dimension,
    style: ButtonStyle,
    parent: Option<Parent>,
    on_click_event: Option<OnClickEvent>,
) {
    world.spawn(
        pos.0,
        dim,
        pos.1,
        Some(style),
        Some(Button),
        parent,
        None,
        None,
        Some(Visible),
        on_click_event,
    );
}

pub fn system_draw_button(world: &World) {
    for entity in 0..world.next_entity {
        let is_visible = world.visible[entity].is_some();
        if !is_visible {
            continue;
        }

        if world.button[entity].is_some() {
            if let (Some(dim), Some(button_style), Some(pos)) = (
                &world.dimension[entity],
                &world.button_style[entity],
                &world.global_pos[entity],
            ) {
                let is_hovered = if let Some(he) = world.hovered_entity {
                    he == entity
                } else {
                    false
                };

                let active_style = if is_hovered {
                    button_style
                        .hover_style
                        .as_ref()
                        .unwrap_or(&button_style.style)
                } else {
                    &button_style.style
                };

                let text_dimension =
                    measure_text(&button_style.text, None, active_style.font_size, 1.0);
                let text_pos = (
                    pos.x + dim.w / 2. - text_dimension.width / 2.,
                    pos.y + text_dimension.height + dim.h / 2. - text_dimension.height / 2.,
                );

                if let UIColor::Fill(color) = active_style.bg_color {
                    draw_rectangle(pos.x, pos.y, dim.w, dim.h, color);
                }

                draw_text(
                    &button_style.text,
                    text_pos.0,
                    text_pos.1,
                    active_style.font_size as f32,
                    WHITE,
                );
            }
        }
    }
}
