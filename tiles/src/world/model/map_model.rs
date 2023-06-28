use crate::resource::tileset::TilesetId;
use crate::util::vec2d::Vec2d;

use super::tile_model::{TileIndex, TileModel};

pub struct MapModel {
    pub tileset_id: TilesetId,
    tiles: Vec2d<TileModel>,
}

impl MapModel {
    pub fn test_map() -> Self {
        let mut rows = vec![vec![TileModel::new(TileIndex(0)); 20]; 15];
        rows[1][1] = TileModel::new(TileIndex(58));
        rows[2][2] = TileModel::new(TileIndex(58));

        Self {
            tileset_id: TilesetId("TestTileset".to_owned()),
            tiles: Vec2d::from_rows(rows),
        }
    }

    pub fn rows(&self) -> &Vec<Vec<TileModel>> {
        self.tiles.rows()
    }
}
