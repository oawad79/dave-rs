use macroquad::{audio::play_sound_once, math::{vec2, Rect}, prelude::collections::storage, time::get_frame_time};
use macroquad_platformer::{Tile, World};
use macroquad_tiled::{load_map, Map};

use crate::{player::Player, resources::Resources, Scene};

#[derive(Debug)]
struct Diamond {
    world_x: f32,
    world_y: f32,
    world_w: f32,
    world_h: f32,
    tile_x: u32,
    tile_y: u32,
    tile_w: u32,
    tile_h: u32,
    name: String,
    collected: bool,
}

struct GameObject {
    world_x: f32,
    world_y: f32,
    name: String,
}

pub struct Game {
    world: World,
    player: Player,
    diamonds: Vec<Diamond>,
    door: GameObject,
    trophy: GameObject,
    game_won: bool,
    tiled_map: Map,
}

impl Game {
    pub fn new() -> Game {
        let resources = storage::get::<Resources>();
        
        let tiled_map = load_map(
            &resources.tiled_map_json,
            &[
                ("mytileset.png", resources.tileset.clone()),
                ("dave_walk.png", resources.player_walk.clone()),
                ("dave_idle.png", resources.player_idle.clone()),
                ("dave_jump.png", resources.player_jump.clone()),
                ("collectibles.png", resources.collectibles.clone()),
                ("door.png", resources.door.clone()),
                ("tuple.png", resources.tuple.clone()),   
                ("cup.png", resources.cup.clone()),     
            ],
            &[],
        )
        .unwrap();

        let mut static_colliders = vec![];
        for (_x, _y, tile) in tiled_map.tiles("platform", None) {
            static_colliders.push(if tile.is_some() {
                Tile::Solid
            } else {
                Tile::Empty
            });
        }
        
        let mut world = World::new();
        world.add_static_tiled_layer(static_colliders, 32., 32., 19, 1);
    
        let actor = world.add_actor(vec2(70.0, 250.0), 32, 32);
    
        let player = Player::new(actor);

        let objects_layer = tiled_map.layers.get("collectibles").unwrap();
        let diamonds = objects_layer
            .objects
            .iter()
            .map(|entry| 
                Diamond {
                    world_x: entry.world_x,
                    world_y: entry.world_y,
                    world_w: entry.world_w,
                    world_h: entry.world_h,
                    tile_x: entry.tile_x,
                    tile_y: entry.tile_y,
                    tile_w: entry.tile_w,
                    tile_h: entry.tile_h,
                    name: entry.name.clone(),
                    collected: false,
                }
            ).collect::<Vec<Diamond>>();
        
        let door = tiled_map.layers.get("door").unwrap().objects.first().unwrap();

        let door = GameObject {
            world_x: door.world_x,
            world_y: door.world_y,
            name: door.name.clone(),
        };
        
        let cup = tiled_map.layers.get("cup").unwrap().objects.first().unwrap();    
        let trophy: GameObject = GameObject {
            world_x: cup.world_x,
            world_y: cup.world_y,
            name: cup.name.clone()
            
        };

        Game {
            world,
            player,
            diamonds,
            door,
            trophy,
            game_won: false,
            tiled_map,
        }
    }
}

impl Scene for Game {
    fn update(&mut self) {
        let resources = storage::get::<Resources>();

        let pos = self.world.actor_pos(self.player.collider);

        // Check for collision between player and diamonds
        for diamond in self.diamonds.iter_mut() {
            let diamond_rect = Rect::new(
                diamond.world_x,
                diamond.world_y - 32.0,
                32.0,
                32.0,
            );

            if self.player.overlaps(pos, &diamond_rect) {
                diamond.collected = true;
                play_sound_once(&resources.sound_collect);
            }
        }

        self.diamonds.retain(|diamond| !diamond.collected);

        let pos = self.world.actor_pos(self.player.collider);

        // Check for collision between player and cup
        if !self.game_won && self.player.overlaps(pos, &Rect::new(
            self.trophy.world_x,
            self.trophy.world_y - 32.0,
            32.0,
            32.0,
        )) {
            self.game_won = true;
            play_sound_once(&resources.sound_cup);
        }

        // Check for collision between player and door
        if self.game_won && self.player.overlaps(pos, &Rect::new(
            self.door.world_x,
            self.door.world_y - 32.0,
            32.0,
            32.0,
        )) {
            self.game_won = false;
            play_sound_once(&resources.sound_win);
        }
        
        self.player.update(get_frame_time(), &mut self.world, &self.tiled_map);
    }

    fn draw(&self) {
        self.tiled_map
            .draw_tiles("platform", Rect::new(0.0, 0.0, 608.0, 320.0), None);

        for diamond in &self.diamonds {
            let x = if diamond.name == "ruby" {
                0.0
            } else if diamond.name == "diamond" {
                32.0
            } else {
                64.0
            };

            self.tiled_map.spr_ex(
                "collectibles",
                Rect::new(
                    x,
                    0.0,
                    32.0,
                    32.0,
                ),
                Rect::new(
                    diamond.world_x,
                    diamond.world_y - 32.0,
                    32.0,
                    32.0,
                ),
            );
        }

        self.tiled_map.spr_ex(
            "door",
            Rect::new(
                0.0,
                0.0,
                32.0,
                32.0,
            ),
            Rect::new(
                self.door.world_x,
                self.door.world_y - 32.0,
                32.0,
                32.0,
            ),
        );

        if !self.game_won {
            self.tiled_map.spr_ex(
                "cup",
                Rect::new(
                    0.0,
                    0.0,
                    32.0,
                    32.0,
                ),
                Rect::new(
                    self.trophy.world_x,
                    self.trophy.world_y - 32.0,
                    32.0,
                    32.0,
                ),
            );
        }
    }
}