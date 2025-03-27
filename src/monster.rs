use std::vec;

use macroquad::{math::Vec2, prelude::{collections::storage, *}};
use macroquad_particles::Emitter;
use macroquad_tiled::Map;

use crate::{bullet::{Bullet, BulletDirection}, player::Player, resources::Resources};

const MONSTER_SPEED: f32 = 250.0;
const MONSTER_ROTATION_TIMER: f32 = 2.0;

#[derive(Debug, Clone)]
pub struct PolyPoint {
    pub x: f32,
    pub y: f32
}

#[derive(Clone)]
pub struct Monster {
    pub location: PolyPoint,
    waypoints: Vec<Vec2>,
    current_waypoint: usize,
    pub alive: bool,
    pub bullets: Vec<Bullet>,
    name: String,
    move_timer: f32,
    bullet_timer: f32,
    pub rotate: bool,
    rotation_degree: f32,
    rotation_timer: f32,
    rotate_y_axis: bool,
    pub kill_value: u32,
}

impl Default for Monster {
    fn default() -> Self {
        Self {
            location: PolyPoint { x: 0.0, y: 0.0 },
            waypoints: Vec::new(),
            current_waypoint: 0,
            alive: true,
            bullets: Vec::new(),
            name: String::new(),
            move_timer: 0.0,
            bullet_timer: 0.0,
            rotate: false,
            rotation_degree: 0.0,
            rotation_timer: 0.0,
            rotate_y_axis: false,
            kill_value: 200
        }
    }
}

