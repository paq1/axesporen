use crate::core::sdd::vecteur2d::Vecteur2D;

#[derive(PartialEq, Clone)]
pub enum TileType {
    Herbe,
    Mur,
    Sand,
    Snow,
    Goo,
    Wood
}

#[derive(Clone)]
pub struct Tile {
    pub pos: Vecteur2D<f32>,
    pub r#type: TileType
}
