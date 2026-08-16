use crate::{
    UIElementId,
    component::{
        Button, ButtonConfig, Dimension, Display, Div, GlobalPosition, OnClickEvent, Parent,
        Position, PositionType, Style,
    },
    table::{
        UIElementTable,
        button::UIButtonTable,
        div::UIDivTable,
        rectangle::UIRectangleTable,
        slot::{SlotIndex, SlotState, UISlotTable},
        switch::{SwitchConfig, UISwitchTable},
        text::UITextTable,
    },
    ui::{UIElement, UILocation},
};

pub struct World {
    pub next_id: UIElementId,
    pub next_z_index: u32,

    pub hoverable_elements: Vec<UIElement>,
    pub hovered_entity: Option<UIElementId>,

    pub ui_locations: Vec<UILocation>,
    pub ui_tables: Vec<UIElementTable>,
}

impl World {
    pub fn new() -> World {
        let ui_div_table = UIElementTable::UIDivTable(UIDivTable::new());
        let ui_button_table = UIElementTable::UIButtonTable(UIButtonTable::new());
        let ui_switch_table = UIElementTable::UISwitchTable(UISwitchTable::new());

        let ui_text_table = UIElementTable::UITextTable(UITextTable::new());
        let ui_slot_table = UIElementTable::UISlotTable(UISlotTable::new());
        let ui_rect_table = UIElementTable::UIRectangleTable(UIRectangleTable::new());

        let ui_tables = vec![
            ui_div_table,
            ui_button_table,
            ui_switch_table,
            ui_text_table,
            ui_slot_table,
            ui_rect_table,
        ];

        World {
            next_id: 0,
            next_z_index: 0,
            hoverable_elements: vec![UIElement::UIButton, UIElement::UISwitch],
            hovered_entity: None,
            ui_locations: Vec::new(),
            ui_tables,
        }
    }

    fn add_parent_child(&mut self, ui_id: UIElementId, parent_id: UIElementId) {
        let parent: &UILocation = &self.ui_locations[parent_id];
        let UIElementTable::UIDivTable(table) = &mut self.ui_tables[parent.table.t_index()] else {
            return;
        };

        table.childs[parent.index].push(ui_id);
    }

    pub fn spawn_button(
        &mut self,
        pos: (Position, PositionType),
        dim: Dimension,
        config: ButtonConfig,
        parent: Option<Parent>,
        on_click_event: Option<OnClickEvent>,
    ) -> UIElementId {
        let current_id = self.next_id.clone();
        let current_z = self.next_z_index.clone();

        if let UIElementTable::UIButtonTable(table) =
            &mut self.ui_tables[UIElement::UIButton.t_index()]
        {
            self.ui_locations.push(UILocation {
                table: UIElement::UIButton,
                index: table.ids.len(),
            });

            table.button.push(Button);
            table.button_config.push(config);
            table.dimension.push(dim);
            table.disabled.push(false);
            table.global_pos.push(GlobalPosition {
                x: pos.0.x,
                y: pos.0.y,
            });
            table.ids.push(current_id);
            table.on_click_event.push(on_click_event);
            table.parent.push(parent);
            table.position.push(pos.0);
            table.position_type.push(pos.1);
            table.visible.push(true);
            table.z_index.push(current_z);
        }

        if let Some(parent) = parent {
            self.add_parent_child(current_id, parent);
        }

        self.next_id += 1;
        self.next_z_index += 1;

        return current_id;
    }

    pub fn spawn_div(
        &mut self,
        pos: (Position, PositionType),
        dim: Dimension,
        display: Display,
        parent: Option<Parent>,
    ) -> UIElementId {
        let current_id = self.next_id;
        let current_z = self.next_z_index;

        if let UIElementTable::UIDivTable(table) = &mut self.ui_tables[UIElement::UIDiv.t_index()] {
            self.ui_locations.push(UILocation {
                table: UIElement::UIDiv,
                index: table.ids.len(),
            });

            table.ids.push(current_id);
            table.global_pos.push(GlobalPosition {
                x: pos.0.x,
                y: pos.0.y,
            });
            table.position.push(pos.0);
            table.position_type.push(pos.1);

            table.visible.push(true);
            table.z_index.push(current_z);

            table.dimension.push(dim);
            table.parent.push(parent);

            table.div.push(Div);
            table.display.push(display);
            table.childs.push(Vec::new());
        }

        if let Some(parent) = parent {
            self.add_parent_child(current_id, parent);
        }

        self.next_id += 1;
        self.next_z_index += 1;

        return current_id;
    }

