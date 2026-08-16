use macroquad::ui::Style;

use crate::{
    UIElementId,
    component::{Dimension, Div, GlobalPosition, Parent, Position, PositionType, ZIndex}, render_q::UIRender,
};

#[derive(Debug)]
pub struct DialogueConfig {
    bg_opacity: f32,
    style: Style,
}

#[derive(Debug)]
pub struct Dialogue;

#[derive(Debug)]
pub struct UIDialogueTable {
    pub ids: Vec<UIElementId>,
    pub position: Vec<Position>,
    pub z_index: Vec<ZIndex>,
    pub global_pos: Vec<GlobalPosition>,
    pub position_type: Vec<PositionType>,
    pub dimension: Vec<Dimension>,
    pub parent: Vec<Option<Parent>>,

    pub visible: Vec<bool>,
    pub div: Vec<Div>,
    pub childs: Vec<Vec<usize>>,

    pub config: Vec<DialogueConfig>,
    pub dialogue: Vec<Dialogue>,
}

impl UIDialogueTable {
    pub fn new() -> UIDialogueTable {
        UIDialogueTable {
            ids: Vec::new(),
            position: Vec::new(),
            z_index: Vec::new(),
            global_pos: Vec::new(),
            position_type: Vec::new(),
            dimension: Vec::new(),
            parent: Vec::new(),

            visible: Vec::new(),
            div: Vec::new(),
            childs: Vec::new(),

            config: Vec::new(),
            dialogue: Vec::new(),
        }
    }
}

pub fn render_dialogue(render_data: &UIRender) {
    
}
