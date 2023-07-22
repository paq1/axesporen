use std::cell::RefCell;
use std::rc::Rc;
use crate::core::graphics::{CanDrawSprite, CanDrawText};
use crate::core::input::CanManageInput;
use crate::core::musics::CanPlayMusic;
use crate::core::scene::scene_exemple::SceneExemple;
use crate::core::scene::scene_menu::SceneMenu;

pub mod scene_menu;
pub mod scene_exemple;

pub enum SceneEnum<SpriteService, TextService, InputService, MusicService>
    where
        SpriteService: CanDrawSprite,
        TextService: CanDrawText,
        InputService: CanManageInput,
        MusicService: CanPlayMusic
{
    SceneMenu(SceneMenu<SpriteService, TextService, InputService, MusicService>),
    SceneExemple(SceneExemple<SpriteService, TextService, InputService, MusicService>),
}

pub struct SceneManager<SpriteService, TextService, InputService, MusicService>
    where
        SpriteService: CanDrawSprite,
        TextService: CanDrawText,
        InputService: CanManageInput,
        MusicService: CanPlayMusic
{
    pub current: SceneEnum<SpriteService, TextService, InputService, MusicService>,
}

impl<SpriteService, TextService, InputService, MusicService> SceneManager<SpriteService, TextService, InputService, MusicService>
    where
        SpriteService: CanDrawSprite,
        TextService: CanDrawText,
        InputService: CanManageInput,
        MusicService: CanPlayMusic
{
    pub fn new(
        key_manager: Rc<RefCell<InputService>>,
        text_service: Rc<RefCell<TextService>>,
        sprite_service: Rc<RefCell<SpriteService>>,
        music_service: Rc<RefCell<MusicService>>,
    ) -> Self {
        let scene_menu = SceneMenu::new(
            Rc::clone(&key_manager),
            Rc::clone(&text_service),
            Rc::clone(&sprite_service),
            Rc::clone(&music_service)
        );
        Self { current: SceneEnum::SceneMenu(scene_menu) }
    }

    pub fn update_scene(&mut self, dt: f32) {
        let nouvelle_scene = match &mut self.current {
            SceneEnum::SceneMenu(menu) => menu.on_scene(dt),
            SceneEnum::SceneExemple(exemple) => exemple.on_scene(dt)
        };

        if let Some(x) = nouvelle_scene {
            self.current = x;
        }
    }
}
