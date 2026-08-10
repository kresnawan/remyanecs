use crate::{
    Entity,
    component::{
        Button, ButtonStyle, Dimension, Display, Div, GlobalPosition, OnClickEvent, Parent,
        Position, PositionType, Visible, ZIndex,
    },
};

pub struct World {
    pub next_entity: Entity,
    pub next_z_index: u32,

    pub position: Vec<Option<Position>>,
    pub z_index: Vec<Option<ZIndex>>,
    pub global_pos: Vec<Option<GlobalPosition>>,
    pub position_type: Vec<Option<PositionType>>,

    pub dimension: Vec<Option<Dimension>>,
    pub hovered_entity: Option<Entity>,
    pub parent: Vec<Option<Parent>>,
    pub on_click_event: Vec<Option<OnClickEvent>>,
    pub visible: Vec<Option<Visible>>,

    pub button_style: Vec<Option<ButtonStyle>>,
    pub button: Vec<Option<Button>>,

    pub div: Vec<Option<Div>>,
    pub display: Vec<Option<Display>>,
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
            hovered_entity: None,
            parent: Vec::new(),
            on_click_event: Vec::new(),
            visible: Vec::new(),

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
        visible: Option<Visible>,
        on_click_event: Option<OnClickEvent>,
    ) -> Entity {
        let current_entity = self.next_entity.clone();

        self.global_pos
            .push(Some(GlobalPosition { x: pos.x, y: pos.y }));
        self.position_type.push(Some(pos_type));
        self.z_index.push(Some(ZIndex(self.next_z_index)));
        self.position.push(Some(pos));

        self.dimension.push(Some(dim));
        self.parent.push(parent);
        self.on_click_event.push(on_click_event);
        self.visible.push(visible);

        self.button.push(button);
        self.button_style.push(button_style);

        self.div.push(div);
        self.display.push(display);

        self.next_entity += 1;
        self.next_z_index += 1;

        return current_entity;
    }
}


pub fn spawn_div(
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
        Some(Visible),
        None,
    )
}
