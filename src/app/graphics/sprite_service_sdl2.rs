use std::cell::RefCell;
use std::rc::Rc;

use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

use crate::app::factories::sprite_factory::SpriteFactory;
use crate::core::graphics::CanDrawSprite;
use crate::core::sdd::vecteur2d::Vecteur2D;

pub struct SpriteServiceSdl2<'sf> {
    pub canvas: Rc<RefCell<WindowCanvas>>,
    pub sprite_factory: Rc<RefCell<SpriteFactory<'sf>>>
}

impl<'sf> CanDrawSprite for SpriteServiceSdl2<'sf> {
    fn draw_sprite(&mut self, index: &str, position: Vecteur2D<i32>, from_size: Option<Vecteur2D<u32>>, to_size: Option<Vecteur2D<u32>>) -> Result<(), String> {

        let fact = self.sprite_factory.borrow();
        let sprite = fact.sprites.get(index).expect(format!("erreur sprite {} inconnu", index).as_str());

        let from = from_size.unwrap_or(Vecteur2D::new(32, 32));
        let to = to_size.unwrap_or(Vecteur2D::new(32, 32));

        self.canvas.borrow_mut().copy_ex(
            sprite,
            Rect::new(0, 0, from.x, from.y),
            Rect::new(position.x, position.y,to.x,to.y),
            0.0,
            Point::new(16, 16),
            false,
            false
        )?;

        Ok(())
    }
}