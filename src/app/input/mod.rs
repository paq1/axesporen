use std::collections::HashMap;
use crate::core::input::CanManageInput;
use crate::core::sdd::vecteur2d::Vecteur2D;

pub struct InputServiceImpl {
    pub key_manager: HashMap<String, bool>,
    pub key_mouse: HashMap<String, bool>,
    pub mouse_position: Vecteur2D<f32>
}

impl InputServiceImpl {
    pub fn new() -> Self {
        Self {
            key_manager: HashMap::new(),
            key_mouse: HashMap::new(),
            mouse_position: Vecteur2D::new(0f32, 0f32)
        }
    }
}

impl CanManageInput for InputServiceImpl {
    fn is_key_pressed(&self, value: &str) -> bool {
        *self.key_manager.get(value).unwrap_or(&false)
    }

    fn is_key_mouse_pressed(&self, value: &str) -> bool {
        *self.key_mouse.get(value).unwrap_or(&false)
    }

    fn key_down(&mut self, keyname: String) {
        if !self.key_manager.contains_key(&keyname) {
            self.key_manager.entry(keyname).or_insert(true);
        } else {
            if let Some(x) = self.key_manager.get_mut(&keyname) {
                *x = true;
            }
        }
    }

    fn key_mouse_down(&mut self, keyname: String) {
        if !self.key_mouse.contains_key(&keyname) {
            self.key_mouse.entry(keyname).or_insert(true);
        } else {
            if let Some(x) = self.key_mouse.get_mut(&keyname) {
                *x = true;
            }
        }
    }

    fn key_up(&mut self, keyname: String) {
        if !self.key_manager.contains_key(&keyname) {
            self.key_manager.entry(keyname).or_insert(false);
        } else {
            if let Some(x) = self.key_manager.get_mut(&keyname) {
                *x = false;
            }
        }
    }

    fn key_mouse_up(&mut self, keyname: String) {
        if !self.key_mouse.contains_key(&keyname) {
            self.key_mouse.entry(keyname).or_insert(false);
        } else {
            if let Some(x) = self.key_mouse.get_mut(&keyname) {
                *x = false;
            }
        }
    }

    fn key_pressed(&self) -> Vec<String> {
        self.key_manager
            .keys()
            .filter(|key| self.is_key_pressed(key))
            .map(|k| k.clone())
            .collect::<Vec<_>>()
    }

    fn mouse_key_pressed(&self) -> Vec<String> {
        self.key_mouse
            .keys()
            .filter(|key| self.is_key_mouse_pressed(key))
            .map(|k| k.clone())
            .collect::<Vec<_>>()
    }

    fn update_mouse_position(&mut self, position: Vecteur2D<f32>) {
        self.mouse_position = position
    }

    fn get_mouse_position(&self) -> Vecteur2D<f32> {
        self.mouse_position.clone()
    }
}