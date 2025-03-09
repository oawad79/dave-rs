use macroquad::{
    audio::play_sound_once, 
    math::{vec2, Rect, Vec2}, 
    prelude::{
        animation::{AnimatedSprite, Animation}, 
        collections::storage
    }
};
use macroquad_platformer::{Actor, World};
use macroquad::prelude::*;
use macroquad_tiled::Map;

use crate::{bullet::Bullet, Resources};

const GRAVITY: f32 = 500.0;
const JUMP_VELOCITY: f32 = -280.0;

pub struct Player {
    pub collider: Actor,
    pub speed: Vec2,
    pub facing_left: bool,
    animated_player: AnimatedSprite,
    simulate_jump: bool,
    pub simulate_left: bool,
    pub simulate_right: bool,
    pub is_dead: bool,
    pub has_gun: bool,
    pub bullets: Vec<Bullet>
}

impl Player {
    pub fn new(collider: Actor) -> Self {
        Player {
            collider,
            speed: vec2(0., 0.),
            facing_left: false,
            animated_player: animated_player(),
            simulate_jump: false,
            simulate_left: false,
            simulate_right: false,
            is_dead: false,
            has_gun: false,
            bullets: vec![]
        }
    }

    pub fn overlaps(&self, pos: Vec2, game_object: &Rect) -> bool {
        let player_rect = Rect::new(
            pos.x,
            pos.y,
            32.0,
            32.0,
        );
        player_rect.overlaps(game_object)
    }

    pub fn update(&mut self, world: &mut World) {
        let resources = storage::get::<Resources>();
        let tiled_map = storage::get::<Map>();

        let delta = get_frame_time();

        let pos = world.actor_pos(self.collider);
        
        let on_ground = world.collide_check(self.collider, pos + vec2(0., 1.));

        let state: &str;
        let flip: f32;

        if self.speed.x != 0.0 {
            state = if !on_ground {
                self.animated_player.set_animation(2); // jump
                "dave_jump"
            } else {
                self.animated_player.set_animation(0); // walk
                "dave_walk"
            };

            if self.speed.x < 0.0 {
                self.facing_left = true;
                flip = -32.0;
            } else {
                self.facing_left = false;
                flip = 32.0;
            }
        } else {
            state = "dave_idle";
            self.animated_player.set_animation(1); // idle
            flip = if self.facing_left { -32.0 } else { 32.0 };
        }

        if !self.is_dead {
            tiled_map.spr_ex(
                state,
                self.animated_player.frame().source_rect,
                Rect::new(
                    pos.x + if flip < 0.0 { 32.0 } else { 0.0 },
                    pos.y,
                    flip,
                    32.0,
                ),
            );
        }

        self.animated_player.update();

        // player movement control
        if !on_ground {
            self.speed.y += GRAVITY * delta;
        }

        if self.simulate_right || is_key_down(KeyCode::Right) {
            self.speed.x = 100.0;
        } else if self.simulate_left || is_key_down(KeyCode::Left) {
            self.speed.x = -100.0;
        } else {
            self.speed.x = 0.;
        }

        if is_key_pressed(KeyCode::Up) && on_ground {
            play_sound_once(resources.get_sound("jump"));
            self.speed.y = JUMP_VELOCITY;
        }

        if is_key_pressed(KeyCode::LeftControl) && self.has_gun  {
            self.bullets.push(Bullet {
                x: pos.x + 10.0,
                y: pos.y,
                speed: 250.0,
                collided: false 
            });
            play_sound_once(resources.get_sound("shoot"));
        }

        for bullet in &mut self.bullets {
            bullet.x += bullet.speed * delta;
        }

        for bullet in &self.bullets {
            draw_texture_ex(
                resources.get_texture("bullet"),
                bullet.x,
                bullet.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(
                        resources.get_texture("bullet").width(), 
                        resources.get_texture("bullet").height()
                    )),
                    ..Default::default()
                },
            );
        }

        world.move_h(self.collider, self.speed.x * delta);
        world.move_v(self.collider, self.speed.y * delta);

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