    pub fn spawn_switch(
        &mut self,
        pos: (Position, PositionType),
        dim: Dimension,
        parent: Option<Parent>,
        on_click_event: Option<OnClickEvent>,
        switch_config: SwitchConfig,
    ) -> UIElementId {
        let current_id = self.next_id.clone();
        let current_z = self.next_z_index.clone();

        if let UIElementTable::UISwitchTable(table) =
            &mut self.ui_tables[UIElement::UISwitch.t_index()]
        {
            self.ui_locations.push(UILocation {
                table: UIElement::UIDiv,
                index: table.ids.len(),
            });

            table.ids.push(current_id);
            table.global_pos.push(GlobalPosition {
                x: pos.0.x,
                y: pos.0.y,
            });
            table.position.push(pos.0);
            table.position_type.push(pos.1);

            table.visible.push(true);
            table.z_index.push(current_z);
            table.disabled.push(false);

            table.dimension.push(dim);
            table.parent.push(parent);

            table.is_on.push(false);
            table.on_click_event.push(on_click_event);
            table.switch_config.push(switch_config);
        }

        if let Some(parent) = parent {
            self.add_parent_child(current_id, parent);
        }

        self.next_id += 1;
        self.next_z_index += 1;

        return current_id;
    }

    pub fn spawn_text(
        &mut self,
        position: (Position, PositionType),
        value: &str,
        style: Style,
        max_width: Option<f32>,
        parent: Option<Parent>,
    ) -> UIElementId {
        let current_id = self.next_id.clone();
        let current_z = self.next_z_index.clone();

        if let UIElementTable::UITextTable(table) = &mut self.ui_tables[UIElement::UIText.t_index()]
        {
            self.ui_locations.push(UILocation {
                table: UIElement::UIText,
                index: table.ids.len(),
            });

            table.ids.push(current_id);
            table.global_pos.push(GlobalPosition {
                x: position.0.x,
                y: position.0.y,
            });
            table.position.push(position.0);
            table.z_index.push(current_z);
            table.position_type.push(position.1);
            table.dimension.push(Dimension::new());
            table.parent.push(parent);
            table.visible.push(true);
            table.max_width.push(max_width);
            table.style.push(style);
            table.value.push(value.to_string());
        }

        if let Some(parent) = parent {
            self.add_parent_child(current_id, parent);
        }

        self.next_id += 1;
        self.next_z_index += 1;

        return current_id;
    }

    pub fn spawn_rectangle(
        &mut self,
        pos: (Position, PositionType),
        dim: Dimension,
        style: Style,
        parent: Option<Parent>,
    ) -> UIElementId {
        let current_id = self.next_id.clone();
        let current_z = self.next_z_index.clone();

        if let UIElementTable::UIRectangleTable(table) =
            &mut self.ui_tables[UIElement::UIRectangle.t_index()]
        {
            self.ui_locations.push(UILocation {
                table: UIElement::UIRectangle,
                index: table.ids.len(),
            });

            table.ids.push(current_id);
            table.global_pos.push(GlobalPosition {
                x: pos.0.x,
                y: pos.0.y,
            });
            table.position.push(pos.0);
            table.z_index.push(current_z);
            table.position_type.push(pos.1);
            table.dimension.push(dim);
            table.parent.push(parent);
            table.visible.push(true);
            table.style.push(style);
        }

        if let Some(parent) = parent {
            self.add_parent_child(current_id, parent);
        }

        self.next_id += 1;
        self.next_z_index += 1;

        return current_id;
    }

    pub fn spawn_slot(
        &mut self,
        pos: (Position, PositionType),
        dim: Dimension,
        index: SlotIndex,
        parent: Option<UIElementId>,
    ) -> UIElementId {
        let current_id = self.next_id.clone();
        let current_z = self.next_z_index.clone();

        if let UIElementTable::UISlotTable(table) = &mut self.ui_tables[UIElement::UISlot.t_index()]
        {
            self.ui_locations.push(UILocation {
                table: UIElement::UISlot,
                index: table.ids.len(),
            });

            table.ids.push(current_id);
            table.global_pos.push(GlobalPosition {
                x: pos.0.x,
                y: pos.0.y,
            });
            table.position.push(pos.0);
            table.z_index.push(current_z);
            table.position_type.push(pos.1);
            table.dimension.push(dim);
            table.parent.push(parent);
            table.visible.push(true);
            table.state.push(SlotState { player: None });
            table.index.push(index);
        }

        if let Some(parent) = parent {
            self.add_parent_child(current_id, parent);
        }

        self.next_id += 1;
        self.next_z_index += 1;

        return current_id;
    }
}
