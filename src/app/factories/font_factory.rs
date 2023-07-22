use std::path::Path;
use sdl2::ttf::{Font, Sdl2TtfContext};

pub struct FontFactory<'a> {
    pub font: Font<'a, 'a>
}

impl<'a> FontFactory<'a> {
    pub fn new(ctx: &'a Sdl2TtfContext) -> Result<FontFactory<'a>, String> {
        let font_path: &Path = Path::new("assets/fonts/dpcomic.ttf");
        let mut font: Font<'a, 'a> = ctx.load_font(font_path, 32)?;
        font.set_style(sdl2::ttf::FontStyle::BOLD);
        Ok(
            Self {
                font
            }
        )
    }
}