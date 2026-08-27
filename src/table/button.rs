use crate::{
    UIElementId,
    component::{
        Button, ButtonConfig, Dimension, GlobalPosition, OnClickEvent, Parent, Position,
        PositionType, Style, UIColor, ZIndex,
    },
    helper::draw_rectangle_extended,
    render_q::{UIRender, UIVisual},
    world::World,
};

use macroquad::prelude::*;

#[derive(Debug)]
pub struct UIButtonTable<T> {
    pub ids: Vec<UIElementId>,
    pub position: Vec<Position>,
    pub z_index: Vec<ZIndex>,
    pub global_pos: Vec<GlobalPosition>,
    pub position_type: Vec<PositionType>,
    pub dimension: Vec<Dimension>,
    pub parent: Vec<Option<Parent>>,

    pub on_click_event: Vec<Option<OnClickEvent<T>>>,
    pub disabled: Vec<bool>,
    pub visible: Vec<bool>,
    pub button_config: Vec<ButtonConfig>,
    pub button: Vec<Button>,

    pub is_dirty: Vec<bool>,
}

impl<T> UIButtonTable<T> {
    pub fn new() -> UIButtonTable<T> {
        UIButtonTable {
            ids: Vec::new(),
            position: Vec::new(),
            z_index: Vec::new(),
            global_pos: Vec::new(),
            position_type: Vec::new(),
            dimension: Vec::new(),
            parent: Vec::new(),
            on_click_event: Vec::new(),
            disabled: Vec::new(),
            visible: Vec::new(),
            button_config: Vec::new(),
            button: Vec::new(),

            is_dirty: Vec::new(),
        }
    }
}

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

pub fn spawn_std_button<T>(
    world: &mut World<T>,
    pos: Position,
    dim: Dimension,
    pos_type: PositionType,
    text: &str,
    parent: Option<Parent>,
    on_click: Option<OnClickEvent<T>>,
) -> UIElementId {
    world.spawn_button(
        (pos, pos_type),
        dim,
        ButtonConfig {
            text: text.to_owned(),
            style: Style {
                bg_color: UIColor::Fill(GREEN),
                font_size: 36,
                corner_radius: 5.,
                ..Default::default()
            },
            hover_style: Some(Style {
                bg_color: UIColor::Fill(PURPLE),
                font_size: 36,
                corner_radius: 5.,
                ..Default::default()
            }),
        },
        parent,
        on_click,
    )
}
