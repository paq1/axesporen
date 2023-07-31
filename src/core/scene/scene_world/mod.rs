use std::cell::RefCell;
use std::rc::Rc;

use crate::core::elements::tilemap::tile::TileType;
use crate::core::elements::tilemap::TileMap;
use crate::core::graphics::{CanDrawSprite, CanDrawText};
use crate::core::graphics::models::color::Color;
use crate::core::input::CanManageInput;
use crate::core::musics::CanPlayMusic;
use crate::core::physics::collide_body::CanCollideWithTileMap;
use crate::core::scene::SceneEnum;
use crate::core::scene::scene_game_over::SceneGameOver;
use crate::core::scene::scene_world::enemy::Enemy;
use crate::core::scene::scene_world::scene_world_data::SceneWorldData;
use crate::core::sdd::vecteur2d::Vecteur2D;

pub mod scene_world_data;
pub mod player;
pub mod enemy;

pub struct SceneWorld<SpriteService, TextService, InputService, MusicService>
    where
        SpriteService: CanDrawSprite,
        TextService: CanDrawText,
        InputService: CanManageInput,
        MusicService: CanPlayMusic
{
    pub input_service: Rc<RefCell<InputService>>,
    pub text_service: Rc<RefCell<TextService>>,
    pub sprite_service: Rc<RefCell<SpriteService>>,
    pub music_service: Rc<RefCell<MusicService>>,
    pub data: SceneWorldData
}

