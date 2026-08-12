pub mod button;
pub mod div;

use crate::{
    UIElementId,
    component::{
        Dimension, Display, GlobalPosition, OnClickEvent, Parent, Position, PositionType, ZIndex,
    },
    render_q::{UIRender, UIVisual},
    table::{button::UIButtonTable, div::UIDivTable},
};

#[derive(Debug)]
pub enum UIElementTable {
    UIButtonTable(UIButtonTable),
    UIDivTable(UIDivTable),
}

impl UIElementTable {
    pub fn render_data(&self, index: usize) -> Option<UIRender<'_>> {
        match self {
            UIElementTable::UIButtonTable(table) => Some(UIRender {
                element_id: table.ids[index],
                global_pos: &table.global_pos[index],
                dim: &table.dimension[index],
                z_index: table.z_index[index],
                is_hovered: false,
                is_pressed: false,
                visible: table.visible[index],
                vis: UIVisual::UIButton(&table.button_config[index]),
            }),
            _ => None,
        }
    }
    pub fn on_click_event(&self) -> Option<&Vec<Option<OnClickEvent>>> {
        match self {
            UIElementTable::UIButtonTable(table) => Some(&table.on_click_event),
            _ => None,
        }
    }
    pub fn z_index(&self) -> &Vec<ZIndex> {
        match self {
            UIElementTable::UIButtonTable(table) => &table.z_index,
            UIElementTable::UIDivTable(table) => &table.z_index,
        }
    }
    pub fn position(&self) -> &Vec<Position> {
        match self {
            UIElementTable::UIButtonTable(table) => &table.position,

            UIElementTable::UIDivTable(table) => &table.position,
        }
    }

    pub fn position_mut(&mut self) -> &mut Vec<Position> {
        match self {
            UIElementTable::UIButtonTable(table) => &mut table.position,

            UIElementTable::UIDivTable(table) => &mut table.position,
        }
    }

    pub fn display(&self) -> Option<&Vec<Display>> {
        if let UIElementTable::UIDivTable(table) = self {
            Some(&table.display)
        } else {
            None
        }
    }

    pub fn childs(&self) -> Option<&Vec<Vec<UIElementId>>> {
        if let UIElementTable::UIDivTable(table) = self {
            Some(&table.childs)
        } else {
            None
        }
    }

    pub fn global_pos(&self) -> &Vec<GlobalPosition> {
        match self {
            UIElementTable::UIButtonTable(table) => &table.global_pos,

            UIElementTable::UIDivTable(table) => &table.global_pos,
        }
    }

    pub fn global_pos_mut(&mut self) -> &mut Vec<GlobalPosition> {
        match self {
            UIElementTable::UIButtonTable(table) => &mut table.global_pos,

            UIElementTable::UIDivTable(table) => &mut table.global_pos,
        }
    }

    pub fn pos_type(&self) -> &Vec<PositionType> {
        match self {
            UIElementTable::UIButtonTable(table) => &table.position_type,

            UIElementTable::UIDivTable(table) => &table.position_type,
        }
    }

    pub fn dimension(&self) -> &Vec<Dimension> {
        match self {
            UIElementTable::UIButtonTable(table) => &table.dimension,

            UIElementTable::UIDivTable(table) => &table.dimension,
        }
    }

    pub fn dimension_mut(&mut self) -> &mut Vec<Dimension> {
        match self {
            UIElementTable::UIButtonTable(table) => &mut table.dimension,

            UIElementTable::UIDivTable(table) => &mut table.dimension,
        }
    }

    pub fn parent(&self) -> &Vec<Option<Parent>> {
        match self {
            UIElementTable::UIButtonTable(table) => &table.parent,

            UIElementTable::UIDivTable(table) => &table.parent,
        }
    }

    pub fn id(&self) -> &Vec<UIElementId> {
        match self {
            UIElementTable::UIButtonTable(table) => &table.ids,

            UIElementTable::UIDivTable(table) => &table.ids,
        }
    }
}
