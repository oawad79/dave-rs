use macroquad::prelude::*;

use crate::Resources;

#[derive(Clone)]
pub enum MenuAction {
    Exit,
    Pause,
    Help,
    Restart,
}

pub struct Menu {
    pub current_menu_item: Option<MenuItem>,
    menu_items: Vec<MenuItem>,
}

pub struct MenuItem {
    pub key: KeyCode,
    pub texture_name: String,
    pub texture_offset: Vec2,
    pub confirm_key: Option<KeyCode>,
    pub action: Option<MenuAction>,
}

impl Menu {
    pub const fn new(menu_items: Vec<MenuItem>) -> Self {
        Self {
            current_menu_item: None,
            menu_items,
        }
    }

    pub fn update(&mut self, resources: &Resources) -> Option<MenuAction> {
        if self.current_menu_item.is_none() {
            for menu_item in &self.menu_items {
                if is_key_down(menu_item.key) {
                    self.current_menu_item = Some(MenuItem {
                        key: menu_item.key,
                        texture_name: menu_item.texture_name.clone(),
                        texture_offset: menu_item.texture_offset,
                        confirm_key: menu_item.confirm_key,
                        action: menu_item.action.clone(),
                    });
                }
            }
        }

        if self.current_menu_item.is_some() {
            let menu_item = self.current_menu_item.as_ref().unwrap();
            set_default_camera();
            draw_texture_ex(
                resources.get_texture(&menu_item.texture_name),
                screen_width() / 2.0 + menu_item.texture_offset.x,
                screen_height() / 2.0 + menu_item.texture_offset.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(
                        resources.get_texture(&menu_item.texture_name).width(),
                        resources.get_texture(&menu_item.texture_name).height(),
                    )),
                    ..Default::default()
                },
            );

            if is_key_down(menu_item.confirm_key.unwrap()) {
                let m = menu_item.action.clone();
                self.current_menu_item = None;
                return m;
            }
        }

        None
    }
}
