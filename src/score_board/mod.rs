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
pub struct ScoreBoard {
    pub score: i32,
    lives: i32,
    level: i32,
}

impl ScoreBoard {
    pub fn new() -> ScoreBoard {
        ScoreBoard {
            score: 0,
            lives: 3,
            level: 1,
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
            5.0,
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
                120.0 + (i as f32 * 20.0),
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
            220.0,
            5.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(resources.level_texture.width(), resources.level_texture.height())), 
                ..Default::default()
            },
        );

        draw_texture_ex(
            &resources.numbers[self.level as usize],
            320.0,
            7.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(resources.numbers[self.level as usize].width(), resources.numbers[self.level as usize].height())), 
                ..Default::default()
            },
        );

        draw_texture_ex(
            &resources.daves_texture,
            400.0,
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
                510.0 + (i as f32 * 30.0),
                2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(resources.dave_face.width(), resources.dave_face.height())), 
                    ..Default::default()
                },
            );
        }
    }
}
