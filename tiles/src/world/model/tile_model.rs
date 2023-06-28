#[derive(Clone, Copy)]
pub struct TileIndex(pub u32);

#[derive(Clone)]
pub struct TileModel {
    pub tile_index: TileIndex,
}

impl TileModel {
    pub fn new(tile_index: TileIndex) -> Self {
        Self { tile_index }
    }
}
