use std::vec;

use animations::Animations;
use collision::CollisionManager;
use macroquad::prelude::{*, animation::*, collections::storage};
use macroquad::audio::{play_sound_once, stop_sound};
use macroquad_platformer::{Tile, World};
use macroquad_tiled::{load_map, Map, Object};
use macroquad_particles::{AtlasConfig, Emitter, EmitterConfig};

use crate::score_board::GameObject;
use crate::{
    player::Player, 
    monster::Monster, 
    resources::Resources, 
    score_board::ScoreBoard, 
    Scene, SceneChange
};
use collectibles::CollectibleType;


mod collectibles;
mod animations;
mod renderer;
mod collision;


const EXPLOSION_DURATION: f32 = 2.0;

pub struct GameWorld {
    pub world: World,
    pub height_tiles: i32,
    pub width_tiles: i32,
    pub camera: Camera2D
}

pub struct GameState {
    pub monster_explosion_active: bool,
    pub monster_explosion_timer: f32,
    pub player_explosion_active: bool,
    pub player_explosion_timer: f32,
    pub message_coord: (f32, f32),
    pub cheat: bool,
    pub is_warp_zone: bool,
}

pub struct AnimationAssets {
    pub animated_fire: Option<AnimatedSprite>,
    pub animated_water: Option<AnimatedSprite>,
    pub animated_grass: Option<AnimatedSprite>,
    pub fires: Vec<Object>,
    pub waters: Vec<Object>,
    pub grasses: Vec<Object>,
}

pub struct Game {
    game_world: GameWorld,
    game_state: GameState,
    animation_assets: AnimationAssets,
    player: Player,
    collectibles: Vec<GameObject>,
    door: GameObject,
    score_board: ScoreBoard,
    explosions: Vec<(Emitter, Vec2)>,
    deadly_objects: Vec<Object>,
    gun: Option<GameObject>,
    monsters: Vec<Monster>,
    jetpack: Option<GameObject>,
    warp_zone_rect: Option<Rect>,
    
}

impl Game {
    pub fn new(level: u32, retry: bool, cheat: bool, is_warp_zone: bool) -> Self {
        let resources = storage::get::<Resources>();
        
        let map_data = if is_warp_zone {
            resources.warp_zones.get(&i32::try_from(if level == 0 {10} else {level}).unwrap()).unwrap()
        }
        else {
            &resources.levels[(if level == 0 {9} else {level - 1}) as usize]
        };
        
        let tiled_map = load_map(
            map_data,
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
                ("images/player_jetpack.png", resources.get_texture("player_jetpack").clone()),
                ("images/stars-sheet.png", resources.get_texture("stars-sheet").clone()),
                ("images/tree.png", resources.get_texture("tree").clone()),
                ("images/climb-sheet.png", resources.get_texture("climb-sheet").clone()),
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

        let mut tree_static_colliders = vec![];
        if tiled_map.contains_layer("tree_collider") {
            
            for (_x, _y, tile) in tiled_map.tiles("tree_collider", None) {
                tree_static_colliders.push(if tile.is_some() {
                    Tile::JumpThrough
                } else {
                    Tile::Empty
                });
            }
        }

        let height = tiled_map.layers.get("platform").unwrap().height;
        let width = tiled_map.layers.get("platform").unwrap().width;
        
        let mut world = World::new();
        world.add_static_tiled_layer(static_colliders, 32., 32., width as usize, 1);
        world.add_static_tiled_layer(tree_static_colliders, 32., 32., width as usize, 2);

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

        let attach = tiled_map.layers.get("player").unwrap().objects.first().unwrap().properties.get("attach");

        let player = Player::new(actor, 
            score_board.gun_captured, score_board.jetpack_captured, attach.is_some());

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
                        width: entry.world_w,
                        height: entry.world_h,
                        name: entry.name.clone(),
                        collected: None,
                        progress: 0.0
                    }
            ).collect::<Vec<GameObject>>()};

        let gun = if tiled_map.contains_layer("gun") {     
            let gun_object = tiled_map.layers.get("gun").unwrap().objects.first().unwrap();    
            Some(GameObject {
                world_x: gun_object.world_x,
                world_y: gun_object.world_y,
                width: gun_object.world_w,
                height: gun_object.world_h,
                name: gun_object.name.clone(),
                collected: None,
                progress: 0.0
            })
        }
        else {
            None
        };

        let jetpack = if tiled_map.contains_layer("jetpack") {
            let jetpack_object = tiled_map.layers.get("jetpack").unwrap().objects.first().unwrap();    
            Some(GameObject {
                world_x: jetpack_object.world_x,
                world_y: jetpack_object.world_y,
                width: jetpack_object.world_w,
                height: jetpack_object.world_h,
                name: jetpack_object.name.clone(),
                collected: if score_board.jetpack_captured { Some(true) } else { None },
                progress: 0.0
            })
        }
        else {
            None
        };

        let door = tiled_map.layers.get("door").unwrap().objects.first().unwrap();

        let door = GameObject {
            world_x: door.world_x,
            world_y: door.world_y,
            width: door.world_w,
            height: door.world_h,
            name: door.name.clone(),
            collected: None,
            progress: 0.0
        };
        
        

        let (animated_fire, fires) = 
                            Animations::load_animation(&tiled_map, "fire", 3);
        let (animated_water, waters) = 
                            Animations::load_animation(&tiled_map, "water", 5);

        let (animated_grass, grasses) = 
                            Animations::load_animation(&tiled_map, "grass", 4);

        let animation_assets = AnimationAssets {
            animated_fire,
            animated_water,
            animated_grass,
            fires,
            waters,
            grasses,
        };

        

        let mut deadly_objects: Vec<Object> = Vec::new();

        deadly_objects.extend(animation_assets.fires.iter().cloned());
        deadly_objects.extend(animation_assets.waters.iter().cloned());
        deadly_objects.extend(animation_assets.grasses.iter().cloned());
        
        let camera = Camera2D::from_display_rect(Rect::new(0.0, 384.0, 608.0, -384.0));

        let message_coord = (
            tiled_map.layers.get("message").unwrap().objects[0].world_x, 
            tiled_map.layers.get("message").unwrap().objects[0].world_y
        );

        let monsters: Vec<Monster>  = if retry {
            score_board.monsters.clone()
        } 
        else if tiled_map.contains_layer("monsters") {
            Monster::load_monsters(&tiled_map)
        } 
        else {
            vec![]
        };
        
        let warp_zone_rect = if tiled_map.contains_layer("warp_zone") {
            let go = tiled_map.layers.get("warp_zone").unwrap().objects.first().unwrap();
            Some(Rect {
                x: go.world_x,
                y: go.world_y,
                w: go.world_w,
                h: go.world_h,
            })
                
        }
        else {
            None
        };

        let game_world = GameWorld {
            world,
            height_tiles: height as i32,
            width_tiles: width as i32,
            camera
        };

        let game_state = GameState {
            monster_explosion_active: false,
            monster_explosion_timer: 2.0,
            player_explosion_active: false,
            player_explosion_timer: 2.0,
            message_coord,
            cheat,
            is_warp_zone: false,
        };

        Self {
            game_world,
            game_state,
            animation_assets,
            player,
            collectibles,
            door,
            score_board,
            explosions: vec![],
            deadly_objects,
            gun,
            monsters,
            jetpack,
            warp_zone_rect,
            
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
    
}

impl Scene for Game {
    fn update(&mut self) -> Option<SceneChange> {
        let resources = storage::get::<Resources>();
        let tiled_map = storage::get::<Map>();

        // Set the camera to follow the player
        set_camera(&self.game_world.camera);

        if tiled_map.contains_layer("night") {
            tiled_map.draw_tiles(
                "night",
                Rect::new(0.0, 0.0, (self.game_world.width_tiles * 32) as f32, (self.game_world.height_tiles * 32) as f32),
                None,
            );
        }

        let pos = self.game_world.world.actor_pos(self.player.collider);

        //Update camera position to follow the player
        if self.score_board.level > 1 || self.score_board.level == 0 {
            let screen_width = screen_width();
            let target_x = if (pos.x > screen_width / 2.0) && 
                              (pos.x < (self.game_world.width_tiles * 32) as f32 - screen_width / 3.4) {
                pos.x
            } else if pos.x > 200.0 && pos.x < (self.game_world.width_tiles * 32) as f32 - 
                              (if screen_width > 1000.0 {screen_width / 5.0} else {screen_width / 3.0}) {
                pos.x + 170.0
            } else if pos.x < 200.0 {
                305.0
            } else {
                self.game_world.camera.target.x
            };

            self.game_world.camera.target.x = self.game_world.camera.target.x + (target_x - self.game_world.camera.target.x) * 0.1;
            self.score_board.position = (self.game_world.camera.target.x - 300.0, pos.y);
        }

        //handle the player falling out of the game so we bring him from top
        if pos.y > screen_height() && !self.player.is_dead {
            self.game_world.world.set_actor_position(self.player.collider, vec2(pos.x, 0.0));
        }

        CollisionManager::handle_collecting_valuables(&mut self.collectibles, pos, &mut self.score_board, &resources);

        self.collectibles.retain(|jewellery| !jewellery.collected.unwrap_or(false));

        if CollisionManager::check_warp_zone_collision(&self.warp_zone_rect, pos) {
            self.score_board.jetpack_captured = false;
            storage::store(self.score_board.clone());
            return Some(SceneChange::WarpZone);
        }

        CollisionManager::check_special_item_collisions(
            &mut self.player, 
            &self.gun, 
            &mut self.jetpack, 
            &mut self.score_board, 
            &resources, 
            pos
        );

        if CollisionManager::check_door_collision(
            &self.door, 
            &mut self.score_board, 
            self.game_state.is_warp_zone, 
            pos
        ) {
            storage::store(self.score_board.clone());
            play_sound_once(resources.get_sound("win"));
            stop_sound(resources.get_sound("jetPackActivated"));
            return Some(SceneChange::Separator);
        }

        self.explosions.retain(|(explosion, _)| explosion.config.emitting);

        CollisionManager::handle_collision_with_deadly(&self.deadly_objects, &mut self.player, &mut self.explosions, &mut self.game_state.player_explosion_active, &mut self.game_state.player_explosion_timer, &resources, pos);

        if !self.game_state.player_explosion_active && self.player.is_dead {
            if self.score_board.lives == 0 {
                play_sound_once(resources.get_sound("gameoverman"));
                return Some(SceneChange::EntryScreen);
            } 
            
            self.score_board.lives -= 1;
            if !self.game_state.is_warp_zone {
                self.score_board.collectibles = self.collectibles.clone();
                self.score_board.monsters = self.monsters.clone();
                self.score_board.jetpack_captured = self.player.has_jetpack; 
            }            
            
            storage::store(self.score_board.clone());
            return Some(SceneChange::Game{level: self.score_board.level, retry: !self.game_state.is_warp_zone, cheat: self.game_state.cheat, warp_zone: false});
            
        }
        
        for (explosion, coords) in &mut self.explosions {
            explosion.draw(vec2(coords.x, coords.y));
        }

        if self.game_state.monster_explosion_active {
            self.game_state.monster_explosion_timer -= get_frame_time();
            if self.game_state.monster_explosion_timer <= 0.0 {
                self.game_state.monster_explosion_active = false;
            }
        }

        

        if self.game_state.player_explosion_active {
            self.game_state.player_explosion_timer -= get_frame_time();
            if self.game_state.player_explosion_timer <= 0.0 {
                self.game_state.player_explosion_active = false;
            }
        }
        
        if tiled_map.contains_layer("tree_collider") {
            tiled_map.draw_tiles(
                "tree_collider",
                Rect::new(0.0, 0.0, (self.game_world.width_tiles * 32) as f32, (self.game_world.height_tiles * 32) as f32),
                None,
            );
        }

        if tiled_map.contains_layer("fallthroughtiles") {
            tiled_map.draw_tiles(
                "fallthroughtiles",
                Rect::new(0.0, 0.0, (self.game_world.width_tiles * 32) as f32, (self.game_world.height_tiles * 32) as f32),
                None,
            );
        }

        self.player.update(&mut self.game_world.world);

        if self.animation_assets.animated_fire.is_some() {
            self.animation_assets.animated_fire.as_mut().unwrap().update();
        }

        if self.animation_assets.animated_water.is_some() {
            self.animation_assets.animated_water.as_mut().unwrap().update();
        }

        if self.animation_assets.animated_grass.is_some() {
            self.animation_assets.animated_grass.as_mut().unwrap().update();
        }
        
        let screen_left = self.game_world.camera.target.x - screen_width() / 2.0;
        let screen_right = self.game_world.camera.target.x + screen_width() / 2.0;

        CollisionManager::handle_monster_collisions(
            &mut self.monsters, 
            &mut self.player, 
            &mut self.score_board, 
            &mut self.explosions, 
            &mut self.game_state, 
            &resources, 
            &mut self.game_world.world, 
            pos
        );

        self.player.bullets.retain(|bullet| {
            if self.game_world.world.collide_solids(Vec2::new(bullet.x, bullet.y), 20, 10) == Tile::Solid {
                return false
            }

            bullet.x < screen_right && bullet.x > screen_left && !bullet.collided
        });

        None
    }
    fn draw(&self) {
        let tiled_map = storage::get::<Map>();
        let resources = storage::get::<Resources>();

        self.score_board.draw();
        
        // Replace direct drawing with calls to the rendering module
        
        renderer::draw_tiles(&tiled_map, self.game_world.width_tiles, self.game_world.height_tiles);
        renderer::draw_collectibles(&self.collectibles, &tiled_map);
        renderer::draw_door(&self.door, &tiled_map);
        renderer::draw_animated_objects(
            &tiled_map,
            &self.animation_assets.animated_fire,
            &self.animation_assets.fires,
            &self.animation_assets.animated_water,
            &self.animation_assets.waters,
            &self.animation_assets.animated_grass,
            &self.animation_assets.grasses
        );
        
        if let Some(g) = &self.gun {
            renderer::draw_gun(
                &tiled_map, 
                g, 
                &resources, 
                self.player.has_gun, 
                self.game_state.message_coord, 
                self.game_world.camera.target.x
            );
        }

        if let Some(j) = self.jetpack.as_ref() {
            renderer::draw_jetpack(
                &tiled_map, 
                j, 
                &resources, 
                self.player.has_jetpack, 
                self.player.progress,
                self.game_state.message_coord, 
                self.game_world.camera.target.x
            );
        }

        renderer::draw_door_enable_banner(
            self.score_board.game_won, 
            &resources, 
            self.game_state.message_coord, 
            self.game_world.camera.target.x
        );
        
    }}

