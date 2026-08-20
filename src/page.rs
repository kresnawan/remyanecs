use std::sync::Arc;

use macroquad::{
    color::{BLUE, RED, WHITE},
    window::clear_background,
};

use crate::{
    FontRegistry,
    component::{
        Dimension, Direction, Display, DynDim, DynPos, Position, PositionType, Style, UIColor,
        UIEvent,
    },
    render_q::render,
    system::{
        system_arrange_text, system_dirty_state, system_dynamic_transform, system_handle_ui_events,
        system_hover, system_on_click, system_parent_display, system_text_dimension,
        system_transform,
    },
    table::{button::spawn_std_button, slot::SlotIndex, switch::spawn_std_switch},
    world::World,
};

pub trait Page {
    fn new(font_registry: Arc<FontRegistry>) -> Self;
    fn update(&mut self);
    fn draw(&self);
}

pub struct MainMenu {
    world: World,
    ui_events: Vec<UIEvent>,
}

impl Page for MainMenu {
    fn new(font_registry: Arc<FontRegistry>) -> MainMenu {
        let mut world = World::new(font_registry);

        let container = world.spawn_div(
            (Position::center(), PositionType::Absolute),
            Dimension::new()
                .dyn_w(DynDim::Percent(0.8))
                .dyn_h(DynDim::Percent(0.8)),
            Display::Normal,
            None,
        );

        spawn_std_switch(
            &mut world,
            Position::new().x(100.),
            Dimension::from(100., 50.),
            PositionType::Relative,
            None,
            None,
        );

        world.spawn_slot(
            (Position::new().y(400.).x(25.), PositionType::Relative),
            Dimension::new().w(100.).h(300.),
            SlotIndex::One,
            None,
        );

        let btn_container = world.spawn_div(
            (
                Position::new().dyn_y(DynPos::End).dyn_x(DynPos::Center),
                PositionType::Relative,
            ),
            Dimension::new()
                .dyn_w(DynDim::Full)
                .dyn_h(DynDim::Percent(0.5)),
            Display::Grid {
                direction: Direction::Vertical,
                gap: 20.,
            },
            Some(container),
        );

        spawn_std_button(
            &mut world,
            Position::new(),
            Dimension::new().h(100.).dyn_w(DynDim::Full),
            PositionType::Relative,
            "Create Room",
            Some(btn_container),
            None,
        );

        spawn_std_button(
            &mut world,
            Position::new().y(300.),
            Dimension::new().h(100.).dyn_w(DynDim::Full),
            PositionType::Relative,
            "Create Room",
            Some(btn_container),
            None,
        );

        spawn_std_button(
            &mut world,
            Position::new().y(300.),
            Dimension::new().h(100.).dyn_w(DynDim::Full),
            PositionType::Relative,
            "Settings",
            Some(btn_container),
            None,
        );

        world.spawn_rectangle(
            (Position::new(), PositionType::Relative),
            Dimension::new().w(200.).dyn_h(DynDim::Full),
            Style {
                bg_color: UIColor::Fill(BLUE),
                outline: 0.,
                outline_color: WHITE,
                corner_radius: 5.,
                ..Default::default()
            },
            None,
        );

        world.spawn_text(
            (
                Position::new().dyn_x(DynPos::Center),
                PositionType::Relative,
            ),
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam dictum nunc quis 
            placerat tempus. Orci varius natoque penatibus et magnis dis parturient montes, nascetur 
            ridiculus mus. Fusce dapibus turpis augue, eget porttitor nisl rutrum id. Vivamus sed luctus 
            lectus. Praesent eget ante vel justo dapibus pharetra nec pretium sapien. Fusce vitae euismod 
            sem. Aliquam malesuada nibh erat, vitae laoreet nunc porta id. Praesent ornare, velit ut tempor 
            pretium, velit ex erat. Suspendisse luctus mauris 
            magna. Donec pretium semper pellentesque. In felis tellus, viverra sed velit et, suscipit dictum 
            massa. Phasellus ultricies porta justo non rhoncus. Etiam rutrum nibh vitae accumsan euismod. 
            In id diam congue, malesuada leo sed, mollis ipsum.",
            Style {
                font_size: 20,
                ..Default::default()
            },
            None,
            None,
        );

        MainMenu {
            world,
            ui_events: Vec::new(),
        }
    }

    fn update(&mut self) {
        system_dirty_state(&mut self.world);

        system_arrange_text(&mut self.world);
        system_text_dimension(&mut self.world);

        system_dynamic_transform(&mut self.world);
        system_parent_display(&mut self.world);
        system_transform(&mut self.world);

        system_hover(&mut self.world);
        system_on_click(&mut self.world, &mut self.ui_events);
        system_handle_ui_events(&mut self.world, &mut self.ui_events);
    }
    fn draw(&self) {
        clear_background(RED);
        render(&self.world);
    }
}
