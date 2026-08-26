use macroquad::{
    input::{MouseButton, is_mouse_button_down}, text::draw_text_ex, time::draw_fps
};

use crate::{
    UIElementId,
    component::{ButtonConfig, Dimension, GlobalPosition, Style},
    table::{
        UIElementTable,
        button::render_button,
        rectangle::render_rectangle,
        slot::{SlotState, render_slot},
        switch::{SwitchConfig, render_switch},
        text::render_text,
        text_input::{TextInputConfig, render_text_input},
    },
    world::World,
};

pub struct UIRender<'a> {
    pub element_id: UIElementId,
    pub global_pos: &'a GlobalPosition,
    pub dim: &'a Dimension,
    pub z_index: u32,
    pub is_hovered: bool,
    pub is_pressed: bool,
    pub is_focused: bool,
    pub is_disabled: bool,
    pub is_on: bool,
    pub visible: bool,
    pub vis: UIVisual<'a>,
}

impl<'a> UIRender<'a> {
    pub fn new(table: &'a UIElementTable, vis: UIVisual<'a>, index: usize) -> UIRender<'a> {
        UIRender {
            element_id: table.id()[index],
            global_pos: &table.global_pos()[index],
            dim: &table.dimension()[index],
            z_index: table.z_index()[index],
            is_hovered: false,
            is_disabled: false,
            is_pressed: false,
            is_focused: false,
            is_on: false,
            visible: table.visible()[index],
            vis,
        }
    }

    pub fn is_on(mut self, value: bool) -> UIRender<'a> {
        self.is_on = value;
        self
    }
}

pub enum UIVisual<'a> {
    UIButton(&'a ButtonConfig),
    UISwitch(&'a SwitchConfig),
    UISlot(&'a SlotState),
    UIRectangle(&'a Style),
    UIText(&'a Style, Option<f32>, &'a Vec<String>),
    UITextInput(&'a TextInputConfig, &'a String),
}

pub fn render(world: &World) {
    let mut render_queue: Vec<UIRender> = Vec::new();
    for table in world.ui_tables.iter() {
        for index in 0..table.id().len() {
            let render = table.render_data(index);
            if let Some(render) = render {
                render_queue.push(render);
            }
        }
    }

    render_queue.sort_unstable_by_key(|item| item.z_index);

    for element in &mut render_queue {
        element.is_hovered = if let Some(entity) = world.hovered_entity {
            entity == element.element_id
        } else {
            false
        };

        element.is_focused = if let Some(entity) = world.focused_entity {
            entity == element.element_id
        } else {
            false
        };

        element.is_pressed = element.is_hovered && is_mouse_button_down(MouseButton::Left);

        match element.vis {
            UIVisual::UIButton(_config) => render_button(&element),
            UIVisual::UISwitch(_config) => render_switch(&element),
            UIVisual::UIRectangle(_config) => render_rectangle(&element),
            UIVisual::UISlot(_config) => render_slot(&element),
            UIVisual::UIText(_, _, _) => render_text(&element, world.font_registry.clone()),
            UIVisual::UITextInput(_, _) => render_text_input(&element, world.font_registry.clone()),
        }
    }

    draw_fps();
    draw_text_ex(&format!("{:?}", world.focused_entity), 10., 40., Default::default());
    draw_text_ex(&format!("{:?}", world.hovered_entity), 10., 50., Default::default());
}
