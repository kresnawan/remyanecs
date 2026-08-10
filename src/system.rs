use macroquad::prelude::*;

use crate::{
    component::{
        Dimension, Display, DynDim, DynPos, GlobalPosition, Position, PositionType, UIColor,
        UIEvent,
    },
    world::World,
};

pub fn system_parent_display(world: &mut World) {
    for entity in 0..world.next_entity {
        if let Some(parent) = &world.parent[entity] {
            if let Some(display) = &world.display[parent.0] {
                match display {
                    Display::Grid { direction, gap } => {
                        let mut child_count = 0;
                        let mut self_index = 0;

                        let parent_width = world.dimension[parent.0].as_ref().unwrap().w;

                        for i in 0..world.next_entity {
                            if let Some(found_parent) = &world.parent[i] {
                                if found_parent.0 == parent.0 {
                                    child_count += 1;
                                    if entity == i {
                                        self_index = child_count - 1;
                                    }
                                }
                            }
                        }

                        if child_count > 0 {
                            let total_gaps = (child_count - 1) as f32 * gap;
                            let available_width = parent_width - total_gaps;
                            let self_width = available_width / child_count as f32;

                            world.dimension[entity]
                                .as_mut()
                                .unwrap()
                                .set_width(self_width);

                            let new_x = self_index as f32 * (self_width + gap);
                            world.position[entity].as_mut().unwrap().set_x(new_x);
                        }
                    }

                    _ => {}
                }
            }
        }
    }
}

pub fn system_dynamic_transform(world: &mut World) {
    for entity in 0..world.next_entity {
        let parent_entity = match &world.parent[entity] {
            Some(e) => e,
            None => continue,
        };

        let (parent_w, parent_h) = match &world.dimension[parent_entity.0] {
            Some(dim) => (dim.w, dim.h),
            None => continue,
        };

        if let (Some(pos), Some(dim)) = (&mut world.position[entity], &mut world.dimension[entity])
        {
            if let Some(dyn_w) = &dim.dyn_w {
                match dyn_w {
                    DynDim::Full => {
                        dim.set_width(parent_w);
                    }
                    DynDim::Percent(p) => {
                        dim.set_width(p * parent_w);
                    }
                    _ => {}
                }
            }

            if let Some(dyn_h) = &dim.dyn_h {
                match dyn_h {
                    DynDim::Full => {
                        dim.set_height(parent_h);
                    }
                    DynDim::Percent(p) => {
                        dim.set_height(p * parent_h);
                    }
                    _ => {}
                }
            }

            if let Some(dyn_x) = &pos.dyn_x {
                match dyn_x {
                    DynPos::Start => pos.set_x(0.),
                    DynPos::Center => pos.set_x(parent_w / 2. - dim.w / 2.),
                    DynPos::End => pos.set_x(parent_w - dim.w),

                    _ => {}
                }
            }

            if let Some(dyn_y) = &pos.dyn_y {
                match dyn_y {
                    DynPos::Start => pos.set_y(0.),
                    DynPos::Center => pos.set_y(parent_h / 2. - dim.h / 2.),
                    DynPos::End => pos.set_y(parent_h - dim.h),

                    _ => {}
                }
            }
        }
    }
}

pub fn system_transform(world: &mut World) {
    for entity in 0..world.next_entity {
        let pos = world.position[entity].as_ref().unwrap();

        if let Some(PositionType::Absolute) = &world.position_type[entity] {
            world.global_pos[entity] = Some(GlobalPosition { x: pos.x, y: pos.y });
            continue;
        }

        if let Some(parent) = &world.parent[entity] {
            let updated_pos = world.position[entity].as_ref().unwrap();

            if let Some(parent_global_pos) = &world.global_pos[parent.0] {
                world.global_pos[entity] = Some(GlobalPosition {
                    x: parent_global_pos.x + updated_pos.x,
                    y: parent_global_pos.y + updated_pos.y,
                });
            }

            continue;
        }

        world.global_pos[entity] = Some(GlobalPosition { x: pos.x, y: pos.y })
    }
}

pub fn system_hover(world: &mut World) {
    let (mx, my) = mouse_position();
    let mut highest_z: Option<u32> = None;
    let mut hovered_entity = None;

    for entity in 0..world.next_entity {
        if let (Some(pos), Some(dim), Some(z)) = (
            &world.global_pos[entity],
            &world.dimension[entity],
            &world.z_index[entity],
        ) {
            let is_inside =
                mx >= pos.x && mx <= (pos.x + dim.w) && my >= pos.y && my <= (pos.y + dim.h);

            let z_index = z.0;

            if is_inside {
                match highest_z {
                    None => {
                        highest_z = Some(z_index);
                        hovered_entity = Some(entity);
                    }
                    Some(max_z) if z_index >= max_z => {
                        highest_z = Some(z_index);
                        hovered_entity = Some(entity);
                    }
                    _ => {}
                }
            }
        }
    }

    world.hovered_entity = hovered_entity;
}

pub fn system_on_click(world: &mut World, ui_events: &mut Vec<UIEvent>) {
    if let Some(entity) = world.hovered_entity {
        if is_mouse_button_released(MouseButton::Left) {
            if let Some(on_click) = &world.on_click_event[entity] {
                ui_events.push(on_click.0.clone());
            }
        }
    }
}

pub fn system_handle_ui_events(world: &mut World, ui_events: &mut Vec<UIEvent>) {
    for event in ui_events.drain(..) {
        match event {
            UIEvent::CreateRoom => {
                println!("Room created")
            }

            _ => {}
        }
    }
}
