use macroquad::{color::RED, window::clear_background};

use crate::{
    component::{Dimension, Direction, Display, DynDim, DynPos, Position, PositionType, UIEvent},
    render_q::render,
    system::{
        system_dynamic_transform, system_handle_ui_events, system_hover, system_on_click,
        system_parent_display, system_transform,
    },
    table::button::spawn_std_button,
    world::World,
};

pub struct MainMenu {
    world: World,
    ui_events: Vec<UIEvent>,
}

impl MainMenu {
    pub fn new() -> MainMenu {
        let mut world = World::new();

        let container = world.spawn_div(
            (Position::center(), PositionType::Absolute),
            Dimension::new()
                .dyn_w(DynDim::Percent(0.8))
                .dyn_h(DynDim::Percent(0.8)),
            Display::Normal,
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
            // Display::Grid {
            //     direction: Direction::Horizontal,
            //     gap: 10.0,
            // },
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

        MainMenu {
            world,
            ui_events: Vec::new(),
        }
    }
}

impl MainMenu {
    pub fn update(&mut self) {
        system_parent_display(&mut self.world);
        system_dynamic_transform(&mut self.world);
        system_transform(&mut self.world);
        system_hover(&mut self.world);
        system_on_click(&mut self.world, &mut self.ui_events);
        system_handle_ui_events(&mut self.world, &mut self.ui_events);
    }
    pub fn draw(&self) {
        clear_background(RED);
        render(&self.world);
    }
}
