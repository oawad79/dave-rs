use std::vec;

use macroquad::{math::Vec2, prelude::{collections::storage, *}};
use macroquad_tiled::Map;

use crate::{game::Bullet, resources::Resources};

const MONSTER_SPEED: f32 = 10.0;

#[derive(Debug, Clone)]
pub struct PolyPoint {
    pub x: f32,
    pub y: f32
}

pub struct Monster {
    pub location: PolyPoint,
    waypoints: Vec<Vec2>,
    current_waypoint: usize,
    pub alive: bool,
    pub bullets: Vec<Bullet>,
    name: String,
    monster_move_timer: f32,
    monster_bullet_timer: f32
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

    pub fn load_monsters(tiled_map: &Map) -> Vec<Monster> {
        let mut monsters: Vec<Monster> = vec![];
        for layer in &tiled_map.raw_tiled_map.layers {
            if layer.name == "monsters" {
                for monster_obj in &layer.objects {
                    let mut monster: Monster = Monster {
                        location: PolyPoint {
                            x: monster_obj.x,
                            y: monster_obj.y
                        },
                        current_waypoint: 0, 
                        alive: true,
                        waypoints: Vec::new(),
                        bullets: vec![],
                        name: monster_obj.name.clone(),
                        monster_move_timer: 0.1,
                        monster_bullet_timer: 6.0
                    };

                    let polygon_pts = monster_obj.polygon.as_ref().unwrap();
                    let mapped_points = polygon_pts
                                            .iter()
                                            .map(|p| Vec2::new(p.x, p.y))
                                            .collect::<Vec<Vec2>>();
                    
                    let pairs = Monster::generate_pairs(&mapped_points);    
                    
                    for (p1, p2) in pairs {
                        let points_between = Monster::get_line_points_lerp(p1, p2, 10);
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

    pub fn current_waypoint(&self) -> &Vec2  {
        &self.waypoints[self.current_waypoint]
    }

    pub fn update(&mut self, player_pos: &Vec2) {
        let resources = storage::get::<Resources>();

        let point = self.waypoints[self.current_waypoint];
        let m = &resources.monsters[self.name.parse::<usize>().unwrap() - 1];

        draw_texture_ex(
            m,
            self.location.x + point.x,
            self.location.y + point.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(m.width() , m.height() )), 
                ..Default::default()
            },
        );

        if self.monster_move_timer > 0.0 {
            self.monster_move_timer -= MONSTER_SPEED * get_frame_time();
        }
        else {
            if self.current_waypoint < self.waypoints.len() - 1 {
                self.current_waypoint += 1;
            }    
            else {
                self.current_waypoint = 0;
            }
            self.monster_move_timer = 0.1;
        }

        if self.monster_bullet_timer > 0.0 {
            self.monster_bullet_timer -= get_frame_time();
        }
        else {
            //only allow monster to shoot if he is close to the player
            if self.location.x + point.x - player_pos.x < screen_width() * 0.7 {
                //shoot a bullet
                self.bullets.push(Bullet {
                    x: self.location.x + point.x + 10.0,
                    y: self.location.y + point.y,
                    speed: 250.0,
                    collided: false 
                });
            }

            self.monster_bullet_timer = 6.0;
        }

        for monster_bullet in &mut self.bullets {
            monster_bullet.x -= monster_bullet.speed * get_frame_time();
        }

        for monster_bullet in &self.bullets {
            draw_texture_ex(
                &resources.monster_bullet,
                monster_bullet.x,
                monster_bullet.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(resources.monster_bullet.width(), resources.monster_bullet.height())),
                    ..Default::default()
                },
            );
        }
    }
}
