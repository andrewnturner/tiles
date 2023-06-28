use std::collections::HashMap;

use sdl2::{
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
};

use super::tileset::{Tileset, TilesetId};

pub struct ResourceBank<'a> {
    tilesets: HashMap<TilesetId, Tileset<'a>>,
}

impl<'a> ResourceBank<'a> {
    pub fn new() -> Self {
        Self {
            tilesets: HashMap::new(),
        }
    }

    pub fn get_tileset(
        &mut self,
        tileset_id: TilesetId,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> &Tileset {
        self.tilesets
            .entry(tileset_id)
            .or_insert_with(|| Tileset::test_tileset(texture_creator))
    }
}
