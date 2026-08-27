use std::fmt::Debug;

use macroquad::{miniquad::window::screen_size, prelude::*};

use crate::{
    component::{Dimension, Direction, Display, DynDim, DynPos, GlobalPosition, PositionType},
    helper::split_by_width,
    table::UIElementTable,
    ui::UIElement,
    world::World,
};

struct PendingLayoutUpdate {
    new_width: Option<f32>,
    new_height: Option<f32>,
    new_x: Option<f32>,
    new_y: Option<f32>,
}

impl Default for PendingLayoutUpdate {
    fn default() -> Self {
        Self {
            new_width: None,
            new_height: None,
            new_x: None,
            new_y: None,
        }
    }
}

pub fn system_visible<T, U>(world: &mut World<T, U>) {
    for table_idx in 0..world.ui_tables.len() {
        for element_idx in 0..world.ui_tables[table_idx].id().len() {
            let mut is_visible = world.ui_tables[table_idx].visible()[element_idx];

            if let Some(parent) = world.ui_tables[table_idx].parent()[element_idx] {
                let parent_location = &world.ui_locations[parent];
                let parent_visibility = world.ui_tables[parent_location.table.t_index()].visible()
                    [parent_location.index];
                is_visible = parent_visibility;
            }

            let table = &mut world.ui_tables[table_idx];
            table.visible_mut()[element_idx] = is_visible;
        }
    }
}

pub fn system_dialogue_visibility<T, U>(world: &mut World<T, U>) {
    for element_idx in 0..world.ui_tables[UIElement::UIDiv.t_index()].id().len() {
        let table = &mut world.ui_tables[UIElement::UIDiv.t_index()];

        let UIElementTable::UIDivTable(table) = table else {
            continue;
        };

        if !table.dialogue[element_idx] {
            continue;
        }

        if let Some(opened_dialogue) = world.opened_dialogue {
            let is_opened = opened_dialogue == table.ids[element_idx];
            table.visible[element_idx] = is_opened;
        } else {
            table.visible[element_idx] = false;
        }
    }
}

pub fn system_dirty_state<T, U>(world: &mut World<T, U>) {
    let current_screen_size = screen_size();

    if current_screen_size != world.current_screen_size {
        world.is_updated = false;

        for i in &mut world.ui_tables {
            if let Some(dirty) = i.is_dirty_mut() {
                for i in 0..dirty.len() {
                    dirty[i] = true;
                }
            }
        }

        world.current_screen_size = current_screen_size;
    } else {
        if !world.is_updated {
            for i in &mut world.ui_tables {
                if let Some(dirty) = i.is_dirty_mut() {
                    for i in 0..dirty.len() {
                        dirty[i] = true;
                    }
                }
            }

            world.is_updated = true;
        }
    }
}

pub fn system_arrange_text<T, U>(world: &mut World<T, U>) {
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

pub fn system_text_dimension<T, U>(world: &mut World<T, U>) {
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
        // table.is_dirty[index] = false;
    }
}

pub fn system_parent_display<T, U>(world: &mut World<T, U>) {
    for table_idx in 0..world.ui_tables.len() {
        for index in 0..world.ui_tables[table_idx].id().len() {
            let mut pending_updates = PendingLayoutUpdate::default();
            let table = &world.ui_tables[table_idx];

            let entity = table.id()[index];
            if let Some(is_dirty) = table.is_dirty() {
                if !is_dirty[index] {
                    continue;
                }
            }

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
                                if *element_id == entity {
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

                                    pending_updates = PendingLayoutUpdate {
                                        new_width: Some(self_width),
                                        new_height: None,
                                        new_x: Some(new_x),
                                        new_y: None,
                                    }
                                }

                                Direction::Vertical => {
                                    let parent_height =
                                        world.ui_tables[0].dimension()[parent_loc.index].h;
                                    let available_height = parent_height - total_gaps;

                                    let self_height = available_height / child_count as f32;
                                    let new_y = self_index as f32 * (self_height + gap);

                                    pending_updates = PendingLayoutUpdate {
                                        new_width: None,
                                        new_height: Some(self_height),
                                        new_x: None,
                                        new_y: Some(new_y),
                                    }
                                }
                            }
                        }
                    }

                    _ => {}
                }
            }

            let table = &mut world.ui_tables[table_idx];

            if let Some(w) = pending_updates.new_width {
                table.dimension_mut()[index].set_width(w);
            }
            if let Some(h) = pending_updates.new_height {
                table.dimension_mut()[index].set_height(h);
            }
            if let Some(x) = pending_updates.new_x {
                table.position_mut()[index].set_x(x);
            }
            if let Some(y) = pending_updates.new_y {
                table.position_mut()[index].set_y(y);
            }
        }
    }
}

