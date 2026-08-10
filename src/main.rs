use macroquad::prelude::*;
use remyan_client_ecs::page::MainMenu;

#[macroquad::main("RemyanECS")]
async fn main() {
    let mut main_menu = MainMenu::new();
    loop {
        main_menu.update();
        main_menu.draw();
        next_frame().await
    }
}
