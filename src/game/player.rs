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
    GameWorld,
    bullet::{
        Bullet,
        BulletDirection,
    },
    camera::GameCamera,
    collision::Collidable,
};
use crate::Resources;

macro_rules! calculate_jetpack_progress {
    ($current_time:expr, $max_time:expr) => {
        (1.0 - ($current_time / $max_time)).max(0.0)
    };
}

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
        jetpack_timer: f32,
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
            jetpack_timer,
            jetpack_timer_active: false,
            jetpack_progress: calculate_jetpack_progress!(jetpack_timer, JETPACK_TIMER),
            pos: vec2(0.0, 0.0),
            current_state: "dave_idle",
        }
    }

    pub fn should_retain_bullet(
        game_world: &GameWorld,
        camera: &GameCamera,
        bullet: &Bullet,
    ) -> bool {
        let screen_left = camera.camera.target.x - screen_width() / 2.0;
        let screen_right = camera.camera.target.x + screen_width() / 2.0;

        if game_world
            .world
            .collide_solids(Vec2::new(bullet.x, bullet.y), 20, 10)
            == Tile::Solid
        {
            return false;
        }

        bullet.x < screen_right && bullet.x > screen_left && !bullet.collided
    }

    pub fn overlaps(pos: Vec2, game_object: &Rect) -> bool {
        let player_rect = Rect::new(pos.x, pos.y, 32.0, 32.0);
        player_rect.overlaps(game_object)
    }

    pub fn update(&mut self, world: &mut World) {
        let delta = get_frame_time();
        self.pos = world.actor_pos(self.collider);

        if self.is_dead {
            self.update_dead_state();
            return;
        }

        let on_ground = world.collide_check(self.collider, self.pos + vec2(0., 1.));

        self.update_jetpack_state(delta);
        self.update_climbing_state(world);
        self.update_movement_state(world, on_ground, delta);
        self.update_shooting();
        self.update_bullets(delta);
        self.update_animation(on_ground);

        // Update player position
        world.move_h(self.collider, self.speed.x * delta);
        world.move_v(self.collider, self.speed.y * delta);
    }

    fn update_dead_state(&mut self) {
        let resources = storage::get::<Resources>();
        stop_sound(resources.get_sound("jetPackActivated"));
        stop_sound(resources.get_sound("climb"));
    }

    fn update_jetpack_state(&mut self, delta: f32) {
        let resources = storage::get::<Resources>();

        if self.jetpack_active {
            self.jetpack_timer += delta;
            self.jetpack_progress = calculate_jetpack_progress!(self.jetpack_timer, JETPACK_TIMER);

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
    }

    fn update_climbing_state(&mut self, world: &World) {
        let resources = storage::get::<Resources>();
        let tiled_map = storage::get::<Map>();

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
    }

    fn update_movement_state(&mut self, world: &World, on_ground: bool, delta: f32) {
        let resources = storage::get::<Resources>();

        // Vertical movement logic
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

        // Horizontal movement logic
        if !self.attach && (self.simulate_right || is_key_down(KeyCode::Right)) {
            self.speed.x = 100.0;
        } else if !self.attach && (self.simulate_left || is_key_down(KeyCode::Left)) {
            self.speed.x = -100.0;
        } else {
            self.speed.x = 0.;
        }
    }

    fn update_shooting(&mut self) {
        let resources = storage::get::<Resources>();

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
    }

    fn update_bullets(&mut self, delta: f32) {
        for bullet in &mut self.bullets {
            if bullet.direction == BulletDirection::Left {
                bullet.x -= bullet.speed * delta;
            } else {
                bullet.x += bullet.speed * delta;
            }
        }
    }

    fn update_animation(&mut self, on_ground: bool) {
        // Update animation state based on player state
        if self.jetpack_active {
            self.animated_player.set_animation(3);
            self.current_state = "player_jetpack";
        } else if self.climbing || self.climbing_active {
            self.current_state = "climb-sheet";
            self.animated_player.set_animation(4);
        } else if self.speed.x != 0.0 {
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

        // Always update the animation - this was being skipped for climbing and jetpack
        self.animated_player.update();
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
