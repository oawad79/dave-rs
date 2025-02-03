mod player;
mod resources;
mod game;


use game::Game;
use resources::Resources;
use macroquad::prelude::*;

pub trait Scene {
    fn update(&mut self);
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
    
    let camera = Camera2D::from_display_rect(Rect::new(0.0, 320.0, 608.0, -320.0));
    
    let mut scene: Box<dyn Scene> = Box::new(Game::new());
    loop {
        clear_background(BLACK);

        set_camera(&camera);

        scene.update();
        scene.draw();
        
        next_frame().await
    }
}
