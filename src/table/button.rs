use crate::{
    UIElementId,
    component::{
        Button, ButtonConfig, Dimension, GlobalPosition, OnClickEvent, Parent, Position,
        PositionType, ZIndex,
    },
};

#[derive(Debug)]
pub struct UIButtonTable {
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
    pub button_config: Vec<ButtonConfig>,
    pub button: Vec<Button>,
}

impl UIButtonTable {
    pub fn new() -> UIButtonTable {
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
        }
    }
}
