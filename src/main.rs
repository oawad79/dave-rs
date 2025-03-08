mod player;
mod resources;
mod game;
mod entry_screen;
mod separator;
mod score_board;
mod monster;

use game::Game;
use entry_screen::EntryScreen;
use separator::Separator;
use resources::Resources;
use macroquad::prelude::{collections::storage, *};

pub enum SceneChange {
    EntryScreen,
    Game{level: i32, retry: bool, cheat: bool},
    Separator
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
    
    macroquad::logging::info!("started program..!!!");

    set_pc_assets_folder("assets");

    let _ = Resources::load().await;
    
    let main_camera = Camera2D::from_display_rect(Rect::new(0.0, 384.0, 608.0, -384.0));
    
    let mut scene: Box<dyn Scene> = Box::new(EntryScreen::new());
    
    let resources = storage::get::<Resources>();

    let mut show_quit = false;

    loop {
        clear_background(BLACK);
        
        set_camera(&main_camera);

        let change = scene.update();
        if let Some(change) = change {
            scene = match change {
                SceneChange::EntryScreen => Box::new(EntryScreen::new()),
                SceneChange::Game{level, retry, cheat} => Box::new(Game::new(level, retry, cheat)),
                SceneChange::Separator => Box::new(Separator::new())
            };
        }

        scene.draw();

        if handle_quit_menu(&resources, &mut show_quit) {
            break;
        }

        if is_key_down(KeyCode::LeftControl) {
            // Check if any number key (0-9) is pressed
            for (i, key) in [
                KeyCode::Key0, KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4,
                KeyCode::Key5, KeyCode::Key6, KeyCode::Key7, KeyCode::Key8, KeyCode::Key9,
            ]
            .iter()
            .enumerate()
            {
                if is_key_down(*key) {
                    scene = Box::new(Game::new(i as i32, false, true));
                }
            }
        }

        next_frame().await
    }

   

}

fn handle_quit_menu(resources: &Resources, show_quit: &mut bool) -> bool {
    if is_key_down(KeyCode::Escape) || *show_quit {
        *show_quit = true;
        draw_texture_ex(
            &resources.quit_texture,
            220.0,
            150.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(resources.quit_texture.width() * 0.7, resources.quit_texture.height() * 0.7)), 
                ..Default::default()
            },
        );
    }

    if *show_quit && is_key_down(KeyCode::Y) {
        return true;
    } else if *show_quit && is_key_down(KeyCode::N) {
        *show_quit = false;
    }

    false
}
