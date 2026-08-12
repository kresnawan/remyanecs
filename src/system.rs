use macroquad::prelude::*;

use crate::{
    component::{Dimension, Direction, Display, DynDim, DynPos, GlobalPosition, PositionType, UIEvent}, ui::UIElement, world::World
};

struct PendingLayoutUpdate {
    table_idx: usize,
    index: usize,
    new_width: Option<f32>,
    new_height: Option<f32>,
    new_x: Option<f32>,
    new_y: Option<f32>,
}

pub fn system_parent_display(world: &mut World) {
    let mut pending_updates: Vec<PendingLayoutUpdate> = Vec::new();

    for (table_idx, table) in world.ui_tables.iter().enumerate() {
        for (index, entity) in table.id().iter().enumerate() {
            if let Some(parent) = table.parent()[index] {
                let parent_loc = world.ui_locations.get(parent).unwrap();

                let parent_display = if let Some(display) = world.ui_tables[0].display() {
                    &display[parent_loc.index]
                } else {
                    continue;
                };

                match parent_display {
                    Display::Grid { direction, gap } => {
                        let mut child_count = 0;
                        let mut self_index = 0;

                        if let Some(childs) = world.ui_tables[0].childs() {
                            for (index, element_id) in childs[parent_loc.index].iter().enumerate() {
                                if element_id == entity {
                                    self_index = index;
                                }

                                child_count += 1;
                            }
                        }

                        if child_count > 0 {
                            let total_gaps = (child_count - 1) as f32 * gap;
                            match direction {
                                Direction::Horizontal => {
                                    let parent_width =
                                        world.ui_tables[0].dimension()[parent_loc.index].w;
                                    let available_width = parent_width - total_gaps;

                                    let self_width = available_width / child_count as f32;
                                    let new_x = self_index as f32 * (self_width + gap);

                                    pending_updates.push(PendingLayoutUpdate {
                                        table_idx,
                                        index,
                                        new_width: Some(self_width),
                                        new_height: None,
                                        new_x: Some(new_x),
                                        new_y: None,
                                    });
                                }

                                Direction::Vertical => {
                                    let parent_height =
                                        world.ui_tables[0].dimension()[parent_loc.index].h;
                                    let available_height = parent_height - total_gaps;

                                    let self_height = available_height / child_count as f32;
                                    let new_y = self_index as f32 * (self_height + gap);

                                    pending_updates.push(PendingLayoutUpdate {
                                        table_idx,
                                        index,
                                        new_width: None,
                                        new_height: Some(self_height),
                                        new_x: None,
                                        new_y: Some(new_y),
                                    });
                                }
                            }
                        }
                    }

                    _ => {}
                }
            }
        }
    }

    for update in pending_updates {
        let table = &mut world.ui_tables[update.table_idx];

        if let Some(w) = update.new_width {
            table.dimension_mut()[update.index].set_width(w);
        }
        if let Some(h) = update.new_height {
            table.dimension_mut()[update.index].set_height(h);
        }
        if let Some(x) = update.new_x {
            table.position_mut()[update.index].set_x(x);
        }
        if let Some(y) = update.new_y {
            table.position_mut()[update.index].set_y(y);
        }
    }
}

