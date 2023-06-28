use sdl2::{
    rect::Rect,
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
};

use crate::{resource::resource_bank::ResourceBank, world::model::map_model::MapModel};

pub struct MapView<'a> {
    resource_bank: ResourceBank<'a>,
}

impl<'a> MapView<'a> {
    pub fn new() -> Self {
        Self {
            resource_bank: ResourceBank::new(),
        }
    }

    pub fn draw(
        &mut self,
        canvas: &mut Canvas<Window>,
        map_model: &MapModel,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) {
        let tileset = self
            .resource_bank
            .get_tileset(map_model.tileset_id.clone(), texture_creator);

        for (row_index, row) in map_model.rows().iter().enumerate() {
            for (column_index, tile_model) in row.iter().enumerate() {
                let tile = tileset.get_tile(tile_model.tile_index);
                let x = column_index * tileset.tile_width as usize;
                let y = row_index * tileset.tile_height as usize;
                let destination =
                    Rect::new(x as i32, y as i32, tileset.tile_width, tileset.tile_height);
                tile.draw(canvas, destination);
            }
        }
    }
}
