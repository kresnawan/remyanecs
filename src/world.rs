use std::fmt::Debug;
use std::sync::Arc;

use macroquad::color::Color;

use crate::{
    FontRegistry, UIElementId,
    component::{
        Button, ButtonConfig, Dimension, Display, Div, DynDim, GlobalPosition, OnClickEvent,
        Parent, Position, PositionType, Style, UIColor,
    },
    render_q::render,
    table::{
        UIElementTable,
        button::UIButtonTable,
        div::UIDivTable,
        rectangle::UIRectangleTable,
        slot::{SlotIndex, SlotState, UISlotTable},
        switch::{SwitchConfig, UISwitchTable},
        text::UITextTable,
        text_input::{TextInputConfig, UITextInputTable},
    },
    ui::{UIElement, UILocation},
};

use crate::system::{
    system_arrange_text, system_dialogue_visibility, system_dirty_state, system_dynamic_transform,
    system_hover, system_on_click, system_parent_display, system_text_dimension, system_text_input,
    system_transform, system_visible,
};

pub struct World<T> {
    pub next_id: UIElementId,
    pub next_z_index: u32,

    pub hoverable_elements: Vec<UIElement>,

    pub hovered_entity: Option<UIElementId>,
    pub focused_entity: Option<UIElementId>,
    pub opened_dialogue: Option<UIElementId>,

    pub font_registry: Arc<FontRegistry>,
    pub initialized: bool,

    pub current_screen_size: (f32, f32),
    pub is_updated: bool,

    pub ui_locations: Vec<UILocation>,
    pub ui_tables: Vec<UIElementTable<T>>,
}

impl<T> World<T> {
    pub fn new(font_registry: Arc<FontRegistry>) -> World<T> {
        let ui_div_table = UIElementTable::UIDivTable(UIDivTable::new());
        let ui_button_table = UIElementTable::UIButtonTable(UIButtonTable::new());
        let ui_switch_table = UIElementTable::UISwitchTable(UISwitchTable::new());

        let ui_text_table = UIElementTable::UITextTable(UITextTable::new());
        let ui_slot_table = UIElementTable::UISlotTable(UISlotTable::new());
        let ui_rect_table = UIElementTable::UIRectangleTable(UIRectangleTable::new());

        let ui_text_input_table = UIElementTable::UITextInputTable(UITextInputTable::new());

        let ui_tables = vec![
            ui_div_table,
            ui_button_table,
            ui_switch_table,
            ui_text_table,
            ui_slot_table,
            ui_rect_table,
            ui_text_input_table,
        ];

        World {
            next_id: 0,
            next_z_index: 0,
            hoverable_elements: vec![
                UIElement::UIButton,
                UIElement::UISwitch,
                UIElement::UITextInput,
                UIElement::UIRectangle,
            ],
            font_registry,
            initialized: false,
            current_screen_size: (0., 0.),
            is_updated: false,
            hovered_entity: None,
            focused_entity: None,
            opened_dialogue: None,
            ui_locations: Vec::new(),
            ui_tables,
        }
    }

    pub fn open_dialogue(&mut self, entity: UIElementId) {
        let location = &self.ui_locations[entity];
        if let UIElementTable::UIDivTable(table) = &self.ui_tables[location.table.t_index()] {
            if table.dialogue[location.index] {
                self.opened_dialogue = Some(entity);
            }
        }
    }

    pub fn close_dialogue(&mut self) {
        self.opened_dialogue = None;
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
        on_click_event: Option<OnClickEvent<T>>,
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
            table.is_dirty.push(true);
        }

        if let Some(parent) = parent {
            self.add_parent_child(current_id, parent);
        }

        self.next_id += 1;
        self.next_z_index += 1;

