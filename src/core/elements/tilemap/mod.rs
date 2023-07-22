use crate::core::elements::tilemap::tile::{Tile, TileType};
use crate::core::sdd::vecteur2d::Vecteur2D;

pub mod tile;

pub struct TileMap {
    pub tiles: Vec<Vec<Tile>>,
    pub tile_size: u32
}

impl TileMap {
    pub fn new(w: u32, h: u32, tile_size: u32) -> Self {
        let tiles = (0u32 .. h)
            .into_iter()
            .map(|current_line| {
                (0u32 .. w)
                    .into_iter()
                    .map(|current_column| {
                        Tile {
                            pos: Vecteur2D::new(
                                current_column as f32,
                                current_line as f32
                            ),
                            r#type: if current_column == 0 || current_column == w - 1 || current_line == 0 || current_line == h - 1 {
                                TileType::Mur
                            } else {
                                TileType::Herbe
                            }
                        }
                    })
                    .collect::<Vec<Tile>>()
            })
            .collect::<Vec<Vec<Tile>>>();

        Self { tiles, tile_size }
    }

    pub fn get_tile_from_position(&self, position: &Vecteur2D<f32>) -> Option<Tile> {
        let index_x = (position.x / self.tile_size as f32).floor() as i32;
        let index_y = (position.y / self.tile_size as f32).floor() as i32;

        match (index_x, index_y) {
            (x, y) if self.indexes_valid(x, y) => {
                // fixme clean le code
                let line = self.tiles
                    .get(y as usize)
                    .unwrap();

                let tile = line.get(x as usize).unwrap().clone();

                Some(tile)
            },
            _ => None
        }
    }

    pub fn indexes_valid(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.tiles.get(0).unwrap().len() as i32 && y < self.tiles.len() as i32
    }
}