impl<SpriteService, TextService, InputService, MusicService> SceneWorld<SpriteService, TextService, InputService, MusicService>
    where
        SpriteService: CanDrawSprite,
        TextService: CanDrawText,
        InputService: CanManageInput,
        MusicService: CanPlayMusic
{
    pub fn on_scene(
        &mut self,
        dt: f32
    ) -> Option<SceneEnum<SpriteService, TextService, InputService, MusicService>> {

        self.init_scene().expect("erreur lors de l'initialisation de la scene");

        self.update_player(dt).expect("erreur lors de l'update du player");
        self.update_enemies(dt);
        self.update_curseur();
        self.update_camera();
        self.test_play_sound();

        self.draw_one_tilemap_upgraded(&self.data.tilemap).expect("erreur lors de l'affichage de la map");
        self.draw_vaisseau_a_trouver().expect("erreur lors de l'affichage du vaisseau");
        self.draw_player().expect("erreur lors de l'affichage du player");
        self.draw_enemies().expect("erreur lors de l'affichage du player");
        self.draw_cursor().expect("erreur lors de l'affichage du curseur");

        let _keys_pressed = self.get_keys_pressed();
        let _mouse_key_pressed = self.get_mouse_keys_pressed();

        let font_size = 14u32;
        let _pos = self.input_service.borrow_mut().get_mouse_position();

        self.sprite_service.borrow_mut().draw_sprite(
            "panel",
            Vecteur2D::new(0, 0),
            Some(Vecteur2D::new(100, 100)),
            Some(Vecteur2D::new(400, 100))
        ).expect("erreur affichage panel");


        vec![
            format!("count enemies = {}", self.data.enemies.len()),
            format!("lvl {}", self.data.compteur_de_monde_genere)
        ]
            .iter()
            .enumerate()
            .for_each(|(index, debug_str)| {
                self.text_service.borrow_mut().create_text(
                    debug_str.as_str(),
                    32i32,
                    font_size as i32 * index as i32 + 32,
                    font_size,
                    Color::rgb(255u8, 0u8, 255u8)
                ).expect("erreur lors de l'affichage");
            });

        if self.data.vaisseau_a_trouver.is_collide_with_object(&self.data.player.pos, 16.0) {
            Some(
                SceneEnum::SceneWorld(
                    SceneWorld::new(
                        Rc::clone(&self.input_service),
                        Rc::clone(&self.text_service),
                        Rc::clone(&self.sprite_service),
                        Rc::clone(&self.music_service),
                        self.data.compteur_de_monde_genere + 1
                    )
                )
            )
        } else if self.player_collide_with_enemy() {
            Some(
                SceneEnum::SceneGameOver(
                    SceneGameOver::new(
                        Rc::clone(&self.input_service),
                        Rc::clone(&self.text_service),
                        Rc::clone(&self.sprite_service),
                        Rc::clone(&self.music_service),
                        self.data.compteur_de_monde_genere
                    )
                )
            )
        } else {
            None
        }

    }

    fn get_keys_pressed(&self) -> String {
        self
            .input_service
            .borrow()
            .key_pressed()
            .join("-")
    }

    fn get_mouse_keys_pressed(&self) -> String {
        self
            .input_service
            .borrow()
            .mouse_key_pressed()
            .join("-")
    }

    pub fn new(
        key_manager: Rc<RefCell<InputService>>,
        text_service: Rc<RefCell<TextService>>,
        sprite_service: Rc<RefCell<SpriteService>>,
        music_service: Rc<RefCell<MusicService>>,
        compteur_de_monde_genere: u32
    ) -> Self {
        Self {
            input_service: key_manager,
            text_service,
            sprite_service,
            music_service,
            data: SceneWorldData::new(compteur_de_monde_genere)
        }
    }

    fn init_scene(&mut self) -> Result<(), String> {
        if !self.data.is_init {
            self.data.is_init = true;
            self.music_service.borrow().play("hold-the-line", 20)
        } else {
            Ok(())
        }
    }

    fn update_player(&mut self, dt: f32) -> Result<(), String> {
        let vitesse = self.data.player.vitesse;
        let vitesse_temps = vitesse * dt;

        if self.input_service.borrow().is_key_pressed("Z") {

            let mut col_body = self.data.player.collide_body.clone();
            col_body.position.y -= vitesse_temps;

            if !col_body.is_collide(&self.data.tilemap, vec![TileType::Mur]) {
                self.data.player.pos.y -= vitesse_temps;
                self.data.player.collide_body.position.y -= vitesse_temps;
            }
        }
        if self.input_service.borrow().is_key_pressed("D") {

            let mut col_body = self.data.player.collide_body.clone();
            col_body.position.x += vitesse_temps;

            if !col_body.is_collide(&self.data.tilemap, vec![TileType::Mur]) {
                self.data.player.pos.x += vitesse_temps;
                self.data.player.collide_body.position.x += vitesse_temps;
            }
        }
        if self.input_service.borrow().is_key_pressed("S") {

            let mut col_body = self.data.player.collide_body.clone();
            col_body.position.y += vitesse_temps;

            if !col_body.is_collide(&self.data.tilemap, vec![TileType::Mur]) {
                self.data.player.pos.y += vitesse_temps;
                self.data.player.collide_body.position.y += vitesse_temps;
            }
        }
        if self.input_service.borrow().is_key_pressed("Q") {

            let mut col_body = self.data.player.collide_body.clone();
            col_body.position.x -= vitesse_temps;

            if !col_body.is_collide(&self.data.tilemap, vec![TileType::Mur]) {
                self.data.player.pos.x -= vitesse_temps;
                self.data.player.collide_body.position.x -= vitesse_temps;
            }
        }

        Ok(())
    }

    fn update_enemies(&mut self, dt: f32) {
        self.data.enemies.iter_mut().for_each(|e: &mut Enemy| e.update(dt, &self.data.tilemap, &self.data.player));
    }

    fn update_curseur(&mut self) {
        // on recup la pos du joueur et de la souris
        let pos_joueur = self.data.player.pos.clone();
        let pos_souris = self.input_service.borrow().get_mouse_position() + self.data.camera.clone();

        // on recupere le vecteur entre ces 2 points et on prend sa valeur unitaire
        let vec_joueur_curseur = Vecteur2D::<f32>::from_points(&pos_joueur, &pos_souris);
        let vec_joueur_curseur_unitaire = vec_joueur_curseur.unitaire();

        let distance_souris_joureur = vec_joueur_curseur.norme();

        // on met a jour la position du curseur uniquement si le calcul unitaire est possible
        match vec_joueur_curseur_unitaire {
            Some(unitaire) => {
                let distance_min = 32.0;
                let distance_max = distance_min * 2.0;

                let distance_viseur = if distance_souris_joureur > distance_max {
                    distance_max
                } else if distance_souris_joureur < distance_min {
                    distance_min
                } else {
                    distance_souris_joureur
                };

                self.data.pos_curseur = pos_joueur.clone() + Vecteur2D::new(
                    unitaire.x * distance_viseur,
                    unitaire.y * distance_viseur
                )
            }
            _ => ()
        }
    }

    fn player_collide_with_enemy(&self) -> bool {
        self.data.enemies.iter()
            .find(|e| {
                Vecteur2D::<f32>::from_points(&e.collide_body.position, &self.data.player.pos)
                    .norme() < 32.0
            })
            .is_some()
    }

    fn update_camera(&mut self) {
        let window_width = 800f32; // fixme utiliser un service window afin de recup les infos de la window
        let window_height = 600f32; // fixme utiliser un service window afin de recup les infos de la window
        // let vec_player = self.data.player.pos.clone();
        self.data.camera = Vecteur2D::new(
            self.data.player.pos.x - window_width / 2.0,
            self.data.player.pos.y - window_height / 2.0,
        );
    }

    fn draw_player(&mut self) -> Result<(), String> {
        self.sprite_service.borrow_mut().draw_sprite(
            "poulet",
            Vecteur2D::new(
                (self.data.player.pos.x - self.data.camera.x - 16f32) as i32,
                (self.data.player.pos.y - self.data.camera.y - 16f32) as i32
            )
            ,Some(Vecteur2D::new(128, 128)), Some(Vecteur2D::new(32, 32))
        )
    }

    fn draw_enemies(&mut self) -> Result<(), String> {
        self.data.enemies
            .clone()
            .iter()
            .map(|e| e.clone())
            .for_each(|e| {
                self.draw_enemy(&e).expect("erreur affichage")
            });

        Ok(())
    }

    fn draw_enemy(&mut self, enemy: &Enemy) -> Result<(), String> {
        self.sprite_service.borrow_mut().draw_sprite(
            "croco",
            Vecteur2D::new(
                (enemy.collide_body.position.x - self.data.camera.x - 16f32) as i32,
                (enemy.collide_body.position.y - self.data.camera.y - 16f32) as i32
            )
            ,Some(Vecteur2D::new(128, 128)), Some(Vecteur2D::new(32, 32))
        )
    }

    fn draw_vaisseau_a_trouver(&mut self) -> Result<(), String> {
        self.sprite_service.borrow_mut().draw_sprite(
            "porte",
            Vecteur2D::new(
                (self.data.vaisseau_a_trouver.position.x - self.data.camera.x - 16f32) as i32,
                (self.data.vaisseau_a_trouver.position.y - self.data.camera.y - 16f32) as i32
            )
            ,Some(Vecteur2D::new(64, 64)), Some(Vecteur2D::new(32, 32))
        )
    }

    fn draw_cursor(&mut self) -> Result<(), String> {
        self.sprite_service.borrow_mut().draw_sprite(
            "viseur",
            Vecteur2D::new(
                (self.data.pos_curseur.x - self.data.camera.x - 16f32) as i32,
                (self.data.pos_curseur.y - self.data.camera.y - 16f32) as i32
            )
            , Some(Vecteur2D::new(512, 512)), None
        )
    }

    fn draw_one_tilemap_upgraded(&self, tilemap: &TileMap) -> Result<(), String> {
        let width = 26;
        let height = 20;
        let index_camera = tilemap.get_indexes_from_position(&self.data.camera);
        let coord_min = Vecteur2D::new(
            {
                if index_camera.x < 0 {
                    0
                } else {
                    index_camera.x
                }
            },
            {
                if index_camera.y < 0 {
                    0
                } else {
                    index_camera.y
                }
            }
        );
        let coord_max = Vecteur2D::new(
            {
                if index_camera.x + width > tilemap.tiles[0].len() as i32 {
                    tilemap.tiles[0].len() as i32
                } else {
                    index_camera.x + width
                }
            },
            {
                if index_camera.y + height > tilemap.tiles.len() as i32 {
                    tilemap.tiles.len() as i32
                } else {
                    index_camera.y + height
                }
            }
        );

        let mut sp_service = self.sprite_service.borrow_mut();

        for ligne in coord_min.y .. coord_max.y {
            for colonne in coord_min.x .. coord_max.x {
                let current = tilemap.tiles.get(ligne as usize).unwrap().get(colonne as usize).unwrap();
                let sprite_index = match current.r#type {
                    TileType::Mur => "tile_brique",
                    TileType::Sand => "tile_sand",
                    TileType::Snow => "tile_snow",
                    TileType::Goo => "tile_goo",
                    TileType::Wood => "tile_wood",
                    _ => "tile_herbe"
                };

                sp_service.draw_sprite(
                    sprite_index,
                    Vecteur2D::new(
                        current.pos.x as i32 * 32 - self.data.camera.x as i32,
                        current.pos.y as i32 * 32 - self.data.camera.y as i32
                    )
                    , Some(Vecteur2D::new(64, 74)), Some(Vecteur2D::new(32, 51))
                ).expect("erreur de lors de la 'affiche de la tuile");
            }
        }

        Ok(())
    }

    // fn is_in_screen(point_x: i32, point_y: i32) -> bool {
    //     let window_width = 800;
    //     let window_height = 600;
    //     let margin = 100;
    //     // fixme utilise un service window (pas encore dev) afin de recupere ces info
    //     point_x > 0 - margin && point_x < window_width && point_y > 0 - margin && point_y < window_height
    // }

    fn test_play_sound(&self) {
        if self.input_service.borrow().is_key_pressed("X") {
            self.music_service.borrow().play_sound("arme", 1).expect("erreur lors de la lecture du son arme");
        }
    }
}