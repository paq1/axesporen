use rand::Rng;
use crate::core::elements::tilemap::TileMapHudge;
use crate::core::physics::collide_body::CollideBody;
use crate::core::scene::scene_world::enemy::Enemy;
use crate::core::scene::scene_world::player::Player;
use crate::core::sdd::vecteur2d::Vecteur2D;

pub struct SceneWorldData {
    pub is_init: bool,
    pub player: Player,
    pub camera: Vecteur2D<f32>,
    pub tilemap: TileMapHudge, // systeme de map basique (si trop grande joue sur les perfs)
    pub pos_curseur: Vecteur2D<f32>,

    pub vaisseau_a_trouver: CollideBody, // vaisseau a trouver pour changer de monde
    pub compteur_de_monde_genere: u32,

    pub enemies: Vec<Enemy>
}

impl SceneWorldData {
    pub fn new(compteur_de_monde_genere: u32) -> Self {
        let player = Player::new();
        let pos_player = player.pos.clone();

        let enemies = vec![
            Enemy::new(Vecteur2D::new(30.0 * 32.0, 30.0 * 32.0)),
            Enemy::new(Vecteur2D::new(10.0 * 32.0, 30.0 * 32.0)),
            Enemy::new(Vecteur2D::new(20.0 * 32.0, 20.0 * 32.0)),
        ];

        Self {
            is_init: false,
            player,
            camera: pos_player.clone(),
            tilemap: TileMapHudge::new(3, 2, 32, 20, 20),
            pos_curseur: pos_player + Vecteur2D::new(32.0, 0.0),
            vaisseau_a_trouver: CollideBody::basic(
                Self::random_vaisseau(),
                16.0
            ),
            compteur_de_monde_genere,
            enemies
        }
    }

    fn random_vaisseau() -> Vecteur2D<f32> {
        let min = 1;
        let max = 30;

        let mut rng = rand::thread_rng();

        let x = rng.gen_range(min..max);
        let y = rng.gen_range(min..max);

        Vecteur2D::new((x * 32) as f32 , (y * 32) as f32)
    }
}