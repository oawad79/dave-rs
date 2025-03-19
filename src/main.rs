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

use game::Game;
use entry_screen::EntryScreen;
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

    let mut is_full_screen: bool = false;

    let _ = Resources::load().await;
    
    let main_camera = Camera2D::from_display_rect(Rect::new(0.0, 384.0, 608.0, -384.0));
    
    let mut scene: Box<dyn Scene> = Box::new(EntryScreen::new());
    
    let resources = storage::get::<Resources>();

    let mut show_quit = false;
    let mut show_help = false;
    let mut show_restart = false;

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

        handle_menu(
            &resources,
            &mut show_help,
            KeyCode::F1,
            "help",
            vec2(-220.0, -120.0),
            None,
        );
        
        if handle_menu(
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

        if handle_menu(
            &resources,
            &mut show_quit,
            KeyCode::Escape,
            "exit",
            vec2(-150.0, -20.0),
            Some(KeyCode::Y),
        ) {
            break;
        }

        handle_cheat_code(&mut scene);

        if is_key_pressed(KeyCode::A) && is_key_down(KeyCode::LeftControl) {
            toggle_fullscreen(&mut is_full_screen);
        }

        next_frame().await;
    }
}

fn handle_cheat_code(scene: &mut Box<dyn Scene>) {
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

fn toggle_fullscreen(is_full_screen: &mut bool) {
    *is_full_screen = !*is_full_screen;

    set_fullscreen(*is_full_screen);
    if !*is_full_screen {
        request_new_screen_size(1000.0, 650.0);
    }
}

fn handle_menu(
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