pub fn system_dynamic_transform<T, U>(world: &mut World<T, U>) {
    for table_idx in 0..world.ui_tables.len() {
        for index in 0..world.ui_tables[table_idx].id().len() {
            let table = &world.ui_tables[table_idx];
            let mut pending_updates = PendingLayoutUpdate::default();

            if let Some(is_dirty) = table.is_dirty() {
                if !is_dirty[index] {
                    continue;
                }
            }

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
                        pending_updates = PendingLayoutUpdate {
                            new_width: Some(parent_dim.w),
                            ..pending_updates
                        };
                    }
                    DynDim::Percent(p) => {
                        pending_updates = PendingLayoutUpdate {
                            new_width: Some(parent_dim.w * p),
                            ..pending_updates
                        };
                    }
                    DynDim::Custom(closure) => {
                        pending_updates = PendingLayoutUpdate {
                            new_width: Some(closure(parent_dim.w, parent_dim.h)),
                            ..pending_updates
                        };
                    }
                }
            }

            if let Some(dyn_h) = &dim.dyn_h {
                match dyn_h {
                    DynDim::Full => {
                        pending_updates = PendingLayoutUpdate {
                            new_height: Some(parent_dim.h),
                            ..pending_updates
                        };
                    }
                    DynDim::Percent(p) => {
                        pending_updates = PendingLayoutUpdate {
                            new_height: Some(parent_dim.h * p),
                            ..pending_updates
                        };
                    }
                    DynDim::Custom(closure) => {
                        pending_updates = PendingLayoutUpdate {
                            new_height: Some(closure(parent_dim.w, parent_dim.h)),
                            ..pending_updates
                        };
                    }
                }
            }

            if let Some(dyn_x) = &pos.dyn_x {
                match dyn_x {
                    DynPos::Start => {
                        pending_updates = PendingLayoutUpdate {
                            new_x: Some(0.),
                            ..pending_updates
                        };
                    }
                    DynPos::Center => {
                        pending_updates = PendingLayoutUpdate {
                            new_x: Some(parent_dim.w / 2. - dim.w / 2.),
                            ..pending_updates
                        };
                    }
                    DynPos::End => {
                        pending_updates = PendingLayoutUpdate {
                            new_x: Some(parent_dim.w - dim.w),
                            ..pending_updates
                        };
                    }
                    DynPos::Custom(closure) => {
                        pending_updates = PendingLayoutUpdate {
                            new_x: Some(closure(parent_dim.w, parent_dim.h)),
                            ..pending_updates
                        };
                    }
                }
            }

            if let Some(dyn_y) = &pos.dyn_y {
                match dyn_y {
                    DynPos::Start => {
                        pending_updates = PendingLayoutUpdate {
                            new_y: Some(0.),
                            ..pending_updates
                        };
                    }
                    DynPos::Center => {
                        pending_updates = PendingLayoutUpdate {
                            new_y: Some(parent_dim.h / 2. - dim.h / 2.),
                            ..pending_updates
                        };
                    }
                    DynPos::End => {
                        pending_updates = PendingLayoutUpdate {
                            new_y: Some(parent_dim.h - dim.h),
                            ..pending_updates
                        };
                    }
                    DynPos::Custom(closure) => {
                        pending_updates = PendingLayoutUpdate {
                            new_y: Some(closure(parent_dim.w, parent_dim.h)),
                            ..pending_updates
                        };
                    }
                }
            }

            let table = &mut world.ui_tables[table_idx];

            if let Some(w) = pending_updates.new_width {
                table.dimension_mut()[index].set_width(w);
            }
            if let Some(h) = pending_updates.new_height {
                table.dimension_mut()[index].set_height(h);
            }
            if let Some(x) = pending_updates.new_x {
                table.position_mut()[index].set_x(x);
            }
            if let Some(y) = pending_updates.new_y {
                table.position_mut()[index].set_y(y);
            }
        }
    }
}

