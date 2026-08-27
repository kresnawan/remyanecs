use crate::{
    UIElementId,
    component::{Dimension, Display, Div, GlobalPosition, Parent, Position, PositionType, ZIndex},
};

#[derive(Debug)]
pub struct UIDivTable {
    pub ids: Vec<UIElementId>,
    pub position: Vec<Position>,
    pub z_index: Vec<ZIndex>,
    pub global_pos: Vec<GlobalPosition>,
    pub position_type: Vec<PositionType>,
    pub dimension: Vec<Dimension>,
    pub parent: Vec<Option<Parent>>,
    
    pub visible: Vec<bool>,
    pub div: Vec<Div>,
    pub display: Vec<Display>,
    pub childs: Vec<Vec<usize>>,
    pub is_dirty: Vec<bool>,
    pub dialogue: Vec<bool>,
}

impl UIDivTable {
    pub fn new() -> UIDivTable {
        UIDivTable {
            ids: Vec::new(),
            position: Vec::new(),
            z_index: Vec::new(),
            global_pos: Vec::new(),
            position_type: Vec::new(),
            dimension: Vec::new(),
            parent: Vec::new(),
            visible: Vec::new(),
            div: Vec::new(),
            display: Vec::new(),
            childs: Vec::new(),
            is_dirty: Vec::new(),
            dialogue: Vec::new()
        }
    }
}
