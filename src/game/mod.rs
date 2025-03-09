use std::ops::Deref;
use std::vec;

use macroquad::prelude::{*, animation::*, collections::storage};
use macroquad::audio::*;
use macroquad_platformer::{Tile, World};
use macroquad_tiled::{load_map, Map, Object};
use macroquad_particles::*;

use crate::score_board::GameObject;
use crate::{player::Player, monster::Monster, resources::Resources, score_board::ScoreBoard, Scene, SceneChange};

const EXPLOSION_DURATION: f32 = 2.0;


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
    gun: Option<GameObject>,
    cheat: bool,
    monsters: Vec<Monster>
}

impl Game {
    pub fn new(level: i32, retry: bool, cheat: bool) -> Game {
        let resources = storage::get::<Resources>();
        
        let tiled_map = load_map(
            &resources.levels[(level - 1) as usize],
            &[
                ("images/mytileset.png", resources.get_texture("mytileset").clone()),
                ("images/dave_walk.png", resources.get_texture("dave_walk").clone()),
                ("images/dave_idle.png", resources.get_texture("dave_idle").clone()),
                ("images/dave_jump.png", resources.get_texture("dave_jump").clone()),
                ("images/collectibles.png", resources.get_texture("collectibles").clone()),
                ("images/door.png", resources.get_texture("door").clone()),
                ("images/tuple.png", resources.get_texture("tuple").clone()),
                ("images/tuple_r.png", resources.get_texture("tuple_r").clone()),   
                ("images/deadly.png", resources.get_texture("deadly").clone()),     
                ("images/fire1-sheet.png", resources.get_texture("fire1-sheet").clone()),
                ("images/water1-sheet.png", resources.get_texture("water1-sheet").clone()),
                ("images/door_enable_banner.png", resources.get_texture("door_enable_banner").clone()),
                ("images/gun_icon.png", resources.get_texture("gun_icon").clone()),
                ("images/gun.png", resources.get_texture("gun").clone()),
                ("images/jetpack2.png", resources.get_texture("jetpack2").clone()),
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
    
        let mut score_board = 
                        if (cheat || level == 1) && !retry {
                            ScoreBoard::new()
                        } else { 
                            storage::get::<ScoreBoard>().clone() 
                        };

        if cheat {
            score_board.level = level;
        }

        let player = Player::new(actor, score_board.gun_captured);

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

        let gun = if tiled_map.contains_layer("gun") {     
            let gun_object = tiled_map.layers.get("gun").unwrap().objects.first().unwrap();    
            Some(GameObject {
                world_x: gun_object.world_x,
                world_y: gun_object.world_y,
                name: gun_object.name.clone(),
                collected: None,
            })
        }
        else {
            None
        };

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
            monsters = Monster::load_monsters(&tiled_map);
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
            gun,
            cheat,
            monsters
        }
    }

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
                "yussuk" => 160.0,
                _ => 192.0
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

    fn draw_gun(&self, tiled_map: &Map, gun: &GameObject, resources: &Resources) {
        if !self.player.has_gun {
            tiled_map.spr_ex(
                "gun_icon",
                Rect::new(0.0, 0.0, 32.0, 32.0),
                Rect::new(gun.world_x, gun.world_y - 32.0, 32.0, 32.0),
            );
        }
        else {
            draw_texture_ex(
                resources.get_texture("gun"),
                self.message_coord.0 + self.camera.target.x - 20.0,
                self.message_coord.1 - 32.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(
                        resources.get_texture("gun").width() , 
                        resources.get_texture("gun").height() 
                    )), 
                    ..Default::default()
                },
            );

            draw_texture_ex(
                resources.get_texture("gun_icon"),
                self.message_coord.0 + self.camera.target.x + 60.0,
                self.message_coord.1 - 32.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(
                        resources.get_texture("gun_icon").width() , 
                        resources.get_texture("gun_icon").height() 
                    )), 
                    ..Default::default()
                },
            );
        }
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
    
    fn handle_collecting_valuables(&mut self, resources: &Resources, pos: Vec2) {
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
                    play_sound_once(resources.get_sound("trophy"));
                }
                else {
                    self.score_board.score += 10;
                    jewellery.collected = Option::Some(true);
                    play_sound_once(resources.get_sound("getitem"));
                }
            }
        }
    }
    
    fn handle_collision_with_deadly(&mut self, resources: &impl Deref<Target = Resources>, pos: Vec2) {
        self.deadly_objects.iter().for_each(|deadly_object| {
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
                        texture: Some(resources.get_texture("explosion").clone()),
                        ..Game::particle_explosion()
                    }), vec2(pos.x + 32.0, pos.y)));
                }
                play_sound_once(resources.get_sound("explosion"));
                play_sound_once(resources.get_sound("hd-die-dave-7"));
            }
        });
    }
    
    fn monster_mechanics(&mut self, resources: &Resources, pos: Vec2) {
        self.monsters.iter_mut().for_each(|monster| {
            if monster.alive {
                monster.update(&pos);
    
                if self.player.overlaps(pos, &monster.monster_rectangle()) {
                    self.player.is_dead = true;
                    monster.alive = false;
                    self.explosion_active = true;
                    self.explosion_timer = EXPLOSION_DURATION;
    
                    if self.explosions.is_empty() {
                        self.explosions.push((Emitter::new(EmitterConfig {
                            amount: 40,
                            texture: Some(resources.get_texture("explosion").clone()),
                            ..Game::particle_explosion()
                        }), vec2(pos.x, pos.y)));
                        self.explosions.push((Emitter::new(EmitterConfig {
                            amount: 40,
                            texture: Some(resources.get_texture("explosion").clone()),
                            ..Game::particle_explosion()
                        }), monster.current_location()));
                    }
    
                    play_sound_once(resources.get_sound("explosion"));
                    play_sound_once(resources.get_sound("hd-die-dave-7"));
                }
    
                for bullet in &mut self.player.bullets {
                    let bullet_rect = Rect {
                        x: bullet.x,
                        y: bullet.y,
                        w: resources.get_texture("bullet").width(),
                        h: resources.get_texture("bullet").height()
                    };
    
                    if bullet_rect.overlaps(&monster.monster_rectangle()) {
                        bullet.collided = true;
                        monster.alive = false;
                        if self.explosions.is_empty() {
                            self.explosions.push((Emitter::new(EmitterConfig {
                                amount: 40,
                                texture: Some(resources.get_texture("explosion").clone()),
                                ..Game::particle_explosion()
                            }), monster.current_location()));
                        }
    
                        play_sound_once(resources.get_sound("explosion"));
                    }
                }
    
                for bullet in &mut monster.bullets {
                    let bullet_rect = Rect {
                        x: bullet.x,
                        y: bullet.y,
                        w: resources.get_texture("monster_bullet").width(),
                        h: resources.get_texture("monster_bullet").height()
                    };
    
                    if self.player.overlaps(pos, &bullet_rect) {
                        bullet.collided = true;
                        self.player.is_dead = true;

                        self.explosion_active = true;
                        self.explosion_timer = EXPLOSION_DURATION;

                        if self.explosions.is_empty() {
                            self.explosions.push((Emitter::new(EmitterConfig {
                                amount: 40,
                                texture: Some(resources.get_texture("explosion").clone()),
                                ..Game::particle_explosion()
                            }), vec2(pos.x, pos.y)));
                        }
    
                        play_sound_once(resources.get_sound("explosion"));
                    }
                }
    
                monster.bullets.retain(|bullet| {
                    if self.world.collide_solids(Vec2::new(bullet.x, bullet.y), 20, 10) == Tile::Solid {
                        return false
                    }
            
                    if !bullet.collided && bullet.x > pos.x - 100.0 {
                        return true;
                    }
            
                    false
                });
            }
        });
    }
}

