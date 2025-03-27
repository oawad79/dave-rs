use std::collections::HashMap;

use macroquad::{
    audio::play_sound_once,
    math::{vec2, Rect, Vec2}, prelude::animation::AnimatedSprite,
};
use macroquad_particles::{AtlasConfig, Emitter, EmitterConfig};
use macroquad_platformer::Tile;
use macroquad_tiled::Object;

use crate::{
    monster::Monster, 
    player::Player, 
    resources::Resources, 
    score_board::GameObject
};

use super::{GameState, collectibles::CollectibleType};

static EXPLOSION_DURATION: f32 = 2.0;

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

    pub fn check_warp_zone_collision(warp_zone_rect: &Option<Rect>, player_pos: Vec2) -> bool {
        if let Some(wz) = warp_zone_rect {
            if Player::overlaps(player_pos, wz) {
                return true;
            }
        }
        false
    }

    pub fn check_door_collision(
        door: &GameObject,
        score_board: &mut crate::score_board::ScoreBoard,
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
            println!("Door collision detected");
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

    pub fn check_jetpack_collision(player: &mut Player,
        jetpack: &mut Option<GameObject>,
        score_board: &mut crate::score_board::ScoreBoard,
        player_pos: Vec2) -> bool {
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
        gun: &Option<GameObject>,
        score_board: &mut crate::score_board::ScoreBoard,
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

    pub fn handle_monster_collisions(
        monsters: &mut Vec<Monster>,
        player: &mut Player,
        score_board: &mut crate::score_board::ScoreBoard,
        explosions: &mut Vec<(Emitter, Vec2)>,
        game_state: &mut GameState,
        resources: &Resources,
        world: &mut macroquad_platformer::World,
        player_pos: Vec2,
    ) {
        monsters.iter_mut().for_each(|monster| {
            if monster.alive {
                monster.update(player_pos);
                monster.draw();

                if Player::overlaps(player_pos, &monster.monster_rectangle()) {
                    player.is_dead = true;
                    monster.alive = false;
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
                            vec2(player_pos.x, player_pos.y),
                        ));
                        explosions.push((
                            Emitter::new(EmitterConfig {
                                amount: 40,
                                texture: Some(resources.get_texture("explosion").clone()),
                                ..Self::particle_explosion()
                            }),
                            monster.current_location(),
                        ));
                    }

                    play_sound_once(resources.get_sound("explosion"));
                    play_sound_once(resources.get_sound("hd-die-dave-7"));
                }

                for bullet in &mut player.bullets {
                    let bullet_rect = Rect {
                        x: bullet.x,
                        y: bullet.y,
                        w: resources.get_texture("bullet").width(),
                        h: resources.get_texture("bullet").height(),
                    };

                    if bullet_rect.overlaps(&monster.monster_rectangle()) {
                        bullet.collided = true;
                        monster.alive = false;
                        if explosions.is_empty() {
                            explosions.push((
                                Emitter::new(EmitterConfig {
                                    amount: 40,
                                    texture: Some(resources.get_texture("explosion").clone()),
                                    ..Self::particle_explosion()
                                }),
                                monster.current_location(),
                            ));
                        }

                        play_sound_once(resources.get_sound("explosion"));
                    }
                }

                for bullet in &mut monster.bullets {
                    let bullet_rect = Rect {
                        x: bullet.x,
                        y: bullet.y,
                        w: resources.get_texture("monster_bullet").width(),
                        h: resources.get_texture("monster_bullet").height(),
                    };

                    if Player::overlaps(player_pos, &bullet_rect) {
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
                                vec2(player_pos.x, player_pos.y),
                            ));
                        }

                        play_sound_once(resources.get_sound("explosion"));
                    }
                }

                monster.bullets.retain(|bullet| {
                    if world.collide_solids(Vec2::new(bullet.x, bullet.y), 20, 10) == Tile::Solid {
                        return false;
                    }

                    if !bullet.collided && bullet.x > player_pos.x - 100.0 {
                        return true;
                    }

                    false
                });
            }
        });
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
        
        deadly_objects.iter().for_each(|(_, (_, objects))| {
            objects.iter().for_each(|object| {
                let deadly_rect = Rect::new(
                object.world_x + 10.0,
                object.world_y - 10.0,
                10.0,
                7.0,
            );

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
        });
    });
}

    pub fn handle_collecting_valuables(
        collectibles: &mut Vec<GameObject>,
        player_pos: Vec2,
        score_board: &mut crate::score_board::ScoreBoard,
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
