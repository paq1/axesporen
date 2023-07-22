use crate::core::sdd::vecteur2d::Vecteur2D;

pub trait CanManageInput {
    fn is_key_pressed(&self, value: &str) -> bool;
    fn is_key_mouse_pressed(&self, value: &str) -> bool;

    fn key_down(&mut self, keyname: String);
    fn key_mouse_down(&mut self, keyname: String);

    fn key_up(&mut self, keyname: String);
    fn key_mouse_up(&mut self, keyname: String);

    fn key_pressed(&self) -> Vec<String>;
    fn mouse_key_pressed(&self) -> Vec<String>;

    fn update_mouse_position(&mut self, position: Vecteur2D<f32>);

    fn get_mouse_position(&self) -> Vecteur2D<f32>;
}