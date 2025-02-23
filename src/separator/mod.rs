use macroquad::{math::{vec2, Rect}, prelude::collections::storage};
use macroquad_platformer::{Tile, World};
use macroquad_tiled::{load_map, Map};

use crate::{player::Player, resources::Resources, score_board::{self, ScoreBoard}, Scene, SceneChange};

pub struct Separator {
    player: Player,
    score_board: ScoreBoard,
}

impl Separator {
    pub fn new() -> Separator {
        let resources = storage::get::<Resources>();

        let tiled_map = load_map(
            &resources.separator_map_json,
            &[
                ("mytileset.png", resources.tileset.clone()),
                ("dave_walk.png", resources.player_walk.clone()),
                ("collectibles.png", resources.collectibles.clone()),
                ("door.png", resources.door.clone()),
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
    
        let actor = world.add_actor(vec2(10.0, 250.0), 32, 32);
    
        let player = Player::new(actor);

        let score_board = storage::get::<ScoreBoard>().clone();

        Separator {
            player,
            score_board,
        }
    }
}

impl Scene for Separator {
    fn update(&mut self) -> Option<SceneChange> {
        None
    }

    fn draw(&self) {
        let tiled_map = storage::get::<Map>();

        tiled_map
            .draw_tiles("seperator", Rect::new(0.0, 0.0, 608.0, 352.0), None);

        self.score_board.draw();
    }
}