#![warn(
    clippy::all,
    clippy::pedantic,
    // clippy::restriction,
    clippy::nursery,
    clippy::cargo,
)]
//this is required to prevent macroquad from opening a
//console window in addition to the game window
//#![windows_subsystem = "windows"]

mod complete;
mod entry_screen;
mod game;
mod input_manager;
mod menu;
mod resources;
mod separator;
mod warp_zone;

use complete::Complete;
use entry_screen::EntryScreen;
use game::Game;
use input_manager::InputManager;
use macroquad::prelude::{
    collections::storage,
    *,
};
use menu::{
    Menu,
    MenuAction,
    MenuItem,
};
use resources::Resources;
use separator::Separator;
use warp_zone::WarpZone;

pub enum SceneChange {
    EntryScreen,
    Game {
        level: u32,
        retry: bool,
        cheat: bool,
        warp_zone: bool,
    },
    Separator,
    Complete,
    WarpZone,
}
pub trait Scene {
    fn update(&mut self) -> Option<SceneChange>;
    fn draw(&mut self);
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Dave".to_owned(),
        fullscreen: false,
        platform: miniquad::conf::Platform {
            linux_backend: miniquad::conf::LinuxBackend::WaylandOnly,
            ..Default::default()
        },
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");

    let _ = Resources::load().await;

    let main_camera = Camera2D::from_display_rect(Rect::new(0.0, 384.0, 608.0, -384.0));

    let mut scene: Box<dyn Scene> = Box::new(EntryScreen::new());

    let resources = storage::get::<Resources>();

    let mut input_manager = InputManager::new();

    // Create UI menus
    let mut menu = Menu::new(vec![
        MenuItem {
            key: KeyCode::F1,
            texture_name: "help".to_string(),
            texture_offset: vec2(-220.0, -120.0),
            confirm_key: Some(KeyCode::Y),
            negative_key: Some(KeyCode::N),
            action: Some(MenuAction::Help),
        },
        MenuItem {
            key: KeyCode::F9,
            texture_name: "pause".to_string(),
            texture_offset: vec2(-190.0, -30.0),
            confirm_key: Some(KeyCode::Y),
            negative_key: Some(KeyCode::N),
            action: Some(MenuAction::Pause),
        },
        MenuItem {
            key: KeyCode::F3,
            texture_name: "restart".to_string(),
            texture_offset: vec2(-190.0, -30.0),
            confirm_key: Some(KeyCode::Y),
            negative_key: Some(KeyCode::N),
            action: Some(MenuAction::Restart { confirm: false }),
        },
        MenuItem {
            key: KeyCode::Escape,
            texture_name: "exit".to_string(),
            texture_offset: vec2(-150.0, -20.0),
            confirm_key: Some(KeyCode::Y),
            negative_key: Some(KeyCode::N),
            action: Some(MenuAction::Exit { confirm: false }),
        },
    ]);

    loop {
        clear_background(BLACK);

        set_camera(&main_camera);

        // pause the game if any menu key pressed
        let change = if menu.current_menu_item.is_some() {
            None
        } else {
            scene.update()
        };

        if let Some(change) = change {
            scene = match change {
                SceneChange::EntryScreen => Box::new(EntryScreen::new()),
                SceneChange::Game {
                    level,
                    retry,
                    cheat,
                    warp_zone,
                } => Box::new(Game::new(level, retry, cheat, warp_zone)),
                SceneChange::WarpZone => Box::new(WarpZone::new()),
                SceneChange::Separator => Box::new(Separator::new()),
                SceneChange::Complete => Box::new(Complete::new()),
            };
        }

        scene.draw();

        if let Some(action) = menu.update(&resources) {
            match action {
                MenuAction::Exit { confirm } => {
                    if confirm {
                        break;
                    }
                }
                MenuAction::Pause | MenuAction::Help => {}
                MenuAction::Restart { confirm } => {
                    if confirm {
                        scene = Box::new(EntryScreen::new());
                    }
                }
            }
        }

        InputManager::handle_cheat_code(&mut scene);

        input_manager.toggle_fullscreen();

        next_frame().await;
    }
}
