use std::cell::RefCell;
use std::rc::Rc;

use crate::core::graphics::{CanDrawSprite, CanDrawText};
use crate::core::graphics::models::color::Color;
use crate::core::input::CanManageInput;
use crate::core::musics::CanPlayMusic;
use crate::core::scene::SceneEnum;
use crate::core::scene::scene_world::SceneWorld;
use crate::core::scene::scene_menu::scene_menu_data::SceneMenuData;
use crate::core::sdd::vecteur2d::Vecteur2D;

pub mod scene_menu_data;

pub struct SceneMenu<SpriteService, TextService, InputService, MusicService>
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
    pub data: SceneMenuData
}

impl<SpriteService, TextService, InputService, MusicService> SceneMenu<SpriteService, TextService, InputService, MusicService>
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
        self.update_panel();


        self.draw_planetes().expect("erreur lors de l'affichage des planetes");

        if self.data.panel_draw {
            self.draw_panel().expect("erreur lors de l'affichage du panel");
        } else {
            self.draw_text_title();
            self.draw_text_for_change_scene();
        }

        next_scene
    }

    pub fn new(
        key_manager: Rc<RefCell<InputService>>,
        text_service: Rc<RefCell<TextService>>,
        sprite_service: Rc<RefCell<SpriteService>>,
        music_service: Rc<RefCell<MusicService>>,
    ) -> Self {
        Self {
            input_service: key_manager,
            text_service,
            sprite_service,
            music_service,
            data: SceneMenuData {
                is_init: false,
                panel_draw: true
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
        if self.input_service.borrow().is_key_pressed("Space") && !self.data.panel_draw {
            self.music_service.borrow().stop().expect("erreur lors de l'arret de la musique");
            let scene_exemple = SceneWorld::new(
                Rc::clone(&self.input_service),
                Rc::clone(&self.text_service),
                Rc::clone(&self.sprite_service),
                Rc::clone(&self.music_service),
                1
            );
            Some(SceneEnum::SceneWorld(scene_exemple))
        } else {
            None
        }
    }

    fn update_panel(&mut self) {
        if self.input_service.borrow().is_key_pressed("Escape") {
            self.data.panel_draw = false;
        }
    }

    fn draw_panel(&mut self) -> Result<(), String> {
        self.sprite_service.borrow_mut().draw_sprite(
            "panel",
            Vecteur2D::new(32, 32),
            Some(Vecteur2D::new(100, 100)),
            Some(Vecteur2D::new(800 - 32 * 2, 600 - 32 * 2))
        )?;

        self.text_service
            .borrow()
            .create_text(
                "Work in progress",
                32 * 5, 64,
                30,
                Color::rgb(200, 150, 0)
            )?;

        self.text_service
            .borrow()
            .create_text(
                "in coming :",
                32 * 3, 32 * 4,
                25,
                Color::rgb(200, 150, 0)
            )?;

        self.text_service
            .borrow()
            .create_text(
                "-> enemie generator",
                32 * 3, 32 * 5,
                25,
                Color::rgb(200, 150, 100)
            )?;

        self.text_service
            .borrow()
            .create_text(
                "-> weapons",
                32 * 3, 32 * 6,
                25,
                Color::rgb(200, 150, 100)
            )?;

        self.text_service
            .borrow()
            .create_text(
                "-> random buildings",
                32 * 3, 32 * 7,
                25,
                Color::rgb(200, 150, 100)
            )?;

        self.text_service
            .borrow()
            .create_text(
                "[Press Escape]",
                32 * 6, 32 * 15,
                30,
                Color::rgb(255, 0, 0)
            )
    }

    fn draw_planetes(&mut self) -> Result<(), String> {
        self.sprite_service.borrow_mut().draw_sprite(
            "planete_0",
            Vecteur2D::new(300, 300),
            Some(Vecteur2D::new(1280, 1280)),
            Some(Vecteur2D::new(600, 600))
        )?;

        self.sprite_service.borrow_mut().draw_sprite(
            "planete_2",
            Vecteur2D::new(200, 100),
            Some(Vecteur2D::new(1280, 1280)),
            Some(Vecteur2D::new(100, 100))
        )?;

        self.sprite_service.borrow_mut().draw_sprite(
            "planete_3",
            Vecteur2D::new(500, 100),
            Some(Vecteur2D::new(1280, 1280)),
            Some(Vecteur2D::new(20, 20))
        )?;

        self.sprite_service.borrow_mut().draw_sprite(
            "planete_1",
            Vecteur2D::new(100, 100),
            Some(Vecteur2D::new(1280, 1280)),
            Some(Vecteur2D::new(200, 200))
        )
    }


    fn draw_text_title(&mut self) {
        self.text_service.borrow_mut()
            .create_text(
                "Axesporen",
                32 * 1,
                0 + 32 * 2,
                40u32,
                Color::rgb(100u8, 0u8, 200u8)
            ).expect("erreur lors de l'affichage");
    }

    fn draw_text_for_change_scene(&mut self) {
        self.text_service.borrow_mut()
            .create_text(
                "[press space]",
                32 * 6,
                600 - 32 * 3,
                32u32,
                Color::rgb(255u8, 0u8, 0u8)
            ).expect("erreur lors de l'affichage");
    }
}