pub fn system_transform<T, U>(world: &mut World<T, U>) {
    for table_idx in 0..world.ui_tables.len() {
        for index in 0..world.ui_tables[table_idx].id().len() {
            let pending_update: (f32, f32);
            let table = &world.ui_tables[table_idx];

            if let Some(is_dirty) = table.is_dirty() {
                if !is_dirty[index] {
                    continue;
                }
            }

            let pos = &table.position()[index];

            if let PositionType::Absolute = table.pos_type()[index] {
                pending_update = (pos.x, pos.y);
            } else if let Some(parent) = &table.parent()[index] {
                let parent_loc = &world.ui_locations[*parent];
                let parent_global_pos = &world.ui_tables[0].global_pos()[parent_loc.index];
                pending_update = (parent_global_pos.x + pos.x, parent_global_pos.y + pos.y);
            } else {
                pending_update = (pos.x, pos.y);
            }

            let table = &mut world.ui_tables[table_idx];

            if let Some(is_dirty) = table.is_dirty_mut() {
                is_dirty[index] = false;
            }

            table.global_pos_mut()[index] = GlobalPosition {
                x: pending_update.0,
                y: pending_update.1,
            }
        }
    }
}

pub fn system_hover<T, U>(world: &mut World<T, U>) {
    let (mx, my) = mouse_position();
    let mut highest_z: Option<u32> = None;
    let mut hovered_entity = None;

    for i in &world.hoverable_elements {
        let table = &world.ui_tables[i.t_index()];
        for (entity_idx, entity) in table.id().iter().enumerate() {
            if !table.visible()[entity_idx] {
                continue;
            }

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

pub fn system_on_click<T: Debug + Clone, U>(world: &mut World<T, U>, ui_event: &mut Vec<T>) {
    if is_mouse_button_released(MouseButton::Left) {
        if let Some(entity) = world.hovered_entity {
            let location = &world.ui_locations[entity];
            let table = &mut world.ui_tables[location.table.t_index()];

            match table {
                UIElementTable::UISwitchTable(table) => {
                    table.is_on[location.index] = !table.is_on[location.index];
                    world.focused_entity = None;
                }

                UIElementTable::UITextInputTable(_) => {
                    world.focused_entity = Some(entity);
                }

                _ => {
                    world.focused_entity = None;
                }
            }

            if let Some(events) = table.on_click_event() {
                if let Some(event) = &events[location.index] {
                    ui_event.push(event.0.clone());
                }
            }
        } else {
            world.focused_entity = None;
        }
    }
}

pub fn system_text_input<T, U>(world: &mut World<T, U>) {
    if let Some(entity) = world.focused_entity {
        let location = &world.ui_locations[entity];
        let table = &mut world.ui_tables[location.table.t_index()];

        if let UIElementTable::UITextInputTable(table) = table {
            while let Some(char) = get_char_pressed() {
                if char.is_ascii_graphic() || char == ' ' {
                    if let Some(max_length) = table.max_length[location.index] {
                        if table.value[location.index].len() < max_length {
                            table.value[location.index].push(char);
                        }
                    } else {
                        table.value[location.index].push(char);
                    }
                }
            }

            if is_key_pressed(KeyCode::Backspace) {
                table.value[location.index].pop();
            }
        }
    }
}
