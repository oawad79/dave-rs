use macroquad::prelude::*;
use crate::Resources;

pub struct Menu {
    pub visible: bool,
    texture_name: String,
    texture_offset: Vec2,
    confirm_key: Option<KeyCode>,
    toggle_key: KeyCode,
}

impl Menu {
    pub fn new(
        texture_name: &str,
        texture_offset: Vec2,
        toggle_key: KeyCode,
        confirm_key: Option<KeyCode>,
    ) -> Self {
        Menu {
            visible: false,
            texture_name: texture_name.to_string(),
            texture_offset,
            confirm_key,
            toggle_key,
        }
    }
    
    pub fn update(&mut self) -> bool {
        if is_key_down(self.toggle_key) {
            self.visible = true;
        }
        
        if self.visible {
            if let Some(confirm_key) = self.confirm_key {
                if is_key_down(confirm_key) {
                    self.visible = false;
                    return true;
                } else if is_key_down(KeyCode::N) {
                    self.visible = false;
                }
            } else if is_key_pressed(KeyCode::Escape) {
                self.visible = false;
            }
        }
        
        false
    }
    
    pub fn draw(&self, resources: &Resources) {
        if self.visible {
            set_default_camera();
            draw_texture_ex(
                resources.get_texture(&self.texture_name),
                screen_width() / 2.0 + self.texture_offset.x,
                screen_height() / 2.0 + self.texture_offset.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(
                        resources.get_texture(&self.texture_name).width(),
                        resources.get_texture(&self.texture_name).height(),
                    )),
                    ..Default::default()
                },
            );
        }
    }
}
