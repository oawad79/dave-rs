use macroquad::{camera::set_default_camera, color::WHITE, input::{is_key_down, is_key_pressed, KeyCode}, math::{vec2, Vec2}, texture::{draw_texture_ex, DrawTextureParams}, window::{request_new_screen_size, screen_height, screen_width, set_fullscreen}};

use crate::{game::Game, resources::Resources, Scene};

pub struct InputManager {
    is_full_screen: bool
    
}   

impl InputManager {
    pub fn new() -> Self {
        Self {
            is_full_screen: false
        }
    }

    pub fn handle_cheat_code(scene: &mut Box<dyn Scene>) {
        if is_key_down(KeyCode::LeftControl) {
            for (i, key) in [
                KeyCode::Key0, KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4,
                KeyCode::Key5, KeyCode::Key6, KeyCode::Key7, KeyCode::Key8, KeyCode::Key9,
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

    pub fn handle_menu(&self,
        resources: &Resources,
        show_menu: &mut bool,
        key_to_show: KeyCode,
        texture_name: &str,
        texture_offset: Vec2,
        confirm_key: Option<KeyCode>,
    ) -> bool {
        if is_key_down(key_to_show) || *show_menu {
            set_default_camera();
            *show_menu = true;
            draw_texture_ex(
                resources.get_texture(texture_name),
                screen_width() / 2.0 + texture_offset.x,
                screen_height() / 2.0 + texture_offset.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(
                        resources.get_texture(texture_name).width(),
                        resources.get_texture(texture_name).height(),
                    )),
                    ..Default::default()
                },
            );
        }
    
        if let Some(confirm_key) = confirm_key {
            if *show_menu && is_key_down(confirm_key) {
                return true;
            } else if *show_menu && is_key_down(KeyCode::N) {
                *show_menu = false;
            }
        } else if *show_menu && is_key_pressed(KeyCode::Escape) {
            *show_menu = false;
        }
    
        false
    }
    
    
}