        return current_id;
    }

    fn spawn_dialogue_div(&mut self) -> UIElementId {
        let current_id = self.next_id;
        let current_z = self.next_z_index;

        if let UIElementTable::UIDivTable(table) = &mut self.ui_tables[UIElement::UIDiv.t_index()] {
            self.ui_locations.push(UILocation {
                table: UIElement::UIDiv,
                index: table.ids.len(),
            });

            table.ids.push(current_id);
            table.global_pos.push(GlobalPosition { x: 0., y: 0. });
            table.position.push(Position::center());
            table.position_type.push(PositionType::Relative);

            table.visible.push(false);
            table.z_index.push(current_z);

            table
                .dimension
                .push(Dimension::new().dyn_h(DynDim::Full).dyn_w(DynDim::Full));
            table.parent.push(None);

            table.div.push(Div);
            table.display.push(Display::Normal);
            table.childs.push(Vec::new());
            table.is_dirty.push(true);
            table.dialogue.push(true);
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
        visible: bool,
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

            table.visible.push(visible);
            table.z_index.push(current_z);

            table.dimension.push(dim);
            table.parent.push(parent);

            table.div.push(Div);
            table.display.push(display);
            table.childs.push(Vec::new());
            table.is_dirty.push(true);
            table.dialogue.push(false);
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
        on_click_event: Option<OnClickEvent<T>>,
        switch_config: SwitchConfig,
    ) -> UIElementId {
        let current_id = self.next_id.clone();
        let current_z = self.next_z_index.clone();

        if let UIElementTable::UISwitchTable(table) =
            &mut self.ui_tables[UIElement::UISwitch.t_index()]
        {
            self.ui_locations.push(UILocation {
                table: UIElement::UISwitch,
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
            table.lines.push(Vec::new());
            table.is_dirty.push(true);
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

    pub fn spawn_text_input(
        &mut self,
        pos: (Position, PositionType),
        dim: Dimension,
        config: TextInputConfig,
        max_length: Option<usize>,
        parent: Option<UIElementId>,
    ) -> UIElementId {
        let current_id = self.next_id.clone();
        let current_z = self.next_z_index.clone();

        if let UIElementTable::UITextInputTable(table) =
            &mut self.ui_tables[UIElement::UITextInput.t_index()]
        {
            self.ui_locations.push(UILocation {
                table: UIElement::UITextInput,
                index: table.ids.len(),
            });

            table.ids.push(current_id);
            table.global_pos.push(GlobalPosition {
                x: pos.0.x,
                y: pos.0.x,
            });
            table.position.push(pos.0);
            table.z_index.push(current_z);
            table.position_type.push(pos.1);
            table.dimension.push(dim);
            table.parent.push(parent);
            table.visible.push(true);
            table.max_length.push(max_length);
            table.config.push(config);
            table.value.push(String::new());
            table.is_dirty.push(true);
        }

        if let Some(parent) = parent {
            self.add_parent_child(current_id, parent);
        }

        self.next_id += 1;
        self.next_z_index += 1;

        return current_id;
    }

    pub fn spawn_dialogue_box(
        &mut self,
        dim: Dimension,
        style: Style,
    ) -> (UIElementId, UIElementId) {
        let container = self.spawn_dialogue_div();

        self.spawn_rectangle(
            (Position::center(), PositionType::Relative),
            Dimension::new().dyn_h(DynDim::Full).dyn_w(DynDim::Full),
            Style {
                bg_color: UIColor::Fill(Color::from_rgba(0, 0, 0, 127)),
                ..Default::default()
            },
            Some(container),
        );

        let inner_container = self.spawn_div(
            (Position::center(), PositionType::Relative),
            dim,
            Display::Normal,
            true,
            Some(container),
        );

        self.spawn_rectangle(
            (Position::center(), PositionType::Relative),
            Dimension::new().dyn_h(DynDim::Full).dyn_w(DynDim::Full),
            style,
            Some(inner_container),
        );

        (container, inner_container)
    }

    pub fn set_button_event(&mut self, entity: UIElementId, event: T) {
        let location = &self.ui_locations[entity];
        if let UIElementTable::UIButtonTable(table) = &mut self.ui_tables[location.table.t_index()]
        {
            table.on_click_event[location.index] = Some(OnClickEvent(event));
        }
    }
}

impl<T: Debug + Clone> World<T> {
    pub fn update(&mut self, ui_events: &mut Vec<T>) {
        if !self.initialized {
            //
            // Any task on init goes here

            self.initialized = true;
        }

        system_visible(self);
        system_dialogue_visibility(self);

        system_dirty_state(self);

        system_arrange_text(self);
        system_text_dimension(self);

        system_dynamic_transform(self);
        system_parent_display(self);
        system_transform(self);

        system_hover(self);

        system_on_click(self, ui_events);

        system_text_input(self);
    }

    pub fn render(&self) {
        render(self);
    }
}