impl Monster {
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
                                ..Game::particle_explosion()
                            }),
                            vec2(player_pos.x, player_pos.y),
                        ));
                        explosions.push((
                            Emitter::new(EmitterConfig {
                                amount: 40,
                                texture: Some(resources.get_texture("explosion").clone()),
                                ..Game::particle_explosion()
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
                                    ..Game::particle_explosion()
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
                                    ..Game::particle_explosion()
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
    
    fn get_line_points_lerp(p1: Vec2, p2: Vec2, steps: usize) -> Vec<Vec2> {
        let mut points = Vec::new();
    
        for i in 0..=steps {
            let t = i as f32 / steps as f32;  // Interpolation factor (0.0 to 1.0)
            let interpolated = p1.lerp(p2, t); // Using glam's built-in lerp()
            points.push(interpolated);
        }
    
        points
    }

    fn generate_pairs(points: &[Vec2]) -> Vec<(Vec2, Vec2)> {
        let mut pairs = Vec::new();
        
        if points.len() < 2 {
            return pairs; // Not enough points to form pairs
        }
    
        for i in 0..points.len() {
            let next_index = (i + 1) % points.len(); // Wrap around to form a closed loop
            pairs.push((points[i], points[next_index]));
        }
    
        pairs
    }

    pub fn load_monsters(tiled_map: &Map) -> Vec<Self> {
        let mut monsters: Vec<Self> = vec![];
        for layer in &tiled_map.raw_tiled_map.layers {
            if layer.name == "monsters" {
                for monster_obj in &layer.objects {
                    let mut monster: Self = Self {
                        location: PolyPoint {
                            x: monster_obj.x,
                            y: monster_obj.y
                        },
                        name: monster_obj.name.clone(),
                        rotate: monster_obj.properties.iter().any(|e| e.name == "rotate"),
                        rotate_y_axis: monster_obj.properties.iter().any(|e| e.name == "y-axis"),
                        ..Default::default()
                    };

                    let polygon_pts = monster_obj.polygon.as_ref().unwrap();
                    let mapped_points = polygon_pts
                                            .iter()
                                            .map(|p| Vec2::new(p.x, p.y))
                                            .collect::<Vec<Vec2>>();
                    
                    let pairs = Self::generate_pairs(&mapped_points);    
                    
                    for (p1, p2) in pairs {
                        let points_between = Self::get_line_points_lerp(p1, p2, 10);
                        monster.waypoints.extend(points_between.iter());
                    }
                    
                    monsters.push(monster);
                }
            }
        }

        monsters
    }

    pub fn monster_rectangle(&self) -> Rect {
        Rect::new(
            self.location.x + self.waypoints[self.current_waypoint].x,
            self.location.y + self.waypoints[self.current_waypoint].y,
            32.0,
            32.0,
        )
    }

    pub fn current_location(&self) -> Vec2 {
        Vec2::new(self.location.x + self.current_waypoint().x, self.location.y + self.current_waypoint().y)
    }

    pub fn current_waypoint(&self) -> &Vec2  {
        &self.waypoints[self.current_waypoint]
    }

    pub fn draw(&self) {
        let resources = storage::get::<Resources>();
        let point = self.waypoints[self.current_waypoint];
        let m = &resources.monsters[self.name.parse::<usize>().unwrap() - 1];
    
        // Create the default draw parameters
        let mut draw_params = DrawTextureParams {
            dest_size: Some(vec2(m.width(), m.height())),
            //flip_y: true,
            ..Default::default()
        };
    
        if self.rotate {
            // If rotating around z-axis (normal rotation)
            if !self.rotate_y_axis {
                draw_params.rotation = self.rotation_degree;
                draw_params.pivot = Some(Vec2::new(
                    self.location.x + point.x + m.width()/2.0, 
                    self.location.y + point.y + m.height()/2.0
                ));
            }
        }
    
        // Handle y-axis rotation separately 
        if self.rotate_y_axis {
            // Calculate rotation based on time or existing rotation_degree
            let angle = self.rotation_degree * std::f32::consts::PI / 180.0; // Convert to radians
            
            // Calculate scale factor - width narrows as we rotate to the side
            let scale_x = angle.cos().abs();
            
            // Calculate x position offset to keep the texture centered during rotation
            let width = m.width();
            let center_offset = (width - width * scale_x) / 2.0;
            
            // Update draw parameters for y-axis rotation
            draw_params.dest_size = Some(vec2(width * scale_x, m.height()));
            
            // Draw with adjusted position to maintain center point
            draw_texture_ex(
                m,
                self.location.x + point.x + center_offset,
                self.location.y + point.y,
                WHITE,
                draw_params
            );
        } else {
            // Draw with regular parameters (z-axis rotation or no rotation)
            draw_texture_ex(
                m,
                self.location.x + point.x,
                self.location.y + point.y,
                WHITE,
                draw_params
            );
        }
    
        // Draw monster bullets
        for monster_bullet in &self.bullets {
            draw_texture_ex(
                resources.get_texture("monster_bullet"),
                monster_bullet.x,
                monster_bullet.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(
                        resources.get_texture("monster_bullet").width(), 
                        resources.get_texture("monster_bullet").height()
                    )),
                    rotation: if monster_bullet.direction == BulletDirection::Right { std::f32::consts::PI } else { 0.0 },
                    ..Default::default()
                },
            );
        }
    }

    
    pub fn update(&mut self, player_pos: Vec2) {
        let point = self.waypoints[self.current_waypoint];
    
        if self.rotate {
            self.rotation_timer -= 150.0 * get_frame_time();
            if self.rotation_timer <= 0.0 {
                self.rotation_degree += 0.5;
                self.rotation_timer = MONSTER_ROTATION_TIMER;
            }
        }

        if self.move_timer > 0.0 {
            self.move_timer -= MONSTER_SPEED * get_frame_time();
        }
        else {
            if self.current_waypoint < self.waypoints.len() - 1 {
                self.current_waypoint += 1;
            }    
            else {
                self.current_waypoint = 0;
            }
            self.move_timer = 0.1;
        }

        if self.bullet_timer > 0.0 {
            self.bullet_timer -= get_frame_time();
        }
        else {
            //only allow monster to shoot if he is close to the player
            if self.location.x + point.x - player_pos.x < screen_width() * 0.7 {
                //shoot a bullet
                self.bullets.push(Bullet {
                    x: self.location.x + point.x + 10.0,
                    y: self.location.y + point.y,
                    speed: 250.0,
                    collided: false,
                    direction: if player_pos.x < self.location.x + point.x {BulletDirection::Left} else {BulletDirection::Right}
                });
            }

            self.bullet_timer = 6.0;
        }

        for monster_bullet in &mut self.bullets {
            if monster_bullet.direction == BulletDirection::Left {
                monster_bullet.x -= monster_bullet.speed * get_frame_time();
            }
            else {
                monster_bullet.x += monster_bullet.speed * get_frame_time();
            }
        }
    }

    
}
