use macroquad::{
    audio::{
        PlaySoundParams,
        play_sound,
        stop_sound,
    },
    camera::set_default_camera,
    color::Color,
    math::{
        Rect,
        Vec2,
        vec2,
    },
    prelude::collections::storage,
    text::{
        TextParams,
        draw_text_ex,
    },
    window::screen_height,
};
use macroquad_platformer::{
    Tile,
    World,
};
use macroquad_tiled::{
    Map,
    load_map,
};

use crate::{
    Scene,
    SceneChange,
    game::{
        camera::GameCamera,
        player::Player,
        score_board::ScoreBoard,
    },
    resources::Resources,
};
pub struct Separator {
    player: Player,
    score_board: ScoreBoard,
    world: World,
    sound_playing: bool,
    game_camera: GameCamera,
}

impl Separator {
    pub fn new() -> Self {
        let resources = storage::get::<Resources>();

        let tiled_map = load_map(
            &resources.separator_map_json,
            &[
                (
                    "images/mytileset.png",
                    resources.get_texture("mytileset").clone(),
                ),
                (
                    "images/dave_walk.png",
                    resources.get_texture("dave_walk").clone(),
                ),
                (
                    "images/dave_idle.png",
                    resources.get_texture("dave_idle").clone(),
                ),
                (
                    "images/dave_jump.png",
                    resources.get_texture("dave_jump").clone(),
                ),
                (
                    "images/collectibles.png",
                    resources.get_texture("collectibles").clone(),
                ),
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

        let player = Player::new(
            actor,
            score_board.gun_captured,
            score_board.jetpack_captured,
            false,
            1.0,
        );

        Self {
            player,
            score_board,
            world,
            sound_playing: false,
            game_camera: GameCamera::new(),
        }
    }

    fn update_camera_and_positions(&mut self, pos: Vec2) {
        // Set the camera to follow the player
        self.game_camera.set_active();

        // Handle the player falling out of the game so we bring him from top
        if pos.y > screen_height() && !self.player.is_dead {
            self.world
                .set_actor_position(self.player.collider, vec2(pos.x, 0.0));
        }
    }
}

impl Scene for Separator {
    fn update(&mut self) -> Option<SceneChange> {
        let pos = self.world.actor_pos(self.player.collider);
        let resources = storage::get::<Resources>();

        if !self.sound_playing {
            play_sound(
                resources.get_sound("hd-walk"),
                PlaySoundParams {
                    looped: true,
                    volume: 1.0,
                },
            );
            self.sound_playing = true;
        }

        self.player.simulate_right = true;
        self.player.update(&mut self.world);

        if pos.x > 608.0 {
            stop_sound(resources.get_sound("hd-walk"));
            storage::store(self.score_board.clone());

            if self.score_board.level == 10 {
                return Some(SceneChange::Complete);
            }
            return Some(SceneChange::Game {
                level: self.score_board.level,
                retry: false,
                cheat: false,
                warp_zone: false,
            });
        }

        None
    }

    fn draw(&mut self) {
        let tiled_map = storage::get::<Map>();
        let resources = storage::get::<Resources>();

        tiled_map.draw_tiles("seperator", Rect::new(0.0, 0.0, 608.0, 352.0), None);

        set_default_camera();
        self.score_board.draw();

        let pos = self.world.actor_pos(self.player.collider);
        self.update_camera_and_positions(pos);

        let m = if self.score_board.level < 10 {
            format!(
                "GOOD WORK! ONLY {} MORE TO GO!",
                (10 - self.score_board.level + 1)
            )
        } else {
            "  YES! YOU FINISHED THE GAME!".to_string()
        };

        draw_text_ex(
            m.as_str(),
            200.0,
            155.0,
            TextParams {
                font: Some(&resources.font),
                font_size: 16,
                color: Color {
                    r: 255.0,
                    g: 255.0,
                    b: 255.0,
                    a: 1.0,
                },
                ..Default::default()
            },
        );

        self.player.draw();
    }
}
