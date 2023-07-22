extern crate sdl2;

use std::cell::RefCell;
use std::rc::Rc;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use crate::app::factories::font_factory::FontFactory;
use crate::app::factories::music_factory::MusicFactory;
use crate::app::factories::sprite_factory::SpriteFactory;
use crate::app::graphics::sprite_service_sdl2::SpriteServiceSdl2;
use crate::app::graphics::text_service_sdl::TextServiceSDL;
use crate::app::input::InputServiceImpl;
use crate::app::musics::MusicServiceImpl;
use crate::app::times::TimeServiceImpl;
use crate::core::graphics::CanDrawText;
use crate::core::graphics::models::color::Color;
use crate::core::input::CanManageInput;
use crate::core::scene::SceneManager;
use crate::core::sdd::vecteur2d::Vecteur2D;

pub mod core;
pub mod app;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let _audio = sdl_context.audio()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Axesporen", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize the video subsystem");
    let canvas = Rc::new(
        RefCell::new(window.into_canvas()
            .build()
            .expect("Failed to initialize canvas")
        )
    );
    let texture_creator = canvas.borrow().texture_creator();

    // todo -- factories -- ajoutez vos factories ici 🏭
    let font_factory: Rc<RefCell<FontFactory>> = Rc::new(
        RefCell::new(
            FontFactory::new(&ttf_context)?
        )
    );
    let music_factory: Rc<RefCell<MusicFactory>> = Rc::new(
        RefCell::new(
            MusicFactory::new()?
        )
    );
    let sprite_factory: Rc<RefCell<SpriteFactory>> = Rc::new(
        RefCell::new(
            SpriteFactory::new(&texture_creator)?
        )
    );

    // todo -- services -- instanciez vos services ici 🛸
    let music_service: Rc<RefCell<MusicServiceImpl>> = Rc::new(
        RefCell::new(
            MusicServiceImpl  {
                music_factory: Rc::clone(&music_factory)
            }
        )
    );
    let text_service: Rc<RefCell<TextServiceSDL>> = Rc::new(
        RefCell::new(
            TextServiceSDL::new(
                Rc::clone(&canvas),
                Rc::clone(&font_factory),
            )
        )
    );
    let input_service: Rc<RefCell<InputServiceImpl>> = Rc::new(
        RefCell::new(
            InputServiceImpl::new()
        )
    );
    let sprite_service: Rc<RefCell<SpriteServiceSdl2>> = Rc::new(RefCell::new(
        SpriteServiceSdl2 {
            canvas: Rc::clone(&canvas),
            sprite_factory: Rc::clone(&sprite_factory),
        }
    ));

    // todo -- initialisation du scene_manager 🤖
    let mut scene_manager = SceneManager::new(
        Rc::clone(&input_service),
        Rc::clone(&text_service),
        Rc::clone(&sprite_service),
        Rc::clone(&music_service)
    );

    let mut times = TimeServiceImpl::new();
    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        canvas.borrow_mut().clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode, .. } => {
                    match keycode {
                        None => {}
                        Some(key) => {
                            input_service.borrow_mut().key_down(key.to_string());
                        }
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    match keycode {
                        None => {}
                        Some(key) => {
                            input_service.borrow_mut().key_up(key.to_string());
                        }
                    }
                }
                Event::MouseMotion { x, y, .. } => {
                    input_service.borrow_mut().update_mouse_position(
                        Vecteur2D::new(x as f32, y as f32)
                    )
                }
                Event::MouseButtonDown {mouse_btn, ..} => {
                    match mouse_btn {
                        MouseButton::Left => input_service.borrow_mut().key_mouse_down("left".to_string()),
                        MouseButton::Right => input_service.borrow_mut().key_mouse_down("right".to_string()),
                        _ => {}
                    }
                }
                Event::MouseButtonUp {mouse_btn, ..} => {
                    match mouse_btn {
                        MouseButton::Left => input_service.borrow_mut().key_mouse_up("left".to_string()),
                        MouseButton::Right => input_service.borrow_mut().key_mouse_up("right".to_string()),
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // todo -- fps / dt ... etc ⏰
        let delta_time = times.calcul_delta_time();

        // todo -- boucle de gameplay 👾
        scene_manager.update_scene(delta_time);

        // debug afficha000
        text_service.borrow().create_text(
            format!("fps : {}", times.frames_per_sec).as_str(),
            800i32 - 5 * 32,
            0i32,
            14u32,
            Color::rgb(255u8, 0u8, 0u8),
        )?;

        canvas.borrow_mut().present();
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
