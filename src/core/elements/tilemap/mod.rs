use crate::core::elements::tilemap::tile::{Tile, TileType};
use crate::core::sdd::vecteur2d::Vecteur2D;

pub mod tile;

pub struct TileMapHudge {
    pub tilemaps: Vec<Vec<TileMap>>,
    pub tile_size: u32,
    pub nb_tilemap_column: u32,
    pub nb_tilemap_line: u32,
    pub width_one_tilemap: u32,
    pub height_one_tilemap: u32,
}

impl TileMapHudge {
    pub fn new(w: u32, h: u32, tile_size: u32) -> Self {

        let width_tilemap = 30;
        let height_tilemap = 30;

        let tilemaps = (0u32 .. h)
            .into_iter()
            .map(|current_line| {
                (0u32 .. w)
                    .into_iter()
                    .map(|current_column| {

                        TileMap::new(
                            width_tilemap,
                            height_tilemap,
                            tile_size,
                            Some(
                                Vecteur2D::new(
                                    current_column * width_tilemap,
                                    current_line * height_tilemap
                                )
                            )
                        )

                    })
                    .collect::<Vec<TileMap>>()
            })
            .collect::<Vec<Vec<TileMap>>>();

        Self {
            tilemaps,
            tile_size,
            nb_tilemap_column: w,
            nb_tilemap_line: h,
            width_one_tilemap: width_tilemap,
            height_one_tilemap: height_tilemap
        }
    }

    pub fn get_tilemap_index_from_position(&self, position: &Vecteur2D<f32>) -> Vecteur2D<u32> {
        let index_tile_x = (position.x / self.tile_size as f32).floor() as u32;
        let index_tile_y = (position.y / self.tile_size as f32).floor() as u32;

        let index_tilemap_x = index_tile_x / self.width_one_tilemap;
        let index_tilemap_y = index_tile_y / self.width_one_tilemap;

        Vecteur2D::new(index_tilemap_x, index_tilemap_y)
    }

    pub fn get_tilemap_from_index(&self, index: &Vecteur2D<u32>) -> &TileMap {
       self.tilemaps.get(index.y as usize).unwrap().get(index.x as usize).unwrap()
    }

    pub fn get_tilemap_from_position(&self, position: &Vecteur2D<f32>) -> Option<&TileMap> {
        let index_curent_tilemap = self.get_tilemap_index_from_position(position);

        match (index_curent_tilemap.x as i32, index_curent_tilemap.y as i32) {
            (x, y) if self.indexes_tilemap_valid(x, y) => {

                let line = self.tilemaps.get(y as usize).unwrap();

                let tilemap = line.get(x as usize).unwrap();

                Some(tilemap)
            },
            _ => None
        }
    }

    pub fn get_tilemap_from_position_mut(&mut self, position: &Vecteur2D<f32>) -> Option<&mut TileMap> {
        let index_curent_tilemap = self.get_tilemap_index_from_position(position);

        match (index_curent_tilemap.x as i32, index_curent_tilemap.y as i32) {
            (x, y) if self.indexes_tilemap_valid(x, y) => {

                let line = self.tilemaps.get_mut(y as usize).unwrap();

                let tilemap = line.get_mut(x as usize).unwrap();

                Some(tilemap)
            },
            _ => None
        }
    }

    pub fn indexes_tilemap_valid(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.nb_tilemap_column as i32 && y < self.nb_tilemap_line as i32
    }
}


pub struct TileMap {
    pub tiles: Vec<Vec<Tile>>,
    pub tile_size: u32
}

impl TileMap {
    pub fn new(w: u32, h: u32, tile_size: u32, from_coord: Option<Vecteur2D<u32>>) -> Self {

        let cood_base = from_coord.unwrap_or(Vecteur2D::new(0, 0));

        let tiles = (0u32 .. h)
            .into_iter()
            .map(|current_line| {
                (0u32 .. w)
                    .into_iter()
                    .map(|current_column| {
                        Tile {
                            pos: Vecteur2D::new(
                                (current_column + cood_base.x) as f32,
                                (current_line + cood_base.y) as f32
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