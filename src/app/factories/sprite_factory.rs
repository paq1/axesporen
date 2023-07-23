use std::collections::HashMap;
use std::path::Path;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

pub struct SpriteFactory<'t> {
    pub sprites: HashMap<&'t str, Texture<'t>>
}

impl<'a> SpriteFactory<'a> {
    pub fn new(tc: &'a TextureCreator<WindowContext>) -> Result<SpriteFactory<'a>, String> {
        let spite_smiley_path: &Path = Path::new("assets/sprites/smiley_sdl_seed.bmp");
        let spite_smiley: Texture<'a> = tc.load_texture(spite_smiley_path)?;

        let tile_herbe_path: &Path = Path::new("assets/sprites/tiles/tileGrass.png");
        let tile_herbe: Texture<'a> = tc.load_texture(tile_herbe_path)?;

        let tile_brique_path: &Path = Path::new("assets/sprites/tiles/tileStone.png");
        let tile_brique: Texture<'a> = tc.load_texture(tile_brique_path)?;

        let tile_sand_path: &Path = Path::new("assets/sprites/tiles/tileSand.png");
        let tile_sand: Texture<'a> = tc.load_texture(tile_sand_path)?;

        let tile_snow_path: &Path = Path::new("assets/sprites/tiles/tileSnow.png");
        let tile_snow: Texture<'a> = tc.load_texture(tile_snow_path)?;

        let tile_goo_path: &Path = Path::new("assets/sprites/tiles/tileGoo.png");
        let tile_goo: Texture<'a> = tc.load_texture(tile_goo_path)?;

        let tile_wood_path: &Path = Path::new("assets/sprites/tiles/tileWood.png");
        let tile_wood: Texture<'a> = tc.load_texture(tile_wood_path)?;

        let poulet_path: &Path = Path::new("assets/sprites/animals/chicken.png");
        let poulet: Texture<'a> = tc.load_texture(poulet_path)?;

        let croco_path: &Path = Path::new("assets/sprites/animals/crocodile.png");
        let croco: Texture<'a> = tc.load_texture(croco_path)?;

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

        let panel_path: &Path = Path::new("assets/sprites/panel/glassPanel.png");
        let panel: Texture<'a> = tc.load_texture(panel_path)?;

        let sprites: HashMap<&str, Texture> = [
            ("smiley", spite_smiley),
            ("poulet", poulet),
            ("croco", croco),
            ("porte", door),
            ("tile_herbe", tile_herbe),
            ("tile_brique", tile_brique),
            ("tile_sand", tile_sand),
            ("tile_snow", tile_snow),
            ("tile_goo", tile_goo),
            ("tile_wood", tile_wood),
            ("viseur", viseur),
            ("planete_0", planet_0),
            ("planete_1", planet_1),
            ("planete_2", planet_2),
            ("planete_3", planet_3),
            ("panel", panel),
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