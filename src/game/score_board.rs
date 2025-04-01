use macroquad::{
    color::WHITE,
    math::vec2,
    prelude::collections::storage,
    texture::{
        DrawTextureParams,
        draw_texture_ex,
    },
};

use crate::{
    Scene,
    SceneChange,
    game::monster::Monster,
    resources::Resources,
};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GameObject {
    pub world_x: f32,
    pub world_y: f32,
    pub width: f32,
    pub height: f32,
    pub name: String,
    pub collected: Option<bool>,
}

#[derive(Clone)]
pub struct ScoreBoard {
    pub score: u32,
    pub lives: i32,
    pub level: u32,
    pub position: (f32, f32),
    pub collectibles: Vec<GameObject>,
    pub game_won: bool,
    pub gun_captured: bool,
    pub monsters: Vec<Monster>,
    pub jetpack_captured: bool,
    pub jetpack_progress: f32,
}

impl ScoreBoard {
    pub const fn new() -> Self {
        Self {
            score: 0,
            lives: 3,
            level: 1,
            position: (5.0, 5.0),
            collectibles: Vec::new(),
            game_won: false,
            gun_captured: false,
            monsters: vec![],
            jetpack_captured: false,
            jetpack_progress: 0.0,
        }
    }

    fn number_to_vec(n: u32) -> Vec<u32> {
        n.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect()
    }

    fn draw_texture(resources: &Resources, texture_key: &str, x: f32, y: f32) {
        draw_texture_ex(
            resources.get_texture(texture_key),
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    resources.get_texture(texture_key).width(),
                    resources.get_texture(texture_key).height(),
                )),
                ..Default::default()
            },
        );
    }
}

impl Scene for ScoreBoard {
    fn update(&mut self) -> Option<SceneChange> {
        None
    }

    fn draw(&mut self) {
        let resources = storage::get::<Resources>();

        Self::draw_texture(&resources, "score", self.position.0, 5.0);

        let score = Self::number_to_vec(self.score);
        for (i, n) in score.iter().enumerate() {
            draw_texture_ex(
                &resources.numbers[*n as usize],
                self.position.0 + 120.0 + (i as f32 * 20.0),
                7.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(
                        resources.numbers[*n as usize].width(),
                        resources.numbers[*n as usize].height(),
                    )),
                    ..Default::default()
                },
            );
        }

        Self::draw_texture(&resources, "level", self.position.0 + 240.0, 5.0);

        let levels = Self::number_to_vec(if self.level == 0 { 10 } else { self.level });
        for (i, n) in levels.iter().enumerate() {
            draw_texture_ex(
                &resources.numbers[*n as usize],
                self.position.0 + 340.0 + (i as f32 * 20.0),
                7.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(
                        resources.numbers[*n as usize].width(),
                        resources.numbers[*n as usize].height(),
                    )),
                    ..Default::default()
                },
            );
        }

        Self::draw_texture(&resources, "daves", self.position.0 + 400.0, 5.0);

        for i in 0..self.lives {
            Self::draw_texture(
                &resources,
                "DaveFace",
                self.position.0 + 510.0 + (i as f32 * 30.0),
                2.0,
            );
        }

        Self::draw_texture(&resources, "thin", self.position.0, 30.0);
    }
}
