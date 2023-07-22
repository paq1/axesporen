use crate::core::elements::tilemap::TileMap;
use crate::core::scene::scene_world::player::Player;
use crate::core::sdd::vecteur2d::Vecteur2D;

pub struct SceneExempleData {
    pub is_init: bool,
    pub player: Player,
    pub camera: Vecteur2D<f32>,
    pub tilemap: TileMap, // systeme de map basique (si trop grande joue sur les perfs)
    pub pos_curseur: Vecteur2D<f32>
}

impl SceneExempleData {
    pub fn new() -> Self {
        let player = Player::new();
        let pos_player = player.pos.clone();

        Self {
            is_init: false,
            player,
            camera: pos_player.clone(),
            tilemap: TileMap::new(30, 30, 32),
            pos_curseur: pos_player + Vecteur2D::new(32.0, 0.0),
        }
    }
}