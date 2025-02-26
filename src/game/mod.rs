use macroquad::{audio::play_sound_once, math::{vec2, Rect}, prelude::{animation::{AnimatedSprite, Animation}, collections::storage, set_camera, Camera2D}, window::screen_width};
use macroquad_platformer::{Tile, World};
use macroquad_tiled::{load_map, Map, Object};

use crate::{player::Player, resources::Resources, score_board::ScoreBoard, Scene, SceneChange};


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
    animated_fire: Option<AnimatedSprite>,
    animated_water: Option<AnimatedSprite>,
    fires: Vec<Object>,
    waters: Vec<Object>,
    camera: Camera2D,
    animated_grass: Option<AnimatedSprite>,
    grasses: Vec<Object>,
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
                ("fire1-sheet.png", resources.fire1.clone()),
                ("water1-sheet.png", resources.water_texture.clone()),
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

        let (animated_fire, fires) = 
                            Game::load_animation(&tiled_map, "fire", 3);
        let (animated_water, waters) = 
                            Game::load_animation(&tiled_map, "water", 5);

        let (animated_grass, grasses) = 
                            Game::load_animation(&tiled_map, "grass", 4);
        
        let camera = Camera2D::from_display_rect(Rect::new(0.0, 352.0, 608.0, -352.0));

        Game {
            world,
            player,
            collectibles,
            door,
            trophy,
            game_won: false,
            score_board: if level == 1 { ScoreBoard::new()} else { storage::get::<ScoreBoard>().clone() },
            height_tiles: height as i32,
            width_tiles: width as i32,
            animated_fire,
            fires,
            animated_water,
            waters,
            camera,
            animated_grass,
            grasses,
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
                _ => 64.0,
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

    fn draw_trophy(&self, tiled_map: &Map) {
        if !self.game_won {
            tiled_map.spr_ex(
                "cup",
                Rect::new(0.0, 0.0, 32.0, 32.0),
                Rect::new(self.trophy.world_x, self.trophy.world_y - 32.0, 32.0, 32.0),
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
}

impl Scene for Game {
    fn update(&mut self) -> Option<SceneChange> {
        let resources = storage::get::<Resources>();

        // Set the camera to follow the player
        set_camera(&self.camera);

        let pos = self.world.actor_pos(self.player.collider);

        // Update camera position to follow the player
        if (self.score_board.level != 1) && 
           (pos.x > screen_width() / 2.0) && 
           (pos.x < (self.width_tiles * 32) as f32 - screen_width() / 3.0) {
            self.camera.target.x = pos.x;
            self.score_board.position = (pos.x - 300.0, pos.y);
        }
        

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

        if self.animated_fire.is_some() {
            self.animated_fire.as_mut().unwrap().update();
        }

        if self.animated_water.is_some() {
            self.animated_water.as_mut().unwrap().update();
        }

        if self.animated_grass.is_some() {
            self.animated_grass.as_mut().unwrap().update();
        }
        
        None
    }

    fn draw(&self) {
        let tiled_map = storage::get::<Map>();

        self.score_board.draw();
        self.draw_tiles(&tiled_map);
        self.draw_collectibles(&tiled_map);
        self.draw_door(&tiled_map);
        self.draw_trophy(&tiled_map);
        self.draw_animated_objects(&tiled_map);
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