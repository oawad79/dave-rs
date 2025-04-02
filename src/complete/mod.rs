use macroquad::{
    color::WHITE,
    input::{
        KeyCode,
        is_key_down,
    },
    math::Rect,
    prelude::collections::storage,
    text::{
        TextParams,
        draw_text_ex,
    },
};
use macroquad_tiled::{
    Map,
    load_map,
};

use crate::{
    Scene,
    SceneChange,
    resources::Resources,
};

pub struct Complete {
    tiled_map: Map,
}

impl Complete {
    pub fn new() -> Self {
        let resources = storage::get::<Resources>();

        let tiled_map = load_map(
            &resources.done_map_json,
            &[(
                "images/collectibles.png",
                resources.get_texture("collectibles").clone(),
            )],
            &[],
        )
        .unwrap();

        Self { tiled_map }
    }

    fn draw_texture(text: &str, x: f32, y: f32, resources: &Resources) {
        draw_text_ex(
            text,
            x,
            y,
            TextParams {
                font: Some(&resources.font),
                font_size: 25,
                color: WHITE,
                ..Default::default()
            },
        );
    }
}

impl Scene for Complete {
    fn update(&mut self) -> Option<SceneChange> {
        if is_key_down(KeyCode::Enter) {
            return Some(SceneChange::EntryScreen);
        }

        None
    }

    fn draw(&mut self) {
        let resources = storage::get::<Resources>();

        self.tiled_map
            .draw_tiles("platform", Rect::new(0.0, 0.0, 608.0, 384.0), None);

        Self::draw_texture("CONGRATULATIONS !", 230.0, 70.0, &resources);
        Self::draw_texture(
            "YOU MADE IT THROUGH ALL THE PERILOUS AREAS IN ",
            50.0,
            110.0,
            &resources,
        );

        Self::draw_texture("THE DAVE THE DUCK ADVENTURE !", 50.0, 150.0, &resources);

        Self::draw_texture("CLYDE'S HIDEOUT!", 50.0, 150.0, &resources);

        Self::draw_texture(
            "VERY GOOD WORK! DID YOU FIND THE 4 WARP ZONES ? ",
            50.0,
            190.0,
            &resources,
        );

        Self::draw_texture(
            "THEY ARE LOCATED ON LEVELS 5, 8, 9 AND 10. JUST ",
            50.0,
            210.0,
            &resources,
        );

        Self::draw_texture(
            "JUMP OFF THE TOP OF THE SCREEN AT THE EXTREME ",
            50.0,
            230.0,
            &resources,
        );

        Self::draw_texture(
            "LEFT OR RIGHT EDGE OF THE WORLD AND VIOLA ! YOU'RE ",
            50.0,
            250.0,
            &resources,
        );

        Self::draw_texture("THERE ! ", 50.0, 270.0, &resources);

        Self::draw_texture("PRESS ENTER ", 240.0, 320.0, &resources);
    }
}