pub fn system_dynamic_transform(world: &mut World) {
    let mut pending_updates: Vec<PendingLayoutUpdate> = Vec::new();

    for (table_idx, table) in world.ui_tables.iter().enumerate() {
        for (index, entity) in table.id().iter().enumerate() {
            let parent_dim = match &table.parent()[index] {
                Some(e) => {
                    &world.ui_tables[UIElement::UIDiv.t_index()].dimension()[*e]
                },
                None => {
                    &Dimension::from(screen_width(), screen_height())
                },
            };

            let (pos, dim) = (&table.position()[index], &table.dimension()[index]);

            if let Some(dyn_w) = &dim.dyn_w {
                match dyn_w {
                    DynDim::Full => {
                        pending_updates.push(PendingLayoutUpdate {
                            table_idx,
                            index,
                            new_width: Some(parent_dim.w),
                            new_height: None,
                            new_x: None,
                            new_y: None,
                        });
                    }
                    DynDim::Percent(p) => {
                        pending_updates.push(PendingLayoutUpdate {
                            table_idx,
                            index,
                            new_width: Some(parent_dim.w * p),
                            new_height: None,
                            new_x: None,
                            new_y: None,
                        });
                    }
                    _ => {}
                }
            }

            if let Some(dyn_h) = &dim.dyn_h {
                match dyn_h {
                    DynDim::Full => {
                        pending_updates.push(PendingLayoutUpdate {
                            table_idx,
                            index,
                            new_width: None,
                            new_height: Some(parent_dim.h),
                            new_x: None,
                            new_y: None,
                        });
                    }
                    DynDim::Percent(p) => {
                        pending_updates.push(PendingLayoutUpdate {
                            table_idx,
                            index,
                            new_width: None,
                            new_height: Some(parent_dim.h * p),
                            new_x: None,
                            new_y: None,
                        });
                    }
                    _ => {}
                }
            }

            if let Some(dyn_x) = &pos.dyn_x {
                match dyn_x {
                    DynPos::Start => {
                        pending_updates.push(PendingLayoutUpdate {
                            table_idx,
                            index,
                            new_width: None,
                            new_height: None,
                            new_x: Some(0.),
                            new_y: None,
                        });
                    }
                    DynPos::Center => {
                        pending_updates.push(PendingLayoutUpdate {
                            table_idx,
                            index,
                            new_width: None,
                            new_height: None,
                            new_x: Some(parent_dim.w / 2. - dim.w / 2.),
                            new_y: None,
                        });
                    }
                    DynPos::End => {
                        pending_updates.push(PendingLayoutUpdate {
                            table_idx,
                            index,
                            new_width: None,
                            new_height: None,
                            new_x: Some(parent_dim.w - dim.w),
                            new_y: None,
                        });
                    }

                    _ => {}
                }
            }

            if let Some(dyn_y) = &pos.dyn_y {
                match dyn_y {
                    DynPos::Start => {
                        pending_updates.push(PendingLayoutUpdate {
                            table_idx,
                            index,
                            new_width: None,
                            new_height: None,
                            new_x: None,
                            new_y: Some(0.),
                        });
                    }
                    DynPos::Center => {
                        pending_updates.push(PendingLayoutUpdate {
                            table_idx,
                            index,
                            new_width: None,
                            new_height: None,
                            new_x: None,
                            new_y: Some(parent_dim.h / 2. - dim.h / 2.),
                        });
                    }
                    DynPos::End => {
                        pending_updates.push(PendingLayoutUpdate {
                            table_idx,
                            index,
                            new_width: None,
                            new_height: None,
                            new_x: None,
                            new_y: Some(parent_dim.h - dim.h),
                        });
                    }

                    _ => {}
                }
            }
        }
    }

    for update in pending_updates {
        let table = &mut world.ui_tables[update.table_idx];

        if let Some(w) = update.new_width {
            table.dimension_mut()[update.index].set_width(w);
        }
        if let Some(h) = update.new_height {
            table.dimension_mut()[update.index].set_height(h);
        }
        if let Some(x) = update.new_x {
            table.position_mut()[update.index].set_x(x);
        }
        if let Some(y) = update.new_y {
            table.position_mut()[update.index].set_y(y);
        }
    }
}

pub fn system_transform(world: &mut World) {
    let mut pending_updates: Vec<(usize, usize, f32, f32)> = Vec::new();
    for (table_idx, table) in world.ui_tables.iter().enumerate() {
        for (index, entity) in table.id().iter().enumerate() {
            let pos = &table.position()[index];

            if let PositionType::Absolute = table.pos_type()[index] {
                pending_updates.push((table_idx, index, pos.x, pos.y));
                continue;
            }

            if let Some(parent) = &table.parent()[index] {
                let parent_loc = &world.ui_locations[*parent];
                let parent_global_pos = &world.ui_tables[0].global_pos()[parent_loc.index];
                pending_updates.push((
                    table_idx,
                    index,
                    parent_global_pos.x + pos.x,
                    parent_global_pos.y + pos.y,
                ));

                continue;
            }

            pending_updates.push((table_idx, *entity, pos.x, pos.y));
        }
    }

    for (table_idx, entity, x, y) in pending_updates {
        let table = &mut world.ui_tables[table_idx];
        table.global_pos_mut()[entity] = GlobalPosition { x, y }
    }
}

pub fn system_hover(world: &mut World) {
    let (mx, my) = mouse_position();
    let mut highest_z: Option<u32> = None;
    let mut hovered_entity = None;

    for i in &world.hoverable_elements {
        let table = &world.ui_tables[i.t_index()];
        for (entity_idx, entity) in table.id().iter().enumerate() {
            let (pos, dim, z) = (
                &table.global_pos()[entity_idx],
                &table.dimension()[entity_idx],
                &table.z_index()[entity_idx],
            );
            let is_inside =
                mx >= pos.x && mx <= (pos.x + dim.w) && my >= pos.y && my <= (pos.y + dim.h);

            let z_index = z;

            if is_inside {
                match highest_z {
                    None => {
                        highest_z = Some(*z_index);
                        hovered_entity = Some(*entity);
                    }
                    Some(max_z) if z_index >= &max_z => {
                        highest_z = Some(*z_index);
                        hovered_entity = Some(*entity);
                    }
                    _ => {}
                }
            }
        }
    }

    world.hovered_entity = hovered_entity;
}

pub fn system_on_click(world: &mut World, ui_events: &mut Vec<UIEvent>) {
    for i in &world.hoverable_elements {
        let table = &world.ui_tables[i.t_index()];
        for entity in table.id() {
            if is_mouse_button_released(MouseButton::Left) {
                if let Some(on_click) = table.on_click_event() {
                    if let Some(event) = &on_click[*entity] {
                        ui_events.push(event.0.clone());
                    }
                }
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
