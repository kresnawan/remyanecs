use macroquad::color::{BLACK, GREEN, PURPLE};

use crate::{
    component::{
        ButtonStyle, Dimension, Direction, Display, DynDim, OnClickEvent, Parent, Position,
        PositionType, Style, UIColor, UIEvent,
    },
    system::{
        system_dynamic_transform, system_handle_ui_events, system_hover, system_on_click,
        system_parent_display, system_transform,
    },
    ui::widgets::button::{spawn_button, system_draw_button},
    world::{World, spawn_div},
};

pub struct MainMenu {
    world: World,
    ui_events: Vec<UIEvent>,
}

impl MainMenu {
    pub fn new() -> MainMenu {
        let mut world = World::new();

        let container = spawn_div(
            &mut world,
            (Position::new(100., 100.), PositionType::Absolute),
            Dimension::new(900., 400.),
            // Display::Grid {
            //     direction: Direction::Horizontal,
            //     gap: 10.0,
            // },
            Display::Normal,
        );

        spawn_button(
            &mut world,
            (Position::new(0., 0.), PositionType::Relative),
            Dimension {
                w: 0.,
                h: 100.,
                dyn_w: Some(DynDim::Percent(0.5)),
                dyn_h: None,
            },
            ButtonStyle {
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
                    bg_color: UIColor::Fill(PURPLE),
                    color: UIColor::Fill(BLACK),
                    font: 1,
                    font_size: 48,
                    outline: None,
                    outline_color: None,
                }),
            },
            Some(Parent(container)),
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
        system_draw_button(&self.world);
    }
}
