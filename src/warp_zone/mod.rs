use macroquad::{audio::{play_sound, stop_sound, PlaySoundParams}, color::Color, math::{vec2, Rect}, prelude::collections::storage, text::{draw_text_ex, TextParams}};
use macroquad_platformer::{Tile, World};
use macroquad_tiled::{load_map, Map};

use crate::{player::Player, resources::Resources, score_board::ScoreBoard, Scene, SceneChange};

pub struct WarpZone {
    player: Player,
    score_board: ScoreBoard,
    world: World,
    sound_playing: bool,
}

impl WarpZone {
    pub fn new() -> Self {
        let resources = storage::get::<Resources>();

        let tiled_map = load_map(
            &resources.warp_zone_separator_map_json,
            &[
                ("images/mytileset.png", resources.get_texture("mytileset").clone()),
                ("images/dave_idle.png", resources.get_texture("dave_idle").clone()),
                ("images/dave_walk.png", resources.get_texture("dave_walk").clone()),
                ("images/dave_jump.png", resources.get_texture("dave_jump").clone()),
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
    
        let player_loc = tiled_map.layers.get("player").unwrap().objects.first().unwrap();

        let actor = world.add_actor(vec2(player_loc.world_x, player_loc.world_y - 32.0), 32, 32);
    
        let score_board = storage::get::<ScoreBoard>().clone();

        let player = Player::new(actor, 
            score_board.gun_captured, score_board.jetpack_captured);

        Self {
            player,
            score_board,
            world,
            sound_playing: false,
        }
    }
}

impl Scene for WarpZone {
    fn update(&mut self) -> Option<SceneChange> {
        let pos = self.world.actor_pos(self.player.collider);
        let resources = storage::get::<Resources>();

        if !self.sound_playing {
            play_sound(resources.get_sound("fall"), PlaySoundParams {
                looped: true, 
                volume: 1.0
            });
            self.sound_playing = true;
        }

        //self.player.simulate_right = true;
        self.player.update(&mut self.world);

        if pos.y > 384.0 {
            stop_sound(resources.get_sound("fall"));
            storage::store(self.score_board.clone());
            
            if self.score_board.level == 10 {
                return Some(SceneChange::Complete);
            }
            return Some(SceneChange::Game { level: self.score_board.level, retry: false, cheat: false, warp_zone: true });
        }

        self.score_board.position = (5.0, 5.0);

        None
    }

    fn draw(&self) {
        let tiled_map = storage::get::<Map>();
        let resources = storage::get::<Resources>();

        tiled_map
            .draw_tiles("platform", Rect::new(0.0, 0.0, 608.0, 352.0), None);
        
        self.score_board.draw();

        draw_text_ex(
            "WARP",
            60.0,
            142.0,
            TextParams {
                font: Some(&resources.font), 
                font_size: 40,     
                color: Color{r: 255.0, g: 255.0, b: 255.0, a: 1.0},  
                ..Default::default()
            },
        );

        draw_text_ex(
            "ZONE",
            410.0,
            144.0,
            TextParams {
                font: Some(&resources.font), 
                font_size: 40,     
                color: Color{r: 255.0, g: 255.0, b: 255.0, a: 1.0},  
                ..Default::default()
            },
        );
    }
}