impl Scene for Game {
    fn update(&mut self) -> Option<SceneChange> {
        let resources = storage::get::<Resources>();

        // Set the camera to follow the player
        set_camera(&self.camera);

        let pos = self.world.actor_pos(self.player.collider);

        //Update camera position to follow the player
        if self.score_board.level > 1 {
            let screen_width = screen_width();
            let target_x = if (pos.x > screen_width / 2.0) && 
                              (pos.x < (self.width_tiles * 32) as f32 - screen_width / 3.0) {
                pos.x
            } else if pos.x > 200.0 && pos.x < (self.width_tiles * 32) as f32 - 
                              (if screen_width > 1000.0 {screen_width / 5.0} else {screen_width / 3.0}) {
                pos.x + 170.0
            } else if pos.x < 200.0 {
                305.0
            } else {
                self.camera.target.x
            };

            self.camera.target.x = self.camera.target.x + (target_x - self.camera.target.x) * 0.1;
            self.score_board.position = (self.camera.target.x - 300.0, pos.y);
        }

        self.handle_collecting_valuables(&resources, pos);

        self.collectibles.retain(|jewellery| !jewellery.collected.unwrap_or(false));

        // Check for collision between player and gun
        if let Some(g) = &self.gun { 
            if !self.player.has_gun && self.player.overlaps(pos, &Rect::new(
                g.world_x,
                g.world_y - 32.0,
                32.0,
                32.0,
            )) {
                play_sound_once(resources.get_sound("gotspecial"));
                self.player.has_gun = true;
                self.score_board.gun_captured = true;
            }
        }

        // Check for collision between player and door
        if self.score_board.game_won && self.player.overlaps(pos, &Rect::new(
            self.door.world_x,
            self.door.world_y - 32.0,
            32.0,
            32.0,
        )) {
            self.score_board.game_won = false;
            play_sound_once(resources.get_sound("win"));
            self.score_board.level += 1;
            storage::store(self.score_board.clone());
            return Some(SceneChange::Separator);
        }
        
        self.explosions.retain(|(explosion, _)| explosion.config.emitting);

        self.handle_collision_with_deadly(&resources, pos);

        if !self.explosion_active && self.player.is_dead {
            if self.score_board.lives == 0 {
                play_sound_once(resources.get_sound("gameoverman"));
                return Some(SceneChange::EntryScreen);
            } else {
                self.score_board.lives -= 1;
                self.score_board.collectibles = self.collectibles.clone();
                storage::store(self.score_board.clone());
                return Some(SceneChange::Game{level: self.score_board.level, retry: true, cheat: self.cheat});
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
        
        let screen_left = self.camera.target.x - screen_width() / 2.0;
        let screen_right = self.camera.target.x + screen_width() / 2.0;

        self.monster_mechanics(&resources, pos);

        self.player.bullets.retain(|bullet| {
            bullet.x < screen_right && bullet.x > screen_left && !bullet.collided
        });

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
        
        if let Some(g) = &self.gun {
            self.draw_gun(&tiled_map, g, &resources);
        }

        if self.score_board.game_won {
            draw_texture_ex(
                resources.get_texture("door_enable_banner"),
                self.message_coord.0 + self.camera.target.x - 300.0,
                self.message_coord.1 - 32.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(
                        resources.get_texture("door_enable_banner").width() , 
                        resources.get_texture("door_enable_banner").height() 
                    )), 
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