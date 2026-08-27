pub mod button;
pub mod div;
pub mod switch;
pub mod text_input;

pub mod rectangle;
pub mod slot;
pub mod text;

use crate::{
    UIElementId,
    component::{
        Dimension, Display, GlobalPosition, OnClickEvent, Parent, Position, PositionType, ZIndex,
    },
    render_q::{UIRender, UIVisual},
    table::{
        button::UIButtonTable, div::UIDivTable, rectangle::UIRectangleTable, slot::UISlotTable,
        switch::UISwitchTable, text::UITextTable, text_input::UITextInputTable,
    },
};

#[derive(Debug)]
pub enum UIElementTable<T, U> {
    UIButtonTable(UIButtonTable<T, U>),
    UIDivTable(UIDivTable),
    UISwitchTable(UISwitchTable<T, U>),
    UISlotTable(UISlotTable),
    UITextTable(UITextTable<U>),
    UIRectangleTable(UIRectangleTable<U>),
    UITextInputTable(UITextInputTable<U>),
}

impl<T, U> UIElementTable<T, U> {
    pub fn as_text(&self) -> Option<&UITextTable<U>> {
        if let UIElementTable::UITextTable(table) = self {
            Some(table)
        } else {
            None
        }
    }

    pub fn is_dirty(&self) -> Option<&Vec<bool>> {
        match self {
            UIElementTable::UITextTable(table) => Some(&table.is_dirty),
            UIElementTable::UIButtonTable(table) => Some(&table.is_dirty),
            UIElementTable::UIDivTable(table) => Some(&table.is_dirty),

            _ => None,
        }
    }

    pub fn is_dirty_mut(&mut self) -> Option<&mut Vec<bool>> {
        match self {
            UIElementTable::UITextTable(table) => Some(&mut table.is_dirty),
            UIElementTable::UIButtonTable(table) => Some(&mut table.is_dirty),
            UIElementTable::UIDivTable(table) => Some(&mut table.is_dirty),

            _ => None,
        }
    }

    pub fn as_text_mut(&mut self) -> Option<&mut UITextTable<U>> {
        if let UIElementTable::UITextTable(table) = self {
            Some(table)
        } else {
            None
        }
    }

