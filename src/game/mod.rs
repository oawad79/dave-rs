use macroquad::{audio::play_sound_once, color::WHITE, math::{vec2, Rect}, prelude::collections::storage, text::draw_text, texture::{draw_texture_ex, DrawTextureParams}, time::get_frame_time};
use macroquad_platformer::{Tile, World};
use macroquad_tiled::{load_map, Map};

use crate::{player::Player, resources::Resources, Scene, SceneChange};

struct ScoreBoard {
    score: i32,
    lives: i32,
    level: i32,
}

impl ScoreBoard {
    fn new() -> ScoreBoard {
        ScoreBoard {
            score: 0,
            lives: 3,
            level: 1,
        }
    }

    fn number_to_vec(n: u32) -> Vec<u32> {
        n.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect()
    }
}

impl Scene for ScoreBoard {
    fn update(&mut self) -> Option<SceneChange> {
        None
    }

    fn draw(&self) {
        let resources = storage::get::<Resources>();

        draw_texture_ex(
            &resources.score_texture,
            5.0,
            5.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(resources.score_texture.width() , resources.score_texture.height() )), 
                ..Default::default()
            },
        );

        let score = ScoreBoard::number_to_vec(self.score as u32);
        for (i, n) in score.iter().enumerate() {
            draw_texture_ex(
                &resources.numbers[*n as usize],
                120.0 + (i as f32 * 20.0),
                7.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(resources.numbers[*n as usize].width() , resources.numbers[*n as usize].height() )), 
                    ..Default::default()
                },
            );
        }

        draw_texture_ex(
            &resources.level_texture,
            220.0,
            5.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(resources.level_texture.width(), resources.level_texture.height())), 
                ..Default::default()
            },
        );

        draw_texture_ex(
            &resources.numbers[self.level as usize],
            320.0,
            7.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(resources.numbers[self.level as usize].width(), resources.numbers[self.level as usize].height())), 
                ..Default::default()
            },
        );

        draw_texture_ex(
            &resources.daves_texture,
            400.0,
            5.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(resources.daves_texture.width(), resources.daves_texture.height())), 
                ..Default::default()
            },
        );

        for i in 0..self.lives {
            draw_texture_ex(
                &resources.dave_face,
                510.0 + (i as f32 * 30.0),
                2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(resources.dave_face.width(), resources.dave_face.height())), 
                    ..Default::default()
                },
            );
        }
    }
}


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
        
        let mut world = World::new();
        world.add_static_tiled_layer(static_colliders, 32., 32., 19, 1);
    
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

        let pos = self.world.actor_pos(self.player.collider);

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
            return Some(SceneChange::MainMenu);
        }
        
        self.player.update(&mut self.world);
        
        None
    }

    fn draw(&self) {
        let tiled_map = storage::get::<Map>();

        self.score_board.draw();

        tiled_map
            .draw_tiles("platform", Rect::new(0.0, 0.0, 608.0, 352.0), None);


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