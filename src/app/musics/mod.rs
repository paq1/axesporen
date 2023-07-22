use std::cell::RefCell;
use std::rc::Rc;
use crate::app::factories::music_factory::MusicFactory;
use crate::core::musics::CanPlayMusic;

pub struct MusicServiceImpl<'mf> {
    pub music_factory: Rc<RefCell<MusicFactory<'mf>>>
}

impl<'mf> CanPlayMusic for MusicServiceImpl<'mf> {
    fn play(&self, id: &str, volume: i32) -> Result<(), String> {
        sdl2::mixer::Music::set_volume(volume);

        self.music_factory.borrow().musics.get(id).map(|music| music.play(-1))
            .map(|_| Ok(()))
            .unwrap_or(Err(format!("musique : '{}' inconnue", id).to_string()))
    }

    fn play_sound(&self, id: &str, volume: i32) -> Result<(), String> {
        self.music_factory.borrow().sounds.get(id).map(|sound| {
            sdl2::mixer::Channel(1).set_volume(volume);
            sdl2::mixer::Channel(1).play(sound, 0).map(|_| ())
        })
            .unwrap_or(Err(format!("erreur lors de la lecture du son : {}", id)))
    }

    fn stop(&self) -> Result<(), String> {
        sdl2::mixer::Music::halt();
        Ok(())
    }
}