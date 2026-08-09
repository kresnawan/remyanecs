use std::collections::HashMap;

use macroquad::{input::KeyCode::P, prelude::*};

struct Page {
    world: World,
}

#[derive(Clone)]
struct Gradient {
    colors: Vec<Color>,
    angle: f32,
}

impl Page {
    pub fn update(&mut self) {
        system_transform(&mut self.world);
        system_hover(&mut self.world);
    }
    pub fn draw(&self) {
        draw_world(&self.world);
    }
}

struct Button;
struct Rectangle;

#[derive(Clone)]
struct Style {
    bg_color: UIColor,
    color: UIColor,
    font: u32,
    font_size: u16,
    outline: Option<f32>,
    outline_color: Option<Color>,
}

struct ButtonStyle {
    text: String,
    style: Style,
    hover_style: Option<Style>,
}

type Entity = usize;

struct ZIndex(u32);

struct GlobalPosition {
    x: f32,
    y: f32,
}

struct Position {
    x: f32,
    y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Position {
        Position { x, y }
    }
}

struct Dimension {
    w: f32,
    h: f32,
}

enum Display {
    Normal,
    Grid,
    Flex,
}

enum PositionType {
    Absolute,
    Relative,
}

struct Div;

#[derive(Clone)]
enum UIColor {
    Fill(Color),
    Gradient(Gradient),
}

struct Hovered;

struct Parent(pub Entity);

struct World {
    next_entity: Entity,
    next_z_index: u32,

    position: Vec<Option<Position>>,
    z_index: Vec<Option<ZIndex>>,
    global_pos: Vec<Option<GlobalPosition>>,
    position_type: Vec<Option<PositionType>>,

    dimension: Vec<Option<Dimension>>,
    hovered: Vec<Option<Hovered>>,
    parent: Vec<Option<Parent>>,

    button_style: Vec<Option<ButtonStyle>>,
    button: Vec<Option<Button>>,

    div: Vec<Option<Div>>,
    display: Vec<Option<Display>>,
}

impl World {
    pub fn new() -> World {
        World {
            next_entity: 0,
            next_z_index: 0,

            position: Vec::new(),
            z_index: Vec::new(),
            global_pos: Vec::new(),
            position_type: Vec::new(),

            dimension: Vec::new(),
            hovered: Vec::new(),
            parent: Vec::new(),

            button_style: Vec::new(),
            button: Vec::new(),

            div: Vec::new(),
            display: Vec::new(),
        }
    }
    pub fn spawn(
        &mut self,
        pos: Position,
        dim: Dimension,
        pos_type: PositionType,
        button_style: Option<ButtonStyle>,
        button: Option<Button>,
        parent: Option<Parent>,
        div: Option<Div>,
        display: Option<Display>,
    ) -> Entity {
        let current_entity = self.next_entity.clone();
        self.global_pos
            .push(Some(GlobalPosition { x: pos.x, y: pos.y }));
        self.position_type.push(Some(pos_type));
        self.z_index.push(Some(ZIndex(self.next_z_index)));
        self.position.push(Some(pos));
        self.dimension.push(Some(dim));
        self.button_style.push(button_style);
        self.hovered.push(None);
        self.button.push(button);
        self.parent.push(parent);
        self.div.push(div);
        self.display.push(display);

        self.next_entity += 1;
        self.next_z_index += 1;

        return current_entity;
    }
}

fn system_transform(world: &mut World) {
    for i in 0..world.next_entity {
        if let Some(pos) = &world.position[i] {
            if let Some(parent) = &world.parent[i] {
                if let Some(parent_global_pos) = &world.global_pos[parent.0] {
                    world.global_pos[i] = Some(GlobalPosition {
                        x: parent_global_pos.x + pos.x,
                        y: parent_global_pos.y + pos.y,
                    });

                    continue;
                }
            }

            world.global_pos[i] = Some(GlobalPosition { x: pos.x, y: pos.y })
        }
    }
}

fn system_hover(world: &mut World) {
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

    for entity in 0..world.next_entity {
        world.hovered[entity] = None;
    }

    if let Some(entity) = hovered_entity {
        world.hovered[entity] = Some(Hovered);
    }
}

fn draw_world(world: &World) {
    for entity in 0..world.next_entity {
        if let (Some(dim), Some(button_style), Some(pos)) = (
            &world.dimension[entity],
            &world.button_style[entity],
            &world.global_pos[entity],
        ) {
            if world.button[entity].is_some() {
                let is_hovered = world.hovered[entity].is_some();
                let active_style = if is_hovered {
                    button_style
                        .hover_style
                        .as_ref()
                        .unwrap_or(&button_style.style)
                } else {
                    &button_style.style
                };

                let text_dimension =
                    measure_text(&button_style.text, None, active_style.font_size, 1.0);
                let text_pos = (
                    pos.x + dim.w / 2. - text_dimension.width / 2.,
                    pos.y + text_dimension.height + dim.h / 2. - text_dimension.height / 2.,
                );

                if let UIColor::Fill(color) = active_style.bg_color {
                    draw_rectangle(pos.x, pos.y, dim.w, dim.h, color);
                }

                draw_text(
                    &button_style.text,
                    text_pos.0,
                    text_pos.1,
                    active_style.font_size as f32,
                    WHITE,
                );
            }
        }
    }
}

fn spawn_button(
    world: &mut World,
    pos: (Position, PositionType),
    dim: Dimension,
    style: ButtonStyle,
    parent: Option<Parent>,
) {
    world.spawn(
        pos.0,
        dim,
        pos.1,
        Some(style),
        Some(Button),
        parent,
        None,
        None,
    );
}

fn spawn_div(
    world: &mut World,
    pos: (Position, PositionType),
    dim: Dimension,
    display: Display,
) -> Entity {
    world.spawn(
        pos.0,
        dim,
        pos.1,
        None,
        None,
        None,
        Some(Div),
        Some(display),
    )
}

#[macroquad::main("MyGame")]
async fn main() {
    let mut world = World::new();

    let container = spawn_div(
        &mut world,
        (Position { x: 100., y: 100. }, PositionType::Absolute),
        Dimension { w: 500., h: 400. },
        Display::Normal,
    );

    spawn_button(
        &mut world,
        (Position::new(0., 0.), PositionType::Relative),
        Dimension { w: 200., h: 100. },
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
    );

    spawn_button(
        &mut world,
        (Position::new(100., 50.), PositionType::Relative),
        Dimension { w: 200., h: 100. },
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
    );

    loop {
        system_transform(&mut world);
        system_hover(&mut world);
        clear_background(RED);
        draw_world(&world);
        next_frame().await
    }
}
