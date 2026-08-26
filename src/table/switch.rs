use crate::{
    UIElementId,
    component::{
        Dimension, GlobalPosition, OnClickEvent, Parent, Position, PositionType, Style, UIColor,
        ZIndex,
    },
    helper::draw_rectangle_extended,
    render_q::{UIRender, UIVisual},
    world::World,
};

use macroquad::prelude::*;

#[derive(Debug)]
pub struct SwitchConfig {
    style: Style,
    hover_style: Option<Style>,
}

#[derive(Debug)]
pub struct UISwitchTable {
    pub ids: Vec<UIElementId>,
    pub position: Vec<Position>,
    pub z_index: Vec<ZIndex>,
    pub global_pos: Vec<GlobalPosition>,
    pub position_type: Vec<PositionType>,
    pub dimension: Vec<Dimension>,
    pub parent: Vec<Option<Parent>>,

    pub on_click_event: Vec<Option<OnClickEvent>>,
    pub disabled: Vec<bool>,
    pub visible: Vec<bool>,
    pub switch_config: Vec<SwitchConfig>,
    pub is_on: Vec<bool>,
}

impl UISwitchTable {
    pub fn new() -> UISwitchTable {
        UISwitchTable {
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
            switch_config: Vec::new(),
            is_on: Vec::new(),
        }
    }
}

pub fn render_switch(render_data: &UIRender) {
    let UIVisual::UISwitch(config) = render_data.vis else {
        return;
    };

    let active_style = if render_data.is_hovered {
        config.hover_style.as_ref().unwrap_or(&config.style)
    } else {
        &config.style
    };

    let pos = render_data.global_pos;
    let dim = render_data.dim;

    draw_rectangle_extended(
        pos.x,
        pos.y,
        dim.w,
        dim.h,
        active_style.corner_radius,
        active_style.bg_color.as_fill(),
        active_style.bg_color.as_fill(),
        0.,
        active_style.outline,
        active_style.outline_color,
    );

    let switch_dim = (dim.w / 2., dim.h);

    let switch_pos = if render_data.is_on {
        (dim.w / 2. + pos.x, pos.y)
    } else {
        (pos.x, pos.y)
    };

    let mut switch_color = if render_data.is_on {
        active_style.color.as_fill()
    } else {
        GRAY
    };

    if render_data.is_disabled {
        switch_color.a = 0.5;
    }

    draw_rectangle_extended(
        switch_pos.0,
        switch_pos.1,
        switch_dim.0,
        switch_dim.1,
        active_style.corner_radius,
        switch_color,
        switch_color,
        0.,
        0.,
        active_style.outline_color,
    );
}

pub fn spawn_std_switch(
    world: &mut World,
    pos: Position,
    dim: Dimension,
    pos_type: PositionType,
    parent: Option<UIElementId>,
    on_click: Option<OnClickEvent>,
) -> UIElementId {
    world.spawn_switch(
        (pos, pos_type),
        dim,
        parent,
        on_click,
        SwitchConfig {
            style: Style {
                bg_color: UIColor::Fill(DARKGRAY),
                color: UIColor::Fill(ORANGE),
                corner_radius: 5.,
                ..Default::default()
            },

            hover_style: None,
        },
    )
}
