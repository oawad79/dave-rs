use macroquad::prelude::{collections::storage, *};
use macroquad_tiled::{load_map, Map};

use crate::{resources::Resources, Scene, SceneChange};

pub struct MainMenu {
    
}

impl MainMenu {
    pub fn new() -> MainMenu {
        let resources = storage::get::<Resources>();

        let tiled_map = load_map(
            &resources.intro_map_json,
            &[
                ("fire1-sheet.png", resources.fire1.clone()),
                ("dangerousdave1-sheet.png", resources.logo1.clone()),
                ("king.png", resources.king.clone()),
                ("mytileset.png", resources.tileset.clone())
                
            ],
            &[],
        )
        .unwrap();

        storage::store(tiled_map);

        MainMenu {           
        }
    }
}

impl Scene for MainMenu {
    fn update(&mut self) -> Option<SceneChange> {

        if is_key_down(KeyCode::Space) {
            return Some(SceneChange::Game);
        }

        None
    }

    fn draw(&self) {
        let tiled_map = storage::get::<Map>();

        tiled_map
            .draw_tiles("logo", Rect::new((screen_width() / 2.0) - 350.0, 0.0, 320.0, 320.0), None);
    }
}