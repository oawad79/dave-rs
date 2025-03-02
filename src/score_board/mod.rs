use macroquad::{
    color::WHITE, 
    math::vec2, 
    prelude::collections::storage, 
    texture::{
        draw_texture_ex, 
        DrawTextureParams
    }
};

use crate::{resources::Resources, Scene, SceneChange};

#[derive(Clone)]
pub struct GameObject {
    pub world_x: f32,
    pub world_y: f32,
    pub name: String,
    pub collected: Option<bool>,
}

#[derive(Clone)]
pub struct ScoreBoard {
    pub score: i32,
    pub lives: i32,
    pub level: i32,
    pub position: (f32, f32),
    pub collectibles: Vec<GameObject>,
}

impl ScoreBoard {
    pub fn new() -> ScoreBoard {
        ScoreBoard {
            score: 0,
            lives: 3,
            level: 1,
            position: (5.0, 5.0),
            collectibles: Vec::new(),
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
            &resources.score_texture,
            self.position.0,
            5.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(resources.score_texture.width() , resources.score_texture.height() )), 
                ..Default::default()
            },
        );

        let score = ScoreBoard::number_to_vec(self.score as u32);
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
            &resources.level_texture,
            self.position.0 + 220.0,
            5.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(resources.level_texture.width(), resources.level_texture.height())), 
                ..Default::default()
            },
        );

        draw_texture_ex(
            &resources.numbers[self.level as usize],
            self.position.0 + 320.0,
            7.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(resources.numbers[self.level as usize].width(), resources.numbers[self.level as usize].height())), 
                ..Default::default()
            },
        );

        draw_texture_ex(
            &resources.daves_texture,
            self.position.0 + 400.0,
            5.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(resources.daves_texture.width(), resources.daves_texture.height())), 
                ..Default::default()
            },
        );

        for i in 0..self.lives {
            draw_texture_ex(
                &resources.dave_face,
                self.position.0 + 510.0 + (i as f32 * 30.0),
                2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(resources.dave_face.width(), resources.dave_face.height())), 
                    ..Default::default()
                },
            );
        }

        draw_texture_ex(
            &resources.thin,
            self.position.0,
            30.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(resources.thin.width() , resources.thin.height() * 0.5)), 
                ..Default::default()
            },
        );
    }
}
