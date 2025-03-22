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
mod entry_screen;
mod game;
mod monster;
mod player;
mod resources;
mod score_board;
mod separator;
mod complete;
mod warp_zone;
mod input_manager;

use game::Game;
use entry_screen::EntryScreen;
use input_manager::InputManager;
use separator::Separator;
use resources::Resources;
use complete::Complete;
use macroquad::prelude::{collections::storage, *};
use warp_zone::WarpZone;

pub enum SceneChange {
    EntryScreen,
    Game{level: u32, retry: bool, cheat: bool, warp_zone: bool},
    Separator,
    Complete,
    WarpZone
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

    let mut show_quit = false;
    let mut show_help = false;
    let mut show_restart = false;

    let mut input_manager = InputManager::new();

    loop {
        clear_background(BLACK);
        
        set_camera(&main_camera);

        let change = scene.update();
        if let Some(change) = change {
            scene = match change {
                SceneChange::EntryScreen => Box::new(EntryScreen::new()),
                SceneChange::Game{level, retry, cheat, warp_zone} => Box::new(Game::new(level, retry, cheat, warp_zone)),
                SceneChange::WarpZone => Box::new(WarpZone::new()),
                SceneChange::Separator => Box::new(Separator::new()),
                SceneChange::Complete => Box::new(Complete::new()),
            };
        }

        scene.draw();

        input_manager.handle_menu(
            &resources,
            &mut show_help,
            KeyCode::F1,
            "help",
            vec2(-220.0, -120.0),
            None,
        );
        
        if input_manager.handle_menu(
            &resources,
            &mut show_restart,
            KeyCode::F3,
            "restart",
            vec2(-190.0, -30.0),
            Some(KeyCode::Y),
        ) {
            scene = Box::new(EntryScreen::new());
            show_restart = false;
        }

        if input_manager.handle_menu(
            &resources,
            &mut show_quit,
            KeyCode::Escape,
            "exit",
            vec2(-150.0, -20.0),
            Some(KeyCode::Y),
        ) {
            break;
        }

        InputManager::handle_cheat_code(&mut scene);

        input_manager.toggle_fullscreen();
        
        next_frame().await;
    }
}

