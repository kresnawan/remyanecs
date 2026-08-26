use std::{collections::HashMap, sync::Arc};

use macroquad::prelude::*;
use remyanecs::{
    FontRegistry,
    page::{MainMenu, Page},
};

#[macroquad::main("RemyanECS")]
async fn main() {
    let font_registry = FontRegistry {
        fonts: HashMap::new(),
    };

    let mut main_menu = MainMenu::new(Arc::new(font_registry));
    loop {
        main_menu.update();
        main_menu.draw();
        next_frame().await
    }
}
