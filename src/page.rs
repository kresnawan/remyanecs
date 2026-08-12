use macroquad::color::{BLACK, BLUE, GREEN, PURPLE};

use crate::{
    component::{
        ButtonConfig, Dimension, Display, DynDim, Position, PositionType, Style, UIColor, UIEvent,
    }, render_q::render, system::{
        system_dynamic_transform, system_handle_ui_events, system_hover, system_on_click,
        system_parent_display, system_transform,
    }, world::World
};

pub struct MainMenu {
    world: World,
    ui_events: Vec<UIEvent>,
}

impl MainMenu {
    pub fn new() -> MainMenu {
        let mut world = World::new();

        let container = world.spawn_div(
            (Position::from(100., 100.), PositionType::Absolute),
            Dimension::new(900., 400.),
            // Display::Grid {
            //     direction: Direction::Horizontal,
            //     gap: 10.0,
            // },
            Display::Normal,
            None,
        );

        world.spawn_button(
            (Position::from(0., 0.), PositionType::Relative),
            Dimension {
                w: 0.,
                h: 100.,
                dyn_w: Some(DynDim::Percent(0.5)),
                dyn_h: None,
            },
            ButtonConfig {
                text: "Pencet".to_owned(),
                style: Style {
                    bg_color: UIColor::Fill(GREEN),
                    color: UIColor::Fill(BLACK),
                    font: 1,
                    font_size: 48,
                    outline: None,
                    outline_color: None,
                },
                hover_style: Some(Style {
                    bg_color: UIColor::Fill(BLUE),
                    color: UIColor::Fill(BLACK),
                    font: 1,
                    font_size: 48,
                    outline: None,
                    outline_color: None,
                }),
            },
            Some(container),
            None,
        );

        world.spawn_button(
            (Position::from(100., 50.), PositionType::Relative),
            Dimension {
                w: 0.,
                h: 100.,
                dyn_w: Some(DynDim::Percent(0.5)),
                dyn_h: None,
            },
            ButtonConfig {
                text: "Pencet".to_owned(),
                style: Style {
                    bg_color: UIColor::Fill(PURPLE),
                    color: UIColor::Fill(BLACK),
                    font: 1,
                    font_size: 48,
                    outline: None,
                    outline_color: None,
                },
                hover_style: Some(Style {
                    bg_color: UIColor::Fill(BLUE),
                    color: UIColor::Fill(BLACK),
                    font: 1,
                    font_size: 48,
                    outline: None,
                    outline_color: None,
                }),
            },
            Some(container),
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

        println!("{:#?}", self.world.hovered_entity)
    }
    pub fn draw(&self) {
        render(&self.world);
    }
}
