use std::vec;

use macroquad::prelude::{*, animation::*, collections::storage};
use macroquad::audio::*;
use macroquad_platformer::{Tile, World};
use macroquad_tiled::{load_map, Map, Object};
use macroquad_particles::*;



use crate::score_board::GameObject;
use crate::{player::Player, resources::Resources, score_board::ScoreBoard, Scene, SceneChange};

const EXPLOSION_DURATION: f32 = 2.0;
const MONSTER_SPEED: f32 = 10.0;

#[derive(Debug, Clone)]
struct PolyPoint {
    x: f32,
    y: f32
}

#[derive(Debug)]
struct Monster {
    location: PolyPoint,
    waypoints: Vec<Vec2>,
    current_waypoint: usize,
    alive: bool
}

pub struct Game {
    world: World,
    player: Player,
    collectibles: Vec<GameObject>,
    door: GameObject,
    score_board: ScoreBoard,
    height_tiles: i32,
    width_tiles: i32,
    animated_fire: Option<AnimatedSprite>,
    animated_water: Option<AnimatedSprite>,
    fires: Vec<Object>,
    waters: Vec<Object>,
    camera: Camera2D,
    animated_grass: Option<AnimatedSprite>,
    grasses: Vec<Object>,
    explosions: Vec<(Emitter, Vec2)>,
    explosion_active: bool,
    explosion_timer: f32,
    deadly_objects: Vec<Object>,
    message_coord: (f32, f32),
    monsters: Vec<Monster>,
    timer: f32
}

