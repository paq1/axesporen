use crate::core::graphics::models::color::Color;
use crate::core::sdd::vecteur2d::Vecteur2D;

pub mod models;

pub trait CanDrawText {
    fn create_text(
        &self,
        text: &str,
        x: i32,
        y :i32,
        font_size: u32,
        color: Color
    ) -> Result<(), String>;
}

pub trait CanDrawSprite {
    fn draw_sprite(
        &mut self,
        index: &str,
        position: Vecteur2D<i32>,
        from_size: Option<Vecteur2D<u32>>,
        to_size: Option<Vecteur2D<u32>>
    ) -> Result<(), String>;
}