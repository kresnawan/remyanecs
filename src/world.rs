use crate::{
    UIElementId,
    component::{
        Button, ButtonConfig, Dimension, Display, Div, GlobalPosition, OnClickEvent, Parent,
        Position, PositionType,
    },
    table::{UIElementTable, button::UIButtonTable, div::UIDivTable},
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
        let ui_button_table = UIElementTable::UIButtonTable(UIButtonTable::new());
        let ui_div_table = UIElementTable::UIDivTable(UIDivTable::new());
        let ui_tables = vec![ui_div_table, ui_button_table];

        World {
            next_id: 0,
            next_z_index: 0,
            hoverable_elements: vec![UIElement::UIButton],
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
        let current_id = self.next_id.clone();
        let current_z = self.next_z_index.clone();

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

            println!("{:#?}", self.ui_locations);
        }

        if let Some(parent) = parent {
            self.add_parent_child(current_id, parent);
        }

        self.next_id += 1;
        self.next_z_index += 1;

        return current_id;
    }
}
