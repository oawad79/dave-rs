mod player;
mod resources;
mod game;
mod main_menu;

use game::Game;
use main_menu::MainMenu;
use resources::Resources;
use macroquad::prelude::*;

pub enum SceneChange {
    MainMenu,
    Game,
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
    
    let main_camera = Camera2D::from_display_rect(Rect::new(0.0, 320.0, 608.0, -320.0));
    
    let mut scene: Box<dyn Scene> = Box::new(MainMenu::new());
    
    loop {
        clear_background(BLACK);
    
        set_camera(&main_camera);

        let change = scene.update();
        if let Some(change) = change {
            scene = match change {
                SceneChange::MainMenu => Box::new(MainMenu::new()),
                SceneChange::Game => Box::new(Game::new()),
            };
        }

        scene.draw();

        next_frame().await
    }
}

