use std::cell::RefCell;
use std::rc::Rc;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

use crate::app::factories::font_factory::FontFactory;
use crate::core::graphics::CanDrawText;
use crate::core::graphics::models::color::Color as ColorCore;

pub struct TextServiceSDL<'a> {
    pub canvas: Rc<RefCell<WindowCanvas>>,
    pub texture_creator: TextureCreator<WindowContext>,
    pub texture_factory: Rc<RefCell<FontFactory<'a>>>
}

impl<'a> TextServiceSDL<'a> {
    pub fn new(
        canvas: Rc<RefCell<WindowCanvas>>,
        texture_factory: Rc<RefCell<FontFactory<'a>>>
    ) -> Self {
        let tc = canvas.borrow().texture_creator();

        Self {
            canvas,
            texture_creator: tc,
            texture_factory
        }
    }
}

impl CanDrawText for TextServiceSDL<'_> {
    fn create_text(
        &self,
        text: &str,
        x: i32,
        y: i32,
        font_size: u32,
        color: ColorCore,
    ) -> Result<(), String> {

        let color_sdl: Color = color.into();

        let surface = self.texture_factory.borrow().font
            .render(text)
            .blended(color_sdl)
            .map_err(|err| err.to_string())?;

        let texture = self.texture_creator
            .create_texture_from_surface(surface)
            .map_err(|err| err.to_string())?;

        let width = font_size * text.len() as u32;
        let height = font_size;

        let target = Rect::new(x, y, width, height);

        self.canvas.borrow_mut().copy(&texture, None, Some(target))?;

        Ok(())
    }
}

impl Into<Color> for ColorCore {
    fn into(self) -> Color {
        Color::RGBA(
            self.r,
            self.g,
            self.b,
            self.a
        )
    }
}
