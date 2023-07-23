use std::collections::HashMap;
use std::path::Path;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

pub struct SpriteFactory<'t> {
    pub sprites: HashMap<&'t str, Texture<'t>>,
    // pub spite_smiley: Texture<'t>
}

impl<'a> SpriteFactory<'a> {
    pub fn new(tc: &'a TextureCreator<WindowContext>) -> Result<SpriteFactory<'a>, String> {
        let spite_smiley_path: &Path = Path::new("assets/sprites/smiley_sdl_seed.bmp");
        let spite_smiley: Texture<'a> = tc.load_texture(spite_smiley_path)?;

        let tile_herbe_path: &Path = Path::new("assets/sprites/tiles/tileGrass.png");
        let tile_herbe: Texture<'a> = tc.load_texture(tile_herbe_path)?;

        let tile_brique_path: &Path = Path::new("assets/sprites/tiles/tileStone.png");
        let tile_brique: Texture<'a> = tc.load_texture(tile_brique_path)?;

        let poulet_path: &Path = Path::new("assets/sprites/animals/chicken.png");
        let poulet: Texture<'a> = tc.load_texture(poulet_path)?;

        let door_path: &Path = Path::new("assets/sprites/doors/porte.png");
        let door: Texture<'a> = tc.load_texture(door_path)?;

        let planet_0_path: &Path = Path::new("assets/sprites/planetes/planet00.png");
        let planet_0: Texture<'a> = tc.load_texture(planet_0_path)?;

        let planet_1_path: &Path = Path::new("assets/sprites/planetes/planet01.png");
        let planet_1: Texture<'a> = tc.load_texture(planet_1_path)?;

        let planet_2_path: &Path = Path::new("assets/sprites/planetes/planet02.png");
        let planet_2: Texture<'a> = tc.load_texture(planet_2_path)?;

        let planet_3_path: &Path = Path::new("assets/sprites/planetes/planet03.png");
        let planet_3: Texture<'a> = tc.load_texture(planet_3_path)?;

        let viseur_path: &Path = Path::new("assets/sprites/curseur/curseur.png");
        let viseur: Texture<'a> = tc.load_texture(viseur_path)?;

        let sprites: HashMap<&str, Texture> = [
            ("smiley", spite_smiley),
            ("poulet", poulet),
            ("porte", door),
            ("tile_herbe", tile_herbe),
            ("tile_brique", tile_brique),
            ("viseur", viseur),
            ("planete_0", planet_0),
            ("planete_1", planet_1),
            ("planete_2", planet_2),
            ("planete_3", planet_3),
        ]
            .into_iter()
            .collect::<HashMap<&str, Texture>>();

        Ok(
            Self {
                sprites
            }
        )
    }
}