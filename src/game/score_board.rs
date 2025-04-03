use macroquad::{
    color::WHITE,
    math::vec2,
    prelude::collections::storage,
    texture::{
        DrawTextureParams,
        draw_texture_ex,
    },
    window::{
        screen_height,
        screen_width,
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
    pub collectibles: Vec<GameObject>,
    pub game_won: bool,
    pub gun_captured: bool,
    pub monsters: Vec<Monster>,
    pub jetpack_captured: bool,
    pub jetpack_timer: f32,
}

impl ScoreBoard {
    pub const fn new() -> Self {
        Self {
            score: 0,
            lives: 3,
            level: 1,
            collectibles: Vec::new(),
            game_won: false,
            gun_captured: false,
            monsters: vec![],
            jetpack_captured: false,
            jetpack_timer: 0.0,
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
                dest_size: Some(vec2(screen_width() * 0.12, screen_height() * 0.06)),
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

        Self::draw_texture(&resources, "score", 5.0, 5.0);

        let score = Self::number_to_vec(self.score);
        for (i, n) in score.iter().enumerate() {
            draw_texture_ex(
                &resources.numbers[*n as usize],
                (i as f32).mul_add(screen_width() * 0.03, screen_width() * 0.14),
                7.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width() * 0.02, screen_height() * 0.06)),
                    ..Default::default()
                },
            );
        }

        Self::draw_texture(&resources, "level", screen_width() * 0.40, 5.0);

        let levels = Self::number_to_vec(if self.level == 0 { 10 } else { self.level });
        for (i, n) in levels.iter().enumerate() {
            draw_texture_ex(
                &resources.numbers[*n as usize],
                (i as f32).mul_add(screen_width() * 0.03, screen_width() * 0.53),
                7.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width() * 0.02, screen_height() * 0.055)),
                    ..Default::default()
                },
            );
        }

        Self::draw_texture(&resources, "daves", screen_width() * 0.7, 5.0);

        for i in 0..self.lives {
            draw_texture_ex(
                resources.get_texture("DaveFace"),
                (i as f32).mul_add(screen_width() * 0.05, screen_width() * 0.83),
                2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width() * 0.07, screen_height() * 0.07)),
                    ..Default::default()
                },
            );
        }

        draw_texture_ex(
            resources.get_texture("thin"),
            1.0,
            screen_height() * 0.077,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), resources.get_texture("thin").height())),
                ..Default::default()
            },
        );
    }
}
