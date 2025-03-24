// #![warn(
//     clippy::all,
//     clippy::pedantic,
//     // clippy::restriction,
//     clippy::nursery,
//     clippy::cargo,
// )]

//this is required to prevent macroquad from opening a
//console window in addition to the game window
//#![windows_subsystem = "windows"]

mod bullet;
mod complete;
mod entry_screen;
mod game;
mod input_manager;
mod menu;
mod monster;
mod player;
mod resources;
mod score_board;
mod separator;
mod warp_zone;


use complete::Complete;
use entry_screen::EntryScreen;
use game::Game;
use input_manager::InputManager;
use macroquad::prelude::{collections::storage, *};
use menu::Menu;
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
    fn draw(&self);
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Dave".to_owned(),
        fullscreen: false,
        platform: miniquad::conf::Platform {
            linux_backend: miniquad::conf::LinuxBackend::WaylandOnly,
            ..Default::default()
        },
        window_width: 1000,
        window_height: 650,
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
    let mut help_menu = Menu::new(
        "help", 
        vec2(-220.0, -120.0), 
        KeyCode::F1, 
        None);
        
    let mut restart_menu = Menu::new(
        "restart",
        vec2(-190.0, -30.0),
        KeyCode::F3,
        Some(KeyCode::Y),
    );
    let mut quit_menu = Menu::new(
        "exit",
        vec2(-150.0, -20.0),
        KeyCode::Escape,
        Some(KeyCode::Y),
    );

    loop {
        clear_background(BLACK);

        set_camera(&main_camera);

        let change = scene.update();
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

        // Handle menus
        help_menu.update();
        help_menu.draw(&resources);

        if restart_menu.update() {
            scene = Box::new(EntryScreen::new());
        }
        restart_menu.draw(&resources);

        if quit_menu.update() {
            break;
        }
        quit_menu.draw(&resources);

        InputManager::handle_cheat_code(&mut scene);

        input_manager.toggle_fullscreen();

        next_frame().await;
    }
}