impl Game {
    pub fn new(level: i32, retry: bool) -> Game {
        let resources = storage::get::<Resources>();
        
        let tiled_map = load_map(
            &resources.levels[(level - 1) as usize],
            &[
                ("mytileset.png", resources.tileset.clone()),
                ("dave_walk.png", resources.player_walk.clone()),
                ("dave_idle.png", resources.player_idle.clone()),
                ("dave_jump.png", resources.player_jump.clone()),
                ("collectibles.png", resources.collectibles.clone()),
                ("door.png", resources.door.clone()),
                ("tuple.png", resources.tuple.clone()),   
                ("deadly.png", resources.deadly_grass_texture.clone()),     
                ("fire1-sheet.png", resources.fire1.clone()),
                ("water1-sheet.png", resources.water_texture.clone()),
                ("gun_icon.png", resources.gun.clone()),
                ("king.png", resources.king.clone()),
                ("lolipop.png", resources.lolipop.clone()),
                ("door_enable_banner.png", resources.go_thru.clone()),
                ("yussuk.png", resources.yussuk.clone())
            ],
            &[],
        )
        .unwrap();

        storage::store(tiled_map);

        let tiled_map = storage::get::<Map>();

        let mut static_colliders = vec![];
        for (_x, _y, tile) in tiled_map.tiles("platform", None) {
            static_colliders.push(if tile.is_some() {
                Tile::Solid
            } else {
                Tile::Empty
            });
        }

        let height = tiled_map.layers.get("platform").unwrap().height;
        let width = tiled_map.layers.get("platform").unwrap().width;
        
        let mut world = World::new();
        world.add_static_tiled_layer(static_colliders, 32., 32., width as usize, 1);
        
        let player_loc = tiled_map.layers.get("player").unwrap().objects.first().unwrap();

        let actor = world.add_actor(vec2(player_loc.world_x, player_loc.world_y - 32.0), 32, 32);
    
        let player = Player::new(actor);

        if tiled_map.layers.contains_key("monster") {
            let monster_layer = tiled_map.layers.get("monster").unwrap();
            for object in &monster_layer.objects {
                if object.name == "another" {
                    println!("{:?}", object);
                }
            }
        }


        let score_board = 
                        if level == 1 && !retry { 
                            ScoreBoard::new()
                        } 
                        else { 
                            storage::get::<ScoreBoard>().clone() 
                        };

        let objects_layer = tiled_map.layers.get("collectibles").unwrap();
        let collectibles = 
            if retry { score_board.collectibles.clone() } 
            else { 
                objects_layer.objects
                .iter()
                .map(|entry| 
                    GameObject {
                        world_x: entry.world_x,
                        world_y: entry.world_y,
                        name: entry.name.clone(),
                        collected: None,
                    }
            ).collect::<Vec<GameObject>>()};
        
        let door = tiled_map.layers.get("door").unwrap().objects.first().unwrap();

        let door = GameObject {
            world_x: door.world_x,
            world_y: door.world_y,
            name: door.name.clone(),
            collected: None,
        };
        
        let (animated_fire, fires) = 
                            Game::load_animation(&tiled_map, "fire", 3);
        let (animated_water, waters) = 
                            Game::load_animation(&tiled_map, "water", 5);

        let (animated_grass, grasses) = 
                            Game::load_animation(&tiled_map, "grass", 4);

        let mut deadly_objects: Vec<Object> = Vec::new();

        deadly_objects.extend(fires.iter().cloned());
        deadly_objects.extend(waters.iter().cloned());
        deadly_objects.extend(grasses.iter().cloned());
        
        let camera = Camera2D::from_display_rect(Rect::new(0.0, 384.0, 608.0, -384.0));

        let message_coord = (
            tiled_map.layers.get("message").unwrap().objects[0].world_x, 
            tiled_map.layers.get("message").unwrap().objects[0].world_y
        );

        let mut monsters: Vec<Monster>  = Vec::new();
        
        if tiled_map.contains_layer("monsters") {
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
                            waypoints: Vec::new()
                        };

                        let polygon_pts = monster_obj.polygon.as_ref().unwrap();
                        let mapped_points = polygon_pts
                                                .iter()
                                                .map(|p| Vec2::new(p.x, p.y))
                                                .collect::<Vec<Vec2>>();
                        
                        let pairs = Game::generate_pairs(&mapped_points);    
                        
                        for (p1, p2) in pairs {
                            let points_between = Game::get_line_points_lerp(p1, p2, 10);
                            monster.waypoints.extend(points_between.iter());
                        }
                        
                        monsters.push(monster);
                    }
                }
            }
        }

        Game {
            world,
            player,
            collectibles,
            door,
            score_board,
            height_tiles: height as i32,
            width_tiles: width as i32,
            animated_fire,
            fires,
            animated_water,
            waters,
            camera,
            animated_grass,
            grasses,
            explosions: vec![],
            explosion_active: false,
            explosion_timer: 2.0,
            deadly_objects,
            message_coord,
            monsters,
            timer: 0.1
        }
    }

    fn particle_explosion() -> EmitterConfig {
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
    
    fn load_animation(tiled_map: &Map, name: &str, frames: i32) -> (Option<AnimatedSprite>, Vec<Object>) {
        let mut objects = vec![];
        let mut animated_object: Option<AnimatedSprite> = None;
        if tiled_map.layers.contains_key(name) {
            animated_object = Some(create_animation(name, frames));
            
            let object_layer = tiled_map.layers.get(name).unwrap();
            objects = object_layer.objects.clone();
        }

        (animated_object, objects)
    } 

    fn draw_collectibles(&self, tiled_map: &Map) {
        for diamond in &self.collectibles {
            let x = match diamond.name.as_str() {
                "ruby" => 0.0,
                "diamond" => 32.0,
                "red" => 64.0,
                "loli" => 96.0,
                "cup" => 128.0,
                _ => 160.0
            };

            tiled_map.spr_ex(
                "collectibles",
                Rect::new(x, 0.0, 32.0, 32.0),
                Rect::new(diamond.world_x, diamond.world_y - 32.0, 32.0, 32.0),
            );
        }
    }

    fn draw_door(&self, tiled_map: &Map) {
        tiled_map.spr_ex(
            "door",
            Rect::new(0.0, 0.0, 32.0, 32.0),
            Rect::new(self.door.world_x, self.door.world_y - 32.0, 32.0, 32.0),
        );
    }

    fn draw_animated_objects(&self, tiled_map: &Map) {
        if let Some(animated_fire) = &self.animated_fire {
            for fire in &self.fires {
                tiled_map.spr_ex(
                    "fire1-sheet",
                    animated_fire.frame().source_rect,
                    Rect::new(fire.world_x, fire.world_y - 32.0, 32.0, 32.0),
                );
            }
        }

        if let Some(animated_water) = &self.animated_water {
            for water in &self.waters {
                tiled_map.spr_ex(
                    "water1-sheet",
                    animated_water.frame().source_rect,
                    Rect::new(water.world_x, water.world_y - 32.0, 32.0, 32.0),
                );
            }
        }

        if let Some(animated_grass) = &self.animated_grass {
            for grass in &self.grasses {
                tiled_map.spr_ex(
                    "deadly",
                    animated_grass.frame().source_rect,
                    Rect::new(grass.world_x, grass.world_y - 32.0, 32.0, 32.0),
                );
            }
        }
    }

    fn draw_tiles(&self, tiled_map: &Map) {
        tiled_map.draw_tiles(
            "platform",
            Rect::new(0.0, 0.0, (self.width_tiles * 32) as f32, (self.height_tiles * 32) as f32),
            None,
        );

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

    
}

impl Scene for Game {
    fn update(&mut self) -> Option<SceneChange> {
        let resources = storage::get::<Resources>();

        // Set the camera to follow the player
        set_camera(&self.camera);

        let pos = self.world.actor_pos(self.player.collider);

        //Update camera position to follow the player
        if self.width_tiles as f32 * 32.0 > screen_width() {
            let target_x = if (pos.x > screen_width() / 2.0) && 
                              (pos.x < (self.width_tiles * 32) as f32 - screen_width() / 3.0) {
                pos.x
            } else if pos.x > 200.0 && pos.x < (self.width_tiles * 32) as f32 - screen_width() / 3.0 {
                pos.x + 150.0
            } else if pos.x < 200.0 {
                305.0
            } else {
                self.camera.target.x
            };

            self.camera.target.x = self.camera.target.x + (target_x - self.camera.target.x) * 0.1;
            self.score_board.position = (self.camera.target.x - 300.0, pos.y);
        }

        // Check for collision between player and Jewellery
        for jewellery in self.collectibles.iter_mut() {
            let jewellery_rect = Rect::new(
                jewellery.world_x,
                jewellery.world_y - 32.0,
                32.0,
                32.0,
            );

            if self.player.overlaps(pos, &jewellery_rect) {
                if !self.score_board.game_won && jewellery.name == "cup" {
                    self.score_board.score += 100;
                    self.score_board.game_won = true;
                    play_sound_once(&resources.sound_cup);
                }
                else {
                    self.score_board.score += 10;
                    jewellery.collected = Option::Some(true);
                    play_sound_once(&resources.sound_collect);
                }
            }
        }

        self.collectibles.retain(|jewellery| !jewellery.collected.unwrap_or(false));

        // Check for collision between player and door
        if self.score_board.game_won && self.player.overlaps(pos, &Rect::new(
            self.door.world_x,
            self.door.world_y - 32.0,
            32.0,
            32.0,
        )) {
            self.score_board.game_won = false;
            play_sound_once(&resources.sound_win);
            self.score_board.level += 1;
            storage::store(self.score_board.clone());
            return Some(SceneChange::Separator);
        }
        
        self.explosions.retain(|(explosion, _)| explosion.config.emitting);

        for deadly_object in &self.deadly_objects {
            let deadly_rect = Rect::new(
                deadly_object.world_x,
                deadly_object.world_y - 32.0,
                10.0,
                10.0,
            );

            if self.player.overlaps(pos, &deadly_rect) && !self.player.is_dead {
                self.player.is_dead = true;    
                self.explosion_active = true;
                self.explosion_timer = EXPLOSION_DURATION;

                if self.explosions.is_empty() {
                    self.explosions.push((Emitter::new(EmitterConfig {
                        amount: 40,
                        texture: Some(resources.explosion.clone()),
                        ..Game::particle_explosion()
                    }), vec2(pos.x + 32.0, pos.y)));
                }
                play_sound_once(&resources.sound_explosion);
                play_sound_once(&resources.sound_die);
            }
        }

        if !self.explosion_active && self.player.is_dead {
            if self.score_board.lives == 0 {
                play_sound_once(&resources.sound_gameover);
                return Some(SceneChange::MainMenu);
            } else {
                self.score_board.lives -= 1;
                self.score_board.collectibles = self.collectibles.clone();
                storage::store(self.score_board.clone());
                return Some(SceneChange::Game{level: self.score_board.level, retry: true});
            }
        }


        for (explosion, coords) in &mut self.explosions {
            explosion.draw(vec2(coords.x, coords.y));
        }

        if self.explosion_active {
            self.explosion_timer -= get_frame_time();
            if self.explosion_timer <= 0.0 {
                self.explosion_active = false;
            }
        }
        
        self.player.update(&mut self.world);

        if self.animated_fire.is_some() {
            self.animated_fire.as_mut().unwrap().update();
        }

        if self.animated_water.is_some() {
            self.animated_water.as_mut().unwrap().update();
        }

        if self.animated_grass.is_some() {
            self.animated_grass.as_mut().unwrap().update();
        }
        
        for monster in &mut self.monsters {
            if monster.alive {
                let point = &monster.waypoints[monster.current_waypoint];
                
                draw_texture_ex(
                    &resources.monster1,
                    monster.location.x + point.x,
                    monster.location.y + point.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(resources.monster1.width() , resources.monster1.height() )), 
                        ..Default::default()
                    },
                );

                if self.timer > 0.0 {
                    self.timer -= MONSTER_SPEED * get_frame_time();
                }
                else {
                    if monster.current_waypoint < monster.waypoints.len() - 1 {
                        monster.current_waypoint += 1;
                    }    
                    else {
                        monster.current_waypoint = 0;
                    }
                    self.timer = 0.1;
                }
            }
        }

        None
    }

    fn draw(&self) {
        let tiled_map = storage::get::<Map>();
        let resources = storage::get::<Resources>();

        self.score_board.draw();
        self.draw_tiles(&tiled_map);
        self.draw_collectibles(&tiled_map);
        self.draw_door(&tiled_map);
        self.draw_animated_objects(&tiled_map);

        if self.score_board.game_won {
            draw_texture_ex(
                &resources.go_thru,
                self.message_coord.0 + self.camera.target.x - 300.0,
                self.message_coord.1 - 32.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(resources.go_thru.width() , resources.go_thru.height() )), 
                    ..Default::default()
                },
            );
        }


        

    }
}

fn create_animation(name: &str, frames: i32) -> AnimatedSprite {
    let mut ani = AnimatedSprite::new(
        32,
        32,
        &[
            Animation {
                name: name.to_string(),
                row: 0,
                frames: frames as u32,
                fps: 4,
            }
        ],
        true,
    );

    ani.set_animation(0);
    ani
}