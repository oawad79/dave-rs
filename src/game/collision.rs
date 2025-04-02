use std::collections::HashMap;

use macroquad::{
    audio::play_sound_once,
    math::{
        Rect,
        Vec2,
        vec2,
    },
    prelude::animation::AnimatedSprite,
};
use macroquad_particles::{
    AtlasConfig,
    Emitter,
    EmitterConfig,
};
use macroquad_tiled::Object;

use super::{
    EXPLOSION_DURATION,
    bullet::Bullet,
    collectibles::CollectibleType,
    game_state::GameState,
    monster::Monster,
    player::Player,
    score_board::{
        GameObject,
        ScoreBoard,
    },
};
use crate::resources::Resources;

pub trait Collidable {
    /// Returns the collision rectangle for this entity
    fn get_collision_rect(&self) -> Rect;

    /// Returns the current position of this entity
    fn get_position(&self) -> Vec2;

    /// Handle being hit by something
    fn on_hit(&mut self);

    /// Check if this entity is alive
    fn is_alive(&self) -> bool;
}

pub struct CollisionManager {}

impl CollisionManager {
    pub fn particle_explosion() -> EmitterConfig {
        EmitterConfig {
            local_coords: false,
            one_shot: true,
            emitting: true,
            lifetime: EXPLOSION_DURATION,
            lifetime_randomness: 0.3,
            explosiveness: 0.65,
            initial_direction_spread: 2.0 * std::f32::consts::PI,
            initial_velocity: 200.0,
            initial_velocity_randomness: 0.8,
            size: 16.0,
            size_randomness: 0.3,
            atlas: Some(AtlasConfig::new(5, 1, 0..)),
            ..Default::default()
        }
    }

    pub fn check_monster_bullet_hit(
        resources: &Resources,
        pos: Vec2,
        bullet: &mut Bullet,
        player: &mut Player,
        explosions: &mut Vec<(Emitter, Vec2)>,
        game_state: &mut GameState,
    ) {
        let bullet_rect = Rect {
            x: bullet.x,
            y: bullet.y,
            w: resources.get_texture("monster_bullet").width(),
            h: resources.get_texture("monster_bullet").height(),
        };

        if Player::overlaps(pos, &bullet_rect) {
            bullet.collided = true;

            player.is_dead = true;

            game_state.player_explosion_active = true;
            game_state.player_explosion_timer = EXPLOSION_DURATION;

            if explosions.is_empty() {
                explosions.push((
                    Emitter::new(EmitterConfig {
                        amount: 40,
                        texture: Some(resources.get_texture("explosion").clone()),
                        ..Self::particle_explosion()
                    }),
                    vec2(pos.x, pos.y),
                ));
            }

            play_sound_once(resources.get_sound("explosion"));
        }
    }

    pub fn handle_collisions(
        monsters: &mut [Monster],
        player: &mut Player,
        score_board: &mut ScoreBoard,
        explosions: &mut Vec<(Emitter, Vec2)>,
        game_state: &mut GameState,
        resources: &Resources,
    ) {
        let player_pos = player.get_position();

        for monster in monsters.iter_mut() {
            if !monster.is_alive() {
                continue;
            }

            // Check player-monster collision
            if Player::overlaps(player_pos, &monster.get_collision_rect()) {
                Self::handle_player_monster_collision(
                    player,
                    monster,
                    score_board,
                    explosions,
                    game_state,
                    resources,
                    player_pos,
                );
            }

            // Check bullet-monster collisions
            for bullet in &mut player.bullets {
                let bullet_rect = Rect {
                    x: bullet.x,
                    y: bullet.y,
                    w: resources.get_texture("bullet").width(),
                    h: resources.get_texture("bullet").height(),
                };

                if bullet_rect.overlaps(&monster.get_collision_rect()) {
                    Self::handle_bullet_monster_collision(bullet, monster, explosions, resources);
                }
            }
        }
    }

    fn handle_player_monster_collision(
        player: &mut Player,
        monster: &mut Monster,
        score_board: &mut ScoreBoard,
        explosions: &mut Vec<(Emitter, Vec2)>,
        game_state: &mut GameState,
        resources: &Resources,
        player_pos: Vec2,
    ) {
        player.is_dead = true;
        monster.on_hit();
        score_board.score += monster.kill_value;

        game_state.monster_explosion_active = true;
        game_state.monster_explosion_timer = EXPLOSION_DURATION;
        game_state.player_explosion_active = true;
        game_state.player_explosion_timer = EXPLOSION_DURATION;

        if explosions.is_empty() {
            explosions.push((
                Emitter::new(EmitterConfig {
                    amount: 40,
                    texture: Some(resources.get_texture("explosion").clone()),
                    ..Self::particle_explosion()
                }),
                player_pos,
            ));
            explosions.push((
                Emitter::new(EmitterConfig {
                    amount: 40,
                    texture: Some(resources.get_texture("explosion").clone()),
                    ..Self::particle_explosion()
                }),
                monster.get_position(),
            ));
        }

        play_sound_once(resources.get_sound("explosion"));
        play_sound_once(resources.get_sound("hd-die-dave-7"));
    }

    fn handle_bullet_monster_collision(
        bullet: &mut crate::game::bullet::Bullet,
        monster: &mut Monster,
        explosions: &mut Vec<(Emitter, Vec2)>,
        resources: &Resources,
    ) {
        bullet.collided = true;
        monster.on_hit();

        if explosions.is_empty() {
            explosions.push((
                Emitter::new(EmitterConfig {
                    amount: 40,
                    texture: Some(resources.get_texture("explosion").clone()),
                    ..Self::particle_explosion()
                }),
                monster.get_position(),
            ));
        }

        play_sound_once(resources.get_sound("explosion"));
    }

    pub fn check_warp_zone_collision(warp_zone_rect: Option<&Rect>, player_pos: Vec2) -> bool {
        if let Some(wz) = warp_zone_rect {
            if Player::overlaps(player_pos, wz) {
                return true;
            }
        }
        false
    }

    pub fn check_door_collision(
        door: &GameObject,
        score_board: &mut crate::game::score_board::ScoreBoard,
        is_warp_zone: bool,
        player_pos: Vec2,
    ) -> bool {
        // Check for collision between player and door
        if score_board.game_won
            && Player::overlaps(
                player_pos,
                &Rect::new(door.world_x, door.world_y - 32.0, 32.0, 32.0),
            )
        {
            score_board.game_won = false;
            score_board.jetpack_captured = false;
            score_board.gun_captured = false;

            if score_board.level == 0 {
                score_board.level = 10;
            } else if !is_warp_zone {
                score_board.level += 1;
            }
            score_board.score += 2000;
            return true;
        }
        false
    }

    pub fn check_jetpack_collision(
        player: &mut Player,
        jetpack: &mut Option<GameObject>,
        score_board: &mut crate::game::score_board::ScoreBoard,
        player_pos: Vec2,
    ) -> bool {
        // Check for collision between player and jetpack
        if let Some(j) = jetpack {
            if !player.has_jetpack
                && Player::overlaps(
                    player_pos,
                    &Rect::new(j.world_x, j.world_y - 32.0, 32.0, 32.0),
                )
            {
                player.has_jetpack = true;
                score_board.jetpack_captured = true;
                jetpack.as_mut().unwrap().collected = Some(true);
                return true;
            }

            return false;
        }

        false
    }

    pub fn check_gun_collision(
        player: &mut Player,
        gun: Option<&GameObject>,
        score_board: &mut crate::game::score_board::ScoreBoard,
        player_pos: Vec2,
    ) -> bool {
        // Check for collision between player and gun
        if let Some(g) = gun {
            if !player.has_gun
                && Player::overlaps(
                    player_pos,
                    &Rect::new(g.world_x, g.world_y - 32.0, 32.0, 32.0),
                )
            {
                player.has_gun = true;
                score_board.gun_captured = true;
                return true;
            }

            return false;
        }

        false
    }

    pub fn handle_collision_with_deadly(
        deadly_objects: &HashMap<String, (Option<AnimatedSprite>, Vec<Object>)>,
        player: &mut Player,
        explosions: &mut Vec<(Emitter, Vec2)>,
        player_explosion_active: &mut bool,
        player_explosion_timer: &mut f32,
        resources: &Resources,
        player_pos: Vec2,
    ) {
        for (_, objects) in deadly_objects.values() {
            for object in objects {
                let deadly_rect =
                    Rect::new(object.world_x + 10.0, object.world_y - 10.0, 10.0, 7.0);

                if Player::overlaps(player_pos, &deadly_rect) && !player.is_dead {
                    player.is_dead = true;
                    *player_explosion_active = true;
                    *player_explosion_timer = EXPLOSION_DURATION;

                    if explosions.is_empty() {
                        explosions.push((
                            Emitter::new(EmitterConfig {
                                amount: 40,
                                texture: Some(resources.get_texture("explosion").clone()),
                                ..Self::particle_explosion()
                            }),
                            vec2(player_pos.x + 32.0, player_pos.y),
                        ));
                    }
                    play_sound_once(resources.get_sound("explosion"));
                    play_sound_once(resources.get_sound("hd-die-dave-7"));
                }
            }
        }
    }

    pub fn handle_collecting_valuables(
        collectibles: &mut Vec<GameObject>,
        player_pos: Vec2,
        score_board: &mut crate::game::score_board::ScoreBoard,
        resources: &Resources,
    ) {
        // Check for collision between player and Jewellery
        for jewellery in collectibles {
            let jewellery_rect = Rect::new(jewellery.world_x, jewellery.world_y - 32.0, 32.0, 32.0);

            if Player::overlaps(player_pos, &jewellery_rect) {
                if !score_board.game_won && jewellery.name == "cup" {
                    score_board.score += CollectibleType::Cup.data().value;
                    score_board.game_won = true;
                    play_sound_once(resources.get_sound("trophy"));
                } else {
                    score_board.score += CollectibleType::from(&jewellery.name).data().value;
                    jewellery.collected = Option::Some(true);
                    play_sound_once(resources.get_sound("getitem"));
                }
            }
        }
    }
}