    pub fn render_data(&self, index: usize) -> Option<UIRender<'_, U>> {
        match self {
            UIElementTable::UIButtonTable(table) => Some(UIRender::new(
                self,
                UIVisual::UIButton(&table.button_config[index]),
                index,
            )),
            UIElementTable::UIDivTable(_) => None,
            UIElementTable::UISwitchTable(table) => Some(
                UIRender::new(self, UIVisual::UISwitch(&table.switch_config[index]), index)
                    .is_on(table.is_on[index]),
            ),
            UIElementTable::UIRectangleTable(table) => Some(UIRender::new(
                self,
                UIVisual::UIRectangle(&table.style[index]),
                index,
            )),
            UIElementTable::UISlotTable(table) => Some(UIRender::new(
                self,
                UIVisual::UISlot(&table.state[index]),
                index,
            )),
            UIElementTable::UITextTable(table) => Some(UIRender::new(
                self,
                UIVisual::UIText(
                    &table.style[index],
                    table.max_width[index],
                    &table.lines[index],
                ),
                index,
            )),
            UIElementTable::UITextInputTable(table) => Some(UIRender::new(
                self,
                UIVisual::UITextInput(&table.config[index], &table.value[index]),
                index,
            )),
        }
    }

    pub fn visible(&self) -> &Vec<bool> {
        match self {
            UIElementTable::UIDivTable(table) => &table.visible,
            UIElementTable::UIButtonTable(table) => &table.visible,
            UIElementTable::UISwitchTable(table) => &table.visible,
            UIElementTable::UISlotTable(table) => &table.visible,
            UIElementTable::UITextTable(table) => &table.visible,
            UIElementTable::UIRectangleTable(table) => &table.visible,
            UIElementTable::UITextInputTable(table) => &table.visible,
        }
    }

    pub fn visible_mut(&mut self) -> &mut Vec<bool> {
        match self {
            UIElementTable::UIDivTable(table) => &mut table.visible,
            UIElementTable::UIButtonTable(table) => &mut table.visible,
            UIElementTable::UISwitchTable(table) => &mut table.visible,
            UIElementTable::UISlotTable(table) => &mut table.visible,
            UIElementTable::UITextTable(table) => &mut table.visible,
            UIElementTable::UIRectangleTable(table) => &mut table.visible,
            UIElementTable::UITextInputTable(table) => &mut table.visible,
        }
    }

    pub fn on_click_event(&self) -> Option<&Vec<Option<OnClickEvent<T>>>> {
        match self {
            UIElementTable::UIButtonTable(table) => Some(&table.on_click_event),
            UIElementTable::UISwitchTable(table) => Some(&table.on_click_event),
            _ => None,
        }
    }

    pub fn z_index(&self) -> &Vec<ZIndex> {
        match self {
            UIElementTable::UIButtonTable(table) => &table.z_index,
            UIElementTable::UIDivTable(table) => &table.z_index,
            UIElementTable::UISwitchTable(table) => &table.z_index,
            UIElementTable::UISlotTable(table) => &table.z_index,
            UIElementTable::UITextTable(table) => &table.z_index,
            UIElementTable::UIRectangleTable(table) => &table.z_index,
            UIElementTable::UITextInputTable(table) => &table.z_index,
        }
    }

    pub fn position(&self) -> &Vec<Position> {
        match self {
            UIElementTable::UIButtonTable(table) => &table.position,
            UIElementTable::UIDivTable(table) => &table.position,
            UIElementTable::UISwitchTable(table) => &table.position,
            UIElementTable::UISlotTable(table) => &table.position,
            UIElementTable::UIRectangleTable(table) => &table.position,
            UIElementTable::UITextTable(table) => &table.position,
            UIElementTable::UITextInputTable(table) => &table.position,
        }
    }

    pub fn position_mut(&mut self) -> &mut Vec<Position> {
        match self {
            UIElementTable::UIButtonTable(table) => &mut table.position,
            UIElementTable::UIDivTable(table) => &mut table.position,
            UIElementTable::UISwitchTable(table) => &mut table.position,
            UIElementTable::UISlotTable(table) => &mut table.position,
            UIElementTable::UIRectangleTable(table) => &mut table.position,
            UIElementTable::UITextTable(table) => &mut table.position,
            UIElementTable::UITextInputTable(table) => &mut table.position,
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
            UIElementTable::UISwitchTable(table) => &table.global_pos,
            UIElementTable::UISlotTable(table) => &table.global_pos,
            UIElementTable::UIRectangleTable(table) => &table.global_pos,
            UIElementTable::UITextTable(table) => &table.global_pos,
            UIElementTable::UITextInputTable(table) => &table.global_pos,
        }
    }

    pub fn global_pos_mut(&mut self) -> &mut Vec<GlobalPosition> {
        match self {
            UIElementTable::UIButtonTable(table) => &mut table.global_pos,
            UIElementTable::UIDivTable(table) => &mut table.global_pos,
            UIElementTable::UISwitchTable(table) => &mut table.global_pos,
            UIElementTable::UISlotTable(table) => &mut table.global_pos,
            UIElementTable::UIRectangleTable(table) => &mut table.global_pos,
            UIElementTable::UITextTable(table) => &mut table.global_pos,
            UIElementTable::UITextInputTable(table) => &mut table.global_pos,
        }
    }

    pub fn pos_type(&self) -> &Vec<PositionType> {
        match self {
            UIElementTable::UIButtonTable(table) => &table.position_type,
            UIElementTable::UIDivTable(table) => &table.position_type,
            UIElementTable::UISwitchTable(table) => &table.position_type,
            UIElementTable::UISlotTable(table) => &table.position_type,
            UIElementTable::UITextTable(table) => &table.position_type,
            UIElementTable::UIRectangleTable(table) => &table.position_type,
            UIElementTable::UITextInputTable(table) => &table.position_type,
        }
    }

    pub fn dimension(&self) -> &Vec<Dimension> {
        match self {
            UIElementTable::UIButtonTable(table) => &table.dimension,
            UIElementTable::UIDivTable(table) => &table.dimension,
            UIElementTable::UISwitchTable(table) => &table.dimension,
            UIElementTable::UISlotTable(table) => &table.dimension,
            UIElementTable::UIRectangleTable(table) => &table.dimension,
            UIElementTable::UITextTable(table) => &table.dimension,
            UIElementTable::UITextInputTable(table) => &table.dimension,
        }
    }

    pub fn dimension_mut(&mut self) -> &mut Vec<Dimension> {
        match self {
            UIElementTable::UIButtonTable(table) => &mut table.dimension,
            UIElementTable::UIDivTable(table) => &mut table.dimension,
            UIElementTable::UISwitchTable(table) => &mut table.dimension,
            UIElementTable::UIRectangleTable(table) => &mut table.dimension,
            UIElementTable::UISlotTable(table) => &mut table.dimension,
            UIElementTable::UITextTable(table) => &mut table.dimension,
            UIElementTable::UITextInputTable(table) => &mut table.dimension,
        }
    }

    pub fn parent(&self) -> &Vec<Option<Parent>> {
        match self {
            UIElementTable::UIButtonTable(table) => &table.parent,
            UIElementTable::UIDivTable(table) => &table.parent,
            UIElementTable::UISwitchTable(table) => &table.parent,
            UIElementTable::UIRectangleTable(table) => &table.parent,
            UIElementTable::UITextTable(table) => &table.parent,
            UIElementTable::UISlotTable(table) => &table.parent,
            UIElementTable::UITextInputTable(table) => &table.parent,
        }
    }

    pub fn id(&self) -> &Vec<UIElementId> {
        match self {
            UIElementTable::UIButtonTable(table) => &table.ids,
            UIElementTable::UIDivTable(table) => &table.ids,
            UIElementTable::UISwitchTable(table) => &table.ids,
            UIElementTable::UIRectangleTable(table) => &table.ids,
            UIElementTable::UISlotTable(table) => &table.ids,
            UIElementTable::UITextTable(table) => &table.ids,
            UIElementTable::UITextInputTable(table) => &table.ids,
        }
    }
}
