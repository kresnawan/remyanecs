use macroquad::prelude::*;

use crate::{
    UIElementId,
    component::{
        Dimension, Direction, Display, DynDim, DynPos, GlobalPosition, PositionType, UIEvent,
    },
    helper::split_by_width,
    table::UIElementTable,
    ui::UIElement,
    world::World,
};

struct PendingLayoutUpdate {
    table_idx: usize,
    index: usize,
    new_width: Option<f32>,
    new_height: Option<f32>,
    new_x: Option<f32>,
    new_y: Option<f32>,
}

pub fn system_arrange_text(world: &mut World) {
    let mut update_queue: Vec<(usize, Vec<String>)> = Vec::new();

    let UIElementTable::UITextTable(table) = &world.ui_tables[UIElement::UIText.t_index()] else {
        return;
    };

    for (index, _entity) in table.ids.iter().enumerate() {
        if !table.is_dirty[index] {
            continue;
        }

        let value = &table.value[index];
        let max_width = {
            if let Some(mw) = table.max_width[index] {
                mw
            } else {
                if let Some(parent_id) = table.parent[index] {
                    let parent_loc = &world.ui_locations[parent_id];

                    let div_table = &world.ui_tables[UIElement::UIDiv.t_index()].dimension()
                        [parent_loc.index]
                        .w;
                    *div_table
                } else {
                    screen_width()
                }
            }
        };
        let font_size = table.style[index].font_size;
        let res = split_by_width(value, max_width.clone(), None, font_size);

        update_queue.push((index, res));
    }

    let UIElementTable::UITextTable(table) = &mut world.ui_tables[UIElement::UIText.t_index()]
    else {
        return;
    };

    for (index, lines) in update_queue {
        table.lines[index] = lines;
    }
}

pub fn system_text_dimension(world: &mut World) {
    let UIElementTable::UITextTable(table) = &mut world.ui_tables[UIElement::UIText.t_index()]
    else {
        return;
    };
    for (index, _entity) in table.ids.iter().enumerate() {
        if !table.is_dirty[index] {
            continue;
        }

        let mut width: f32 = 0.;
        let mut line_height: f32 = 0.;

        let font = None;
        let font_size = table.style[index].font_size;
        let line_spacing = table.style[index].line_spacing;
        let font_scale = 1.;
        let line_number = table.lines[index].len();

        for line in &table.lines[index] {
            let dim = measure_text(line, font, font_size, font_scale);
            width = width.max(dim.width);
            line_height = dim.height;
        }

        let height = line_height * line_number as f32 + line_spacing * (line_number - 1) as f32;

        table.dimension[index].w = width;
        table.dimension[index].h = height;

        table.is_dirty[index] = false;
    }
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
        for (index, _entity) in table.id().iter().enumerate() {
            let parent_dim = match &table.parent()[index] {
                Some(entity) => {
                    let location = &world.ui_locations[*entity];
                    &world.ui_tables[UIElement::UIDiv.t_index()].dimension()[location.index]
                }
                None => &Dimension::from(screen_width(), screen_height()),
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
        for (index, _entity) in table.id().iter().enumerate() {
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

            pending_updates.push((table_idx, index, pos.x, pos.y));
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
    let mut update_queue: Vec<(usize, UIElementId, bool)> = Vec::new();

    for i in &world.hoverable_elements {
        let table = &world.ui_tables[i.t_index()];
        for (index, entity) in table.id().iter().enumerate() {
            let is_hovered = if let Some(hovered_entity) = world.hovered_entity {
                hovered_entity == *entity
            } else {
                false
            };

            if is_mouse_button_released(MouseButton::Left) && is_hovered {
                match table {
                    UIElementTable::UISwitchTable(table) => {
                        update_queue.push((i.t_index(), index, !table.is_on[index]));
                    }

                    _ => {}
                }

                if let Some(on_click) = table.on_click_event() {
                    if let Some(event) = &on_click[index] {
                        ui_events.push(event.0.clone());
                    }
                }
            }
        }
    }

    for update in update_queue {
        let table = &mut world.ui_tables[update.0];
        match table {
            UIElementTable::UISwitchTable(table) => table.is_on[update.1] = update.2,

            _ => {}
        }
    }
}

pub fn system_handle_ui_events(_world: &mut World, ui_events: &mut Vec<UIEvent>) {
    for event in ui_events.drain(..) {
        match event {
            UIEvent::CreateRoom => {
                println!("Room created")
            }

            _ => {}
        }
    }
}
