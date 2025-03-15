use macroquad::{color::WHITE, input::{is_key_down, KeyCode}, math::Rect, prelude::collections::storage, text::{draw_text_ex, TextParams}};
use macroquad_tiled::{load_map, Map};

use crate::{resources::{self, Resources}, Scene, SceneChange};

pub struct Complete {
    tiled_map: Map,
}

impl Complete {
    pub fn new() -> Self {
        let resources = storage::get::<Resources>();

        let tiled_map = load_map(
            &resources.done_map_json,
            &[
                ("images/collectibles.png", resources.get_texture("collectibles").clone())
            ],
            &[],
        )
        .unwrap();

        Self {
            tiled_map
        }
    }
}

impl Scene for Complete {
    fn update(&mut self) -> Option<SceneChange> {

        if is_key_down(KeyCode::Space) {
            return Some(SceneChange::EntryScreen);
        }

        None
    }

    fn draw(&self) {
        let resources = storage::get::<Resources>();
        
        self.tiled_map
            .draw_tiles("platform", Rect::new(0.0, 0.0, 608.0, 384.0), None);

        draw_text_ex(
            "CONGRATULATIONS !",
            230.0,
            70.0,
            TextParams {
                font: Some(&resources.font), 
                font_size: 25,     
                color: WHITE,  

                ..Default::default()
            },
        );

        draw_text_ex(
            "YOU MADE IT THROUGH ALL THE PERILOUS AREAS IN ",
            50.0,
            130.0,
            TextParams {
                font: Some(&resources.font), 
                font_size: 25,     
                color: WHITE,  

                ..Default::default()
            },
        );

        draw_text_ex(
            "CLYDE'S HIDEOUT!",
            50.0,
            150.0,
            TextParams {
                font: Some(&resources.font), 
                font_size: 25,     
                color: WHITE,  

                ..Default::default()
            },
        );
         
        draw_text_ex(
        "VERY GOOD WORK! DID YOU FIND THE 4 WARP ZONES ? ",
            50.0,
            190.0,
            TextParams {
                font: Some(&resources.font), 
                font_size: 25,     
                color: WHITE,  

                ..Default::default()
            },
        );    

        draw_text_ex(
            "THEY ARE LOCATED ON LEVELS 5, 8, 9 AND 10. JUST ",
            50.0,
            210.0,
            TextParams {
                font: Some(&resources.font), 
                font_size: 25,     
                color: WHITE,  

                ..Default::default()
            },
        );    

        draw_text_ex(
            "JUMP OFF THE TOP OF THE SCREEN AT THE EXTREME ",
            50.0,
            230.0,
            TextParams {
                font: Some(&resources.font), 
                font_size: 25,     
                color: WHITE,  

                ..Default::default()
            },
        );    

        draw_text_ex(
            "LEFT OR RIGHT EDGE OF THE WORLD AND VIOLA ! YOU'RE ",
            50.0,
            250.0,
            TextParams {
                font: Some(&resources.font), 
                font_size: 25,     
                color: WHITE,  

                ..Default::default()
            },
        );    

        draw_text_ex(
            "THERE ! ",
            50.0,
            270.0,
            TextParams {
                font: Some(&resources.font), 
                font_size: 25,     
                color: WHITE,  

                ..Default::default()
            },
        );    

        draw_text_ex(
            "PRESS SPACE ",
            240.0,
            320.0,
            TextParams {
                font: Some(&resources.font), 
                font_size: 25,     
                color: WHITE,  

                ..Default::default()
            },
        );    

    }    
}