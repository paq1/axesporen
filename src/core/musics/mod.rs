pub trait CanPlayMusic {
    fn play(&self, id: &str, volume: i32) -> Result<(), String>;
    fn play_sound(&self, id: &str, volume: i32) -> Result<(), String>;
    fn stop(&self) -> Result<(), String>;
}