use macroquad::{
    color::WHITE, 
    math::vec2, 
    prelude::collections::storage, 
    texture::{
        draw_texture_ex, 
        DrawTextureParams
    }
};

use crate::{monster::Monster, resources::Resources, Scene, SceneChange};


#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GameObject {
    pub world_x: f32,
    pub world_y: f32,
    pub width: f32,
    pub height: f32,
    pub name: String,
    pub collected: Option<bool>,
    pub progress: f32
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
    pub jetpack_captured: bool
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
            jetpack_captured: false
        }
    }

    fn number_to_vec(n: u32) -> Vec<u32> {
        n.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect()
    }
}

impl Scene for ScoreBoard {
    fn update(&mut self) -> Option<SceneChange> {
        None
    }

    fn draw(&self) {
        let resources = storage::get::<Resources>();

        draw_texture_ex(
            resources.get_texture("score"),
            self.position.0,
            5.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    resources.get_texture("score").width() , 
                    resources.get_texture("score").height() 
                )), 
                ..Default::default()
            },
        );

        let score = Self::number_to_vec(self.score);
        for (i, n) in score.iter().enumerate() {
            draw_texture_ex(
                &resources.numbers[*n as usize],
                self.position.0 + 120.0 + (i as f32 * 20.0),
                7.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(resources.numbers[*n as usize].width() , resources.numbers[*n as usize].height() )), 
                    ..Default::default()
                },
            );
        }

        draw_texture_ex(
            resources.get_texture("level"),
            self.position.0 + 240.0,
            5.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    resources.get_texture("level").width(), 
                    resources.get_texture("level").height()
                )), 
                ..Default::default()
            },
        );

        let levels = Self::number_to_vec(if self.level == 0 {10} else {self.level});
        for (i, n) in levels.iter().enumerate() {
            draw_texture_ex(
                &resources.numbers[*n as usize],
                self.position.0 + 340.0 + (i as f32 * 20.0),
                7.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(resources.numbers[*n as usize].width() , resources.numbers[*n as usize].height() )), 
                    ..Default::default()
                },
            );
        }

        draw_texture_ex(
            resources.get_texture("daves"),
            self.position.0 + 400.0,
            5.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    resources.get_texture("daves").width(), 
                    resources.get_texture("daves").height()
                )), 
                ..Default::default()
            },
        );

        for i in 0..self.lives {
            draw_texture_ex(
                resources.get_texture("DaveFace"),
                self.position.0 + 510.0 + (i as f32 * 30.0),
                2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(
                        resources.get_texture("DaveFace").width(), 
                        resources.get_texture("DaveFace").height()
                    )), 
                    ..Default::default()
                },
            );
        }

        draw_texture_ex(
            resources.get_texture("thin"),
            self.position.0,
            30.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    resources.get_texture("thin").width() , 
                    resources.get_texture("thin").height() * 0.5)
                ), 
                ..Default::default()
            },
        );
    }
}
