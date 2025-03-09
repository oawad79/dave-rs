use macroquad::{audio::{play_sound, stop_sound, PlaySoundParams}, color::Color, math::{vec2, Rect}, prelude::collections::storage, text::{draw_text_ex, TextParams}};
use macroquad_platformer::{Tile, World};
use macroquad_tiled::{load_map, Map};

use crate::{player::Player, resources::Resources, score_board::ScoreBoard, Scene, SceneChange};

pub struct Separator {
    player: Player,
    score_board: ScoreBoard,
    world: World,
    sound_playing: bool,
}

impl Separator {
    pub fn new() -> Separator {
        let resources = storage::get::<Resources>();

        let tiled_map = load_map(
            &resources.separator_map_json,
            &[
                ("images/mytileset.png", resources.get_texture("mytileset").clone()),
                ("images/dave_walk.png", resources.get_texture("dave_walk").clone()),
                ("images/dave_idle.png", resources.get_texture("dave_idle").clone()),
                ("images/dave_jump.png", resources.get_texture("dave_jump").clone()),
                ("images/collectibles.png", resources.get_texture("collectibles").clone()),
                ("images/door.png", resources.get_texture("door").clone()),
            ],
            &[],
        )
        .unwrap();

        storage::store(tiled_map);

        let tiled_map = storage::get::<Map>();

        let mut static_colliders = vec![];
        for (_x, _y, tile) in tiled_map.tiles("seperator", None) {
            static_colliders.push(if tile.is_some() {
                Tile::Solid
            } else {
                Tile::Empty
            });
        }

        let mut world = World::new();
        world.add_static_tiled_layer(static_colliders, 32., 32., 19, 1);
    
        let actor = world.add_actor(vec2(50.0, 192.0), 32, 32);
    
        let score_board = storage::get::<ScoreBoard>().clone();

        let player = Player::new(actor, score_board.gun_captured);

        Separator {
            player,
            score_board,
            world,
            sound_playing: false,
        }
    }
}

impl Scene for Separator {
    fn update(&mut self) -> Option<SceneChange> {
        let pos = self.world.actor_pos(self.player.collider);
        let resources = storage::get::<Resources>();

        if !self.sound_playing {
            play_sound(resources.get_sound("hd-walk"), PlaySoundParams {
                looped: true, 
                volume: 1.0
            });
            self.sound_playing = true;
        }

        self.player.simulate_right = true;
        self.player.update(&mut self.world);

        if pos.x > 608.0 {
            stop_sound(resources.get_sound("hd-walk"));
            storage::store(self.score_board.clone());
            return Some(SceneChange::Game { level: self.score_board.level, retry: false, cheat: false });
        }

        self.score_board.position = (5.0, 5.0);

        None
    }

    fn draw(&self) {
        let tiled_map = storage::get::<Map>();
        let resources = storage::get::<Resources>();

        tiled_map
            .draw_tiles("seperator", Rect::new(0.0, 0.0, 608.0, 352.0), None);
        
        
        self.score_board.draw();

        draw_text_ex(
            format!("GOOD WORK! ONLY {} MORE TO GO!", (10 - self.score_board.level + 1)).as_str(),
            200.0,
            155.0,
            TextParams {
                font: Some(&resources.font), 
                font_size: 16,     
                color: Color{r: 255.0, g: 255.0, b: 255.0, a: 1.0},  
                ..Default::default()
            },
        );
    }
}