use sdl2::{
    pixels::Color,
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
};

use crate::{event::Event, world::world::World};

pub struct Game<'a> {
    world: World<'a>,
}

impl<'a> Game<'a> {
    pub fn new() -> Self {
        Self {
            world: World::new(),
        }
    }

    pub fn process_event(&mut self, event: Event) {}

    pub fn draw(
        &mut self,
        canvas: &mut Canvas<Window>,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) {
        self.world.draw(canvas, texture_creator);
    }
}
