use rand::Rng;

use crate::core::elements::tilemap::TileMap;
use crate::core::physics::collide_body::CollideBody;
use crate::core::scene::scene_world::enemy::Enemy;
use crate::core::scene::scene_world::player::Player;
use crate::core::sdd::vecteur2d::Vecteur2D;

pub struct SceneWorldData {
    pub is_init: bool,
    pub player: Player,
    pub camera: Vecteur2D<f32>,
    pub tilemap: TileMap,// TileMapHudge, // systeme de map basique (si trop grande joue sur les perfs)
    pub pos_curseur: Vecteur2D<f32>,

    pub vaisseau_a_trouver: CollideBody, // vaisseau a trouver pour changer de monde
    pub compteur_de_monde_genere: u32,

    pub enemies: Vec<Enemy>
}

impl SceneWorldData {
    pub fn new(compteur_de_monde_genere: u32) -> Self {
        let player = Player::new();
        let pos_player = player.pos.clone();

        let nb_biome_w: u32 = 10;
        let nb_biome_h: u32 = 10;
        let tile_size: u32 = 32;

        let width_per_biome: u32 = 20;
        let height_per_biome: u32 = 20;

        let enemies = Self::generate_random_enemies(
            Vecteur2D::<i32>::new(10, 10),
            Vecteur2D::<i32>::new((nb_biome_w * width_per_biome - 1u32) as i32 , (nb_biome_h * height_per_biome - 1u32) as i32),
            tile_size,
            10 * compteur_de_monde_genere
        );

        Self {
            is_init: false,
            player,
            camera: pos_player.clone(),
            tilemap: TileMap::new(
                nb_biome_w * width_per_biome,
                nb_biome_h * height_per_biome,
                32u32,
                None
            ),// TileMapHudge::new(nb_biome_w, nb_biome_h, tile_size, width_per_biome, height_per_biome),
            pos_curseur: pos_player + Vecteur2D::new(32.0, 0.0),
            vaisseau_a_trouver: CollideBody::basic(
                Self::random_vaisseau(
                    &Vecteur2D::<i32>::new(10, 10),
                    &Vecteur2D::<i32>::new((nb_biome_w * width_per_biome - 1u32) as i32 , (nb_biome_h * height_per_biome - 1u32) as i32),
                    tile_size,
                ),
                16.0
            ),
            compteur_de_monde_genere,
            enemies
        }
    }

    fn generate_random_enemies(coord_min: Vecteur2D<i32>, coord_max: Vecteur2D<i32>, tile_size: u32, number: u32) -> Vec<Enemy> {
        (0..number)
            .map(|_| Self::generate_random_enemy(&coord_min, &coord_max, tile_size))
            .collect::<Vec<_>>()
    }

    fn generate_random_enemy(coord_min: &Vecteur2D<i32>, coord_max: &Vecteur2D<i32>, tile_size: u32) -> Enemy {
        let mut rnd = rand::thread_rng();

        let pos = Vecteur2D::<f32>::new(
            rnd.gen_range(coord_min.x..coord_max.x) as f32 * tile_size as f32,
            rnd.gen_range(coord_min.y..coord_max.y) as f32 * tile_size as f32,
        );

        Enemy::new(pos)
    }

    fn random_vaisseau(coord_min: &Vecteur2D<i32>, coord_max: &Vecteur2D<i32>, tile_size: u32) -> Vecteur2D<f32> {
        let mut rng = rand::thread_rng();

        let coord = match rng.gen_range(0..3) {
            0 => coord_max.clone(),
            1 => Vecteur2D::new(coord_min.x, coord_max.y),
            _ => Vecteur2D::new(coord_max.x, coord_min.y)
        };

        Vecteur2D::new(
            (coord.x * tile_size as i32) as f32,
            (coord.y * tile_size as i32) as f32,
        )

        // let x = rng.gen_range(min..max);
        // let y = rng.gen_range(min..max);
        //
        // Vecteur2D::new((x * 32) as f32 , (y * 32) as f32)
    }
}