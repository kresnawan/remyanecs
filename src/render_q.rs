use macroquad::{
    input::{MouseButton, is_mouse_button_down},
    time::draw_fps,
};

use crate::{
    UIElementId,
    component::{ButtonConfig, Dimension, GlobalPosition},
    table::button::render_button,
    world::World,
};

pub struct UIRender<'a> {
    pub element_id: UIElementId,
    pub global_pos: &'a GlobalPosition,
    pub dim: &'a Dimension,
    pub z_index: u32,
    pub is_hovered: bool,
    pub is_pressed: bool,
    pub visible: bool,
    pub vis: UIVisual<'a>,
}

pub enum UIVisual<'a> {
    UIButton(&'a ButtonConfig),
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

        element.is_pressed = element.is_hovered && is_mouse_button_down(MouseButton::Left);

        match element.vis {
            UIVisual::UIButton(_config) => render_button(&element),
        }
    }

    draw_fps();
}
