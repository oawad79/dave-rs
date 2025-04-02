use macroquad::{
    input::{
        KeyCode,
        is_key_down,
        is_key_pressed,
    },
    window::{
        request_new_screen_size,
        set_fullscreen,
    },
};

use crate::{
    Scene,
    game::Game,
};

pub struct InputManager {
    is_full_screen: bool,
}

impl InputManager {
    pub const fn new() -> Self {
        Self {
            is_full_screen: false,
        }
    }

    pub fn handle_cheat_code(scene: &mut Box<dyn Scene>) {
        if is_key_down(KeyCode::LeftControl) {
            for (i, key) in [
                KeyCode::Key0,
                KeyCode::Key1,
                KeyCode::Key2,
                KeyCode::Key3,
                KeyCode::Key4,
                KeyCode::Key5,
                KeyCode::Key6,
                KeyCode::Key7,
                KeyCode::Key8,
                KeyCode::Key9,
            ]
            .iter()
            .enumerate()
            {
                if is_key_down(*key) {
                    if let Ok(level) = u32::try_from(i) {
                        *scene = Box::new(Game::new(level, false, true, false));
                    }
                }
            }
        }
    }

    pub fn toggle_fullscreen(&mut self) {
        if is_key_pressed(KeyCode::A) && is_key_down(KeyCode::LeftControl) {
            self.is_full_screen = !self.is_full_screen;

            set_fullscreen(self.is_full_screen);
            if !self.is_full_screen {
                request_new_screen_size(1000.0, 650.0);
            }
        }
    }
}
