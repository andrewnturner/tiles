use sdl2::{
    image::{LoadSurface, LoadTexture},
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    surface::Surface,
    video::{Window, WindowContext},
};

use crate::world::model::tile_model::TileIndex;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct TilesetId(pub String);

pub struct Tileset<'a> {
    pub tile_width: u32,
    pub tile_height: u32,
    texture: Texture<'a>,
    tiles_per_row: u32,
}

impl<'a> Tileset<'a> {
    pub fn test_tileset(texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        let texture = texture_creator
            .load_texture("../data/tileset/test_tileset.png")
            .unwrap();

        let texture_query = texture.query();
        let tiles_per_row = texture_query.width / 16;

        Self {
            tile_width: 16,
            tile_height: 16,
            texture,
            tiles_per_row,
        }
    }

    pub fn get_tile(&self, tile_index: TileIndex) -> Tile {
        let tile_location_x = tile_index.0 % self.tiles_per_row;
        let tile_location_y = tile_index.0 / self.tiles_per_row;

        let tile_position_x = tile_location_x * self.tile_width;
        let tile_position_y = tile_location_y * self.tile_height;

        let tile = Tile {
            texture: &self.texture,
            source: Rect::new(
                tile_position_x as i32,
                tile_position_y as i32,
                self.tile_width,
                self.tile_height,
            ),
        };

        tile
    }
}

pub struct Tile<'a> {
    texture: &'a Texture<'a>,
    source: Rect,
}

impl<'a> Tile<'a> {
    pub fn draw(&self, canvas: &mut Canvas<Window>, destination: Rect) {
        canvas.copy(self.texture, self.source, destination).unwrap();
    }
}
