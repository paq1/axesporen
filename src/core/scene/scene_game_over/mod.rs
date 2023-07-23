pub mod scene_game_over_data;

use std::cell::RefCell;
use std::rc::Rc;

use crate::core::graphics::{CanDrawSprite, CanDrawText};
use crate::core::graphics::models::color::Color;
use crate::core::input::CanManageInput;
use crate::core::musics::CanPlayMusic;
use crate::core::scene::scene_game_over::scene_game_over_data::SceneGameOverData;
use crate::core::scene::SceneEnum;
use crate::core::scene::scene_menu::SceneMenu;

pub struct SceneGameOver<SpriteService, TextService, InputService, MusicService>
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
    pub data: SceneGameOverData
}

impl<SpriteService, TextService, InputService, MusicService> SceneGameOver<SpriteService, TextService, InputService, MusicService>
    where
        SpriteService: CanDrawSprite,
        TextService: CanDrawText,
        InputService: CanManageInput,
        MusicService: CanPlayMusic
{
    pub fn on_scene(
        &mut self,
        _dt: f32
    ) -> Option<SceneEnum<SpriteService, TextService, InputService, MusicService>> {

        self.init_scene().expect("erreur lors de l'initialisation du menu");

        let next_scene = self.change_scene();

        // self.draw_planetes().expect("erreur lors de l'affichage des planetes");

        self.draw_text_title();
        self.draw_text_for_change_scene();

        next_scene
    }

    pub fn new(
        key_manager: Rc<RefCell<InputService>>,
        text_service: Rc<RefCell<TextService>>,
        sprite_service: Rc<RefCell<SpriteService>>,
        music_service: Rc<RefCell<MusicService>>,
        nb_monde_decouvert: u32,
    ) -> Self {
        Self {
            input_service: key_manager,
            text_service,
            sprite_service,
            music_service,
            data: SceneGameOverData {
                is_init: false,
                nb_monde_decouvert
            }
        }
    }

    fn init_scene(&mut self) -> Result<(), String> {
        if !self.data.is_init {
            self.data.is_init = true;
            self.music_service.borrow().play("digital-love", 1)
        } else {
            Ok(())
        }
    }

    fn change_scene(&mut self) -> Option<SceneEnum<SpriteService, TextService, InputService, MusicService>> {
        if self.input_service.borrow().is_key_pressed("Escape") {
            self.music_service.borrow().stop().expect("erreur lors de l'arret de la musique");
            let scene_menu = SceneMenu::new(
                Rc::clone(&self.input_service),
                Rc::clone(&self.text_service),
                Rc::clone(&self.sprite_service),
                Rc::clone(&self.music_service)
            );
            Some(SceneEnum::SceneMenu(scene_menu))
        } else {
            None
        }
    }

    fn draw_text_title(&mut self) {
        self.text_service.borrow_mut()
            .create_text(
                format!("Game Over").as_str(),
                32 * 1,
                0 + 32 * 2,
                40u32,
                Color::rgb(100u8, 0u8, 200u8)
            ).expect("erreur lors de l'affichage");

        self.text_service.borrow_mut()
            .create_text(
                format!("explored {} worlds", self.data.nb_monde_decouvert).as_str(),
                32 * 2,
                0 + 32 * 4,
                30u32,
                Color::rgb(100u8, 0u8, 0u8)
            ).expect("erreur lors de l'affichage");
    }

    fn draw_text_for_change_scene(&mut self) {
        self.text_service.borrow_mut()
            .create_text(
                "[press Escape]",
                32 * 6,
                600 - 32 * 3,
                32u32,
                Color::rgb(255u8, 0u8, 0u8)
            ).expect("erreur lors de l'affichage");
    }
}