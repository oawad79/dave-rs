use macroquad::{
    audio::{
        PlaySoundParams,
        play_sound,
        play_sound_once,
        stop_sound,
    },
    math::{
        Rect,
        Vec2,
        vec2,
    },
    prelude::{
        animation::{
            AnimatedSprite,
            Animation,
        },
        collections::storage,
        *,
    },
};
use macroquad_platformer::{
    Actor,
    Tile,
    World,
};
use macroquad_tiled::Map;

use super::{
    bullet::{
        Bullet,
        BulletDirection,
    },
    collidable::Collidable,
};
use crate::Resources;

const GRAVITY: f32 = 400.0;
const JUMP_VELOCITY: f32 = -250.0;
const JETPACK_VELOCITY: f32 = 100.0;
const JETPACK_TIMER: f32 = 10.0;
pub struct Player {
    pub collider: Actor,
    pub speed: Vec2,
    pub facing_left: bool,
    animated_player: AnimatedSprite,
    pub simulate_left: bool,
    pub simulate_right: bool,
    pub is_dead: bool,
    pub has_gun: bool,
    pub bullets: Vec<Bullet>,
    pub has_jetpack: bool,
    pub jetpack_active: bool,
    pub climbing: bool,
    pub climbing_active: bool,
    attach: bool,
    pub jetpack_timer: f32,
    jetpack_timer_active: bool,
    pub jetpack_progress: f32,
    pos: Vec2,
    current_state: &'static str,
}

impl Collidable for Player {
    fn get_collision_rect(&self) -> Rect {
        Rect::new(self.pos.x, self.pos.y, 32.0, 32.0)
    }

    fn get_position(&self) -> Vec2 {
        self.pos
    }

    fn on_hit(&mut self) {
        self.is_dead = true;
    }

    fn is_alive(&self) -> bool {
        !self.is_dead
    }
}

impl Player {
    pub fn new(
        collider: Actor,
        has_gun: bool,
        has_jetpack: bool,
        attach: bool,
        jetpack_progress: f32,
    ) -> Self {
        Self {
            collider,
            speed: vec2(0., 0.),
            facing_left: false,
            animated_player: animated_player(),
            simulate_left: false,
            simulate_right: false,
            is_dead: false,
            has_gun,
            bullets: vec![],
            has_jetpack,
            jetpack_active: false,
            climbing: false,
            climbing_active: false,
            attach,
            jetpack_timer: 0.0,
            jetpack_timer_active: false,
            jetpack_progress,
            pos: vec2(0.0, 0.0),
            current_state: "dave_idle",
        }
    }

    pub fn overlaps(pos: Vec2, game_object: &Rect) -> bool {
        let player_rect = Rect::new(pos.x, pos.y, 32.0, 32.0);
        player_rect.overlaps(game_object)
    }

    pub fn update(&mut self, world: &mut World) {
        let resources = storage::get::<Resources>();
        let tiled_map = storage::get::<Map>();

        let delta = get_frame_time();

        self.pos = world.actor_pos(self.collider);

        let on_ground = world.collide_check(self.collider, self.pos + vec2(0., 1.));

        if self.is_dead {
            stop_sound(resources.get_sound("jetPackActivated"));
            stop_sound(resources.get_sound("climb"));
        }

        // Update player animation state
        if !self.climbing_active {
            if self.speed.x != 0.0 {
                if !on_ground {
                    self.animated_player.set_animation(2); // jump
                    self.current_state = "dave_jump";
                } else {
                    self.animated_player.set_animation(0); // walk
                    self.current_state = "dave_walk";
                }

                self.facing_left = self.speed.x < 0.0;
            } else {
                self.current_state = "dave_idle";
                self.animated_player.set_animation(1); // idle
            }
        } else {
            self.current_state = "climb-sheet";
            self.animated_player.set_animation(4);
        }

        if self.jetpack_active {
            self.jetpack_timer += get_frame_time();
            self.jetpack_progress = (1.0 - (self.jetpack_timer / JETPACK_TIMER)).max(0.0);

            //println!("Jetpack progress: {}", self.jetpack_progress);

            if self.jetpack_timer >= JETPACK_TIMER {
                self.jetpack_active = false;
                self.jetpack_timer_active = false;
                self.has_jetpack = false;

                stop_sound(resources.get_sound("jetPackActivated"));
            }
        }

        if is_key_pressed(KeyCode::LeftAlt) && self.has_jetpack {
            self.jetpack_active = !self.jetpack_active;

            if self.jetpack_active {
                self.jetpack_timer_active = true;

                play_sound(
                    resources.get_sound("jetPackActivated"),
                    PlaySoundParams {
                        looped: true,
                        volume: 1.0,
                    },
                );
            } else {
                self.jetpack_timer_active = false;
                stop_sound(resources.get_sound("jetPackActivated"));
            }
        }

        if tiled_map.contains_layer("tree_collider") {
            if world.collide_tag(2, self.pos, 10, 32) == Tile::JumpThrough {
                if is_key_down(KeyCode::Up) || is_key_down(KeyCode::Down) {
                    if !self.climbing {
                        play_sound(
                            resources.get_sound("climb"),
                            PlaySoundParams {
                                looped: true,
                                volume: 0.2,
                            },
                        );
                    }

                    self.climbing = true;
                    self.climbing_active = true;
                } else {
                    self.climbing = false;
                    stop_sound(resources.get_sound("climb"));
                }
            } else {
                self.climbing_active = false;
                self.climbing = false;
                stop_sound(resources.get_sound("climb"));
            }
        }

        if self.jetpack_active {
            self.animated_player.set_animation(3);
            self.current_state = "player_jetpack";
        }

        if self.climbing {
            self.current_state = "climb-sheet";
            self.animated_player.set_animation(4);
        }

        self.animated_player.update();

        // player movement control
        if !self.attach && !on_ground && !self.jetpack_active && !self.climbing_active {
            self.speed.y += GRAVITY * delta;
        } else if self.jetpack_active || self.climbing_active {
            if is_key_down(KeyCode::Up) {
                self.speed.y = -JETPACK_VELOCITY;
            } else if is_key_down(KeyCode::Down) {
                self.speed.y = JETPACK_VELOCITY;
            } else {
                self.speed.y = 0.0;
            }
        }

        if is_key_down(KeyCode::Down) {
            self.attach = false;
        }

        if !self.climbing && is_key_pressed(KeyCode::Up) && on_ground {
            play_sound_once(resources.get_sound("jump"));
            self.speed.y = JUMP_VELOCITY;
        }

        if !self.attach && (self.simulate_right || is_key_down(KeyCode::Right)) {
            self.speed.x = 100.0;
        } else if !self.attach && (self.simulate_left || is_key_down(KeyCode::Left)) {
            self.speed.x = -100.0;
        } else {
            self.speed.x = 0.;
        }

        if is_key_pressed(KeyCode::LeftControl) && self.has_gun {
            self.bullets.push(Bullet {
                x: self.pos.x + 10.0,
                y: self.pos.y,
                speed: 250.0,
                collided: false,
                direction: if self.facing_left {
                    BulletDirection::Left
                } else {
                    BulletDirection::Right
                },
            });
            play_sound_once(resources.get_sound("shoot"));
        }

        // Update bullet positions
        for bullet in &mut self.bullets {
            if bullet.direction == BulletDirection::Left {
                bullet.x -= bullet.speed * delta;
            } else {
                bullet.x += bullet.speed * delta;
            }
        }

        // Update player position
        world.move_h(self.collider, self.speed.x * delta);
        world.move_v(self.collider, self.speed.y * delta);
    }

    pub fn draw(&self) {
        if self.is_dead {
            return;
        }

        let resources = storage::get::<Resources>();
        let tiled_map = storage::get::<Map>();

        // Draw player sprite
        let flip: f32 = if self.facing_left { -32.0 } else { 32.0 };

        tiled_map.spr_ex(
            self.current_state,
            self.animated_player.frame().source_rect,
            Rect::new(
                self.pos.x + if flip < 0.0 { 32.0 } else { 0.0 },
                self.pos.y,
                flip,
                32.0,
            ),
        );

        // Draw bullets
        for bullet in &self.bullets {
            draw_texture_ex(
                resources.get_texture("bullet"),
                bullet.x,
                bullet.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(
                        resources.get_texture("bullet").width(),
                        resources.get_texture("bullet").height(),
                    )),
                    rotation: if bullet.direction == BulletDirection::Left {
                        std::f32::consts::PI
                    } else {
                        0.0
                    },
                    ..Default::default()
                },
            );
        }
    }
}

pub enum AnimationState {
    Walk,
    Idle,
    Jump,
    Fly,
    Climb,
}

impl AnimationState {
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Walk => "walk",
            Self::Idle => "idle",
            Self::Jump => "jump",
            Self::Fly => "fly",
            Self::Climb => "climb",
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
            Animation {
                name: AnimationState::Fly.as_str().to_string(),
                row: 0,
                frames: 1,
                fps: 1,
            },
            Animation {
                name: AnimationState::Climb.as_str().to_string(),
                row: 0,
                frames: 3,
                fps: 3,
            },
        ],
        true,
    )
}
