use crate::core::elements::tilemap::tile::{Tile, TileType};
use crate::core::elements::tilemap::TileMap;
use crate::core::sdd::vecteur2d::Vecteur2D;

#[derive(Clone)]
pub struct CollideBody {
    pub position: Vecteur2D<f32>,
    pub radars: Vec<Vecteur2D<f32>> // ce des position a ajouter a la position initial
}


pub trait CanCollideWithTileMap {
    fn collide_with(&self, tilemap: &TileMap) -> Vec<Option<Tile>>;
    fn is_collide(&self, tilemap: &TileMap, discriminants: Vec<TileType>) -> bool;
}

impl CollideBody {
    pub fn new(position: Vecteur2D<f32>, radars: Vec<Vecteur2D<f32>>) -> Self {
        Self {
            position, radars
        }
    }

    pub fn basic(position: Vecteur2D<f32>, taille: f32) -> Self {
        Self {
            position,
            radars: vec![
                Vecteur2D::new(0f32, -taille),
                Vecteur2D::new(taille, 0f32),
                Vecteur2D::new(0f32, taille),
                Vecteur2D::new(-taille, 0f32),
            ]
        }
    }

    pub fn is_collide_with_object(&self, position: &Vecteur2D<f32>, size: f32) -> bool {
        self.radars.iter()
            .map(|radar| radar.clone() + self.position.clone())
            .find(|radar| {
                Vecteur2D::<f32>::from_points(radar, position)
                    .norme() < size
            })
            .is_some()
    }
}

impl CanCollideWithTileMap for CollideBody {
    fn collide_with(&self, tilemap: &TileMap) -> Vec<Option<Tile>> {
        self.radars.iter()
            .map(|radar| self.position.clone() + radar.clone())
            .map(|pos_radar| {
                tilemap.get_tile_from_position(&pos_radar)
            })
            .collect::<Vec<_>>()
    }

    fn is_collide(&self, tilemap: &TileMap, discriminants: Vec<TileType>) -> bool {
        let collide_number = self.collide_with(tilemap)
            .into_iter()
            .filter(|t| {
                t.is_none() || t.clone().map(|tile| discriminants.contains(&tile.r#type)).unwrap_or(false)
            })
            .count();

        collide_number > 0
    }
}