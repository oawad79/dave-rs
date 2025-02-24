use macroquad::{audio::play_sound_once, math::{vec2, Rect}, prelude::collections::storage};
use macroquad_platformer::{Tile, World};
use macroquad_tiled::{load_map, Map};

use crate::{player::Player, resources::Resources, Scene, SceneChange, score_board::ScoreBoard};


struct GameObject {
    world_x: f32,
    world_y: f32,
    name: String,
    collected: Option<bool>,
}

pub struct Game {
    world: World,
    player: Player,
    collectibles: Vec<GameObject>,
    door: GameObject,
    trophy: GameObject,
    game_won: bool,
    score_board: ScoreBoard,
    height_tiles: i32,
    width_tiles: i32,
}

impl Game {
    pub fn new(level: i32) -> Game {
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
                ("cup.png", resources.cup.clone()),    
                ("deadly.png", resources.deadly_grass_texture.clone()),     
                ("fire1-sheet.png", resources.fire1.clone())
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
    
        let actor = world.add_actor(vec2(70.0, 250.0), 32, 32);
    
        let player = Player::new(actor);

        let objects_layer = tiled_map.layers.get("collectibles").unwrap();
        let collectibles = objects_layer
            .objects
            .iter()
            .map(|entry| 
                GameObject {
                    world_x: entry.world_x,
                    world_y: entry.world_y,
                    name: entry.name.clone(),
                    collected: None,
                }
            ).collect::<Vec<GameObject>>();
        
        let door = tiled_map.layers.get("door").unwrap().objects.first().unwrap();

        let door = GameObject {
            world_x: door.world_x,
            world_y: door.world_y,
            name: door.name.clone(),
            collected: None,
        };
        
        let cup = tiled_map.layers.get("cup").unwrap().objects.first().unwrap();    
        let trophy: GameObject = GameObject {
            world_x: cup.world_x,
            world_y: cup.world_y,
            name: cup.name.clone(),
            collected: None,
        };

        Game {
            world,
            player,
            collectibles,
            door,
            trophy,
            game_won: false,
            score_board: ScoreBoard::new(),
            height_tiles: height as i32,
            width_tiles: width as i32,
        }
    }
}

impl Scene for Game {
    fn update(&mut self) -> Option<SceneChange> {
        let resources = storage::get::<Resources>();

        let pos = self.world.actor_pos(self.player.collider);

        // Check for collision between player and diamonds
        for diamond in self.collectibles.iter_mut() {
            let diamond_rect = Rect::new(
                diamond.world_x,
                diamond.world_y - 32.0,
                32.0,
                32.0,
            );

            if self.player.overlaps(pos, &diamond_rect) {
                self.score_board.score += 10;
                diamond.collected = Option::Some(true);
                play_sound_once(&resources.sound_collect);
            }
        }

        self.collectibles.retain(|diamond| !diamond.collected.unwrap_or(false));

        // Check for collision between player and cup
        if !self.game_won && self.player.overlaps(pos, &Rect::new(
            self.trophy.world_x,
            self.trophy.world_y - 32.0,
            32.0,
            32.0,
        )) {
            self.score_board.score += 100;
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
            self.score_board.level += 1;
            storage::store(self.score_board.clone());
            return Some(SceneChange::Separator);
        }
        
        self.player.update(&mut self.world);
        
        None
    }

    fn draw(&self) {
        let tiled_map = storage::get::<Map>();

        self.score_board.draw();

        tiled_map
            .draw_tiles("platform", 
                    Rect::new(0.0, 0.0, (self.width_tiles * 32) as f32, (self.height_tiles * 32) as f32), None);


        for diamond in &self.collectibles {
            let x = if diamond.name == "ruby" {
                0.0
            } else if diamond.name == "diamond" {
                32.0
            } else {
                64.0
            };

            tiled_map.spr_ex(
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

        tiled_map.spr_ex(
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
            tiled_map.spr_ex(
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