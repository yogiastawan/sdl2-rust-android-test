use std::cell::{RefCell, RefMut};

use bevy_ecs::{
    bundle,
    component::{self, Component},
    world::World,
};
use sdl2::event::Event;

pub trait Scene {
    fn get_world(&self) -> &RefCell<World>;

    fn update(&self, event: &Event);

    fn draw(&self);
}

pub struct MyScene {
    world: RefCell<World>,
}

impl MyScene {
    pub fn new() -> Self {
        MyScene {
            world: RefCell::new(World::new()),
        }
    }
}

impl Scene for MyScene {
    fn update(&self, event: &Event) {
        // todo!()
    }

    fn draw(&self) {
        let bundle = Ab {};
        self.get_world().borrow_mut().spawn(bundle);
    }

    fn get_world(&self) -> &RefCell<World> {
        &self.world
    }
}

#[derive(Component)]
struct Ab {}
