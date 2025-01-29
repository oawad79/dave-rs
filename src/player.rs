use macroquad::{math::{vec2, Vec2}, prelude::animation::{AnimatedSprite, Animation}};
use macroquad_platformer::Actor;

pub struct Player {
    pub collider: Actor,
    pub speed: Vec2,
    pub facing_left: bool,
    
}

impl Player {
    pub fn new(collider: Actor) -> Self {
        Player {
            collider,
            speed: vec2(0., 0.),
            facing_left: false,
            
        }
    }
}

pub enum AnimationState {
    Walk,
    Idle,
    Jump,
}

impl AnimationState {
    pub fn as_str(&self) -> &str {
        match self {
            AnimationState::Walk => "walk",
            AnimationState::Idle => "idle",
            AnimationState::Jump => "jump",
        }
    }
}

pub fn animated_player() -> AnimatedSprite {
    AnimatedSprite::new(
        32,
        32,
        &[
            Animation {
                name: AnimationState::Walk.as_str().to_string(),
                row: 0,
                frames: 2,
                fps: 4,
            },
            Animation {
                name: AnimationState::Idle.as_str().to_string(),
                row: 0,
                frames: 1,
                fps: 1,
            },
            Animation {
                name: AnimationState::Jump.as_str().to_string(),
                row: 0,
                frames: 1,
                fps: 1,
            },
        ],
        true,
    )
}

