use crate::{
    UIElementId,
    component::{Dimension, GlobalPosition, Parent, Position, PositionType, ZIndex},
    helper::draw_rectangle_extended,
    render_q::{UIRender, UIVisual},
};

use macroquad::prelude::*;

#[derive(Debug)]
pub struct SlotState {
    pub player: Option<u32>,
}

#[derive(Debug)]
pub enum SlotIndex {
    One, Two, Three, Four
}

impl SlotIndex {
    pub fn as_index(&self) -> usize {
        match self {
            SlotIndex::One => 0,
            SlotIndex::Two => 1,
            SlotIndex::Three => 2,
            SlotIndex::Four => 3,
        }
    }
}

#[derive(Debug)]
pub struct UISlotTable {
    pub ids: Vec<UIElementId>,
    pub position: Vec<Position>,
    pub z_index: Vec<ZIndex>,
    pub global_pos: Vec<GlobalPosition>,
    pub position_type: Vec<PositionType>,
    pub dimension: Vec<Dimension>,
    pub parent: Vec<Option<Parent>>,
    pub visible: Vec<bool>,

    pub state: Vec<SlotState>,
    pub index: Vec<SlotIndex>
}

impl UISlotTable {
    pub fn new() -> UISlotTable {
        UISlotTable {
            ids: Vec::new(),
            position: Vec::new(),
            z_index: Vec::new(),
            global_pos: Vec::new(),
            position_type: Vec::new(),
            dimension: Vec::new(),
            parent: Vec::new(),
            state: Vec::new(),
            visible: Vec::new(),
            index: Vec::new()
        }
    }
}

pub fn render_slot(render_data: &UIRender) {
    let UIVisual::UISlot(config) = render_data.vis else {
        return;
    };

    let pos = render_data.global_pos;
    let dim = render_data.dim;

    if let Some(_player) = config.player {
    } else {
        draw_rectangle_extended(
            pos.x,
            pos.y,
            dim.w,
            dim.h,
            5.,
            WHITE,
            WHITE,
            0.,
            2.,
            Color::from_rgba(255, 255, 255, 255 / 2),
        );
    }
}
