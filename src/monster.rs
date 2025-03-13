use std::vec;

use macroquad::{math::Vec2, prelude::{collections::storage, *}};
use macroquad_tiled::Map;

use crate::{bullet::{Bullet, BulletDirection}, player, resources::Resources};

const MONSTER_SPEED: f32 = 250.0;
const MONSTER_ROTATION_TIMER: f32 = 6.0;

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
    rotation_timer: f32
}

impl Monster {
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
                        current_waypoint: 0, 
                        alive: true,
                        waypoints: Vec::new(),
                        bullets: vec![],
                        name: monster_obj.name.clone(),
                        move_timer: 0.1,
                        bullet_timer: 6.0,
                        rotate: monster_obj.properties.iter().any(|e| e.name == "rotate"),
                        rotation_degree: 0.0,
                        rotation_timer: MONSTER_ROTATION_TIMER
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

    pub fn update(&mut self, player_pos: Vec2) {
        let resources = storage::get::<Resources>();

        let point = self.waypoints[self.current_waypoint];
        let m = &resources.monsters[self.name.parse::<usize>().unwrap() - 1];

        if self.rotate {
            self.rotation_timer -= 30.0 * get_frame_time();
            if self.rotation_timer <= 0.0 {
                self.rotation_degree += 0.5;
                self.rotation_timer = MONSTER_ROTATION_TIMER;
            }
        }

        draw_texture_ex(
            m,
            self.location.x + point.x,
            self.location.y + point.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(m.width() , m.height() )), 
                rotation: self.rotation_degree, 
                ..Default::default()
            },
            
        );

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
}
