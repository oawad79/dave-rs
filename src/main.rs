mod player;
mod resources;
mod game;
mod main_menu;
mod separator;
mod score_board;

use game::Game;
use main_menu::MainMenu;
use separator::Separator;
use resources::Resources;
use macroquad::prelude::{collections::storage, *};

pub enum SceneChange {
    MainMenu,
    Game{level: i32, retry: bool},
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
    
    let main_camera = Camera2D::from_display_rect(Rect::new(0.0, 352.0, 608.0, -352.0));
    
    let mut scene: Box<dyn Scene> = Box::new(MainMenu::new());
    
    let resources = storage::get::<Resources>();

    let mut show_quit = false;

    loop {
        clear_background(BLACK);
        
        set_camera(&main_camera);

        let change = scene.update();
        if let Some(change) = change {
            scene = match change {
                SceneChange::MainMenu => Box::new(MainMenu::new()),
                SceneChange::Game{level, retry} => Box::new(Game::new(level, retry)),
                SceneChange::Separator => Box::new(Separator::new())
            };
        }

        scene.draw();

        if handle_quit_menu(&resources, &mut show_quit) {
            break;
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
