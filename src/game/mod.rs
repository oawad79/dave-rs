use std::vec;

use animations::Animations;
use camera::GameCamera;
use collectibles::CollectibleType;
use collision::CollisionManager;
use game_state::GameState;
use initialization::map_width_height;
use macroquad::{
    audio::{
        play_sound_once,
        stop_sound,
    },
    prelude::{
        collections::storage,
        *,
    },
};
use macroquad_particles::{
    AtlasConfig,
    Emitter,
    EmitterConfig,
};
use macroquad_platformer::{
    Tile,
    World,
};
use macroquad_tiled::Map;

use crate::{
    Scene,
    SceneChange,
    monster::Monster,
    player::Player,
    resources::Resources,
    score_board::{
        GameObject,
        ScoreBoard,
    },
};

mod animations;
mod camera;
mod collectibles;
mod collision;
mod game_state;
mod initialization;
mod renderer;

static EXPLOSION_DURATION: f32 = 2.0;

pub struct GameWorld {
    pub world: World,
    pub height_tiles: i32,
    pub width_tiles: i32,
}

pub struct Game {
    game_world: GameWorld,
    game_state: GameState,
    game_camera: GameCamera,
    animations: Animations,
    player: Player,
    collectibles: Vec<GameObject>,
    door: GameObject,
    score_board: ScoreBoard,
    explosions: Vec<(Emitter, Vec2)>,
    gun: Option<GameObject>,
    monsters: Vec<Monster>,
    jetpack: Option<GameObject>,
    warp_zone_rect: Option<Rect>,
}

impl Game {
    pub fn new(level: u32, retry: bool, cheat: bool, is_warp_zone: bool) -> Self {
        let resources = storage::get::<Resources>();

        let tiled_map = initialization::load_map_data(&resources, level, is_warp_zone);

        storage::store(tiled_map);

        let tiled_map = storage::get::<Map>();

        let (height, width) = map_width_height(&tiled_map);

        let (world, actor) = initialization::create_world(width as i32, &tiled_map);

        let mut score_board = if (cheat || level == 1) && !retry {
            ScoreBoard::new()
        } else {
            storage::get::<ScoreBoard>().clone()
        };

        if cheat {
            score_board.level = level;
        }

        let player = Player::new(
            actor,
            score_board.gun_captured,
            score_board.jetpack_captured,
            initialization::should_attach_player(&tiled_map),
        );

        let collectibles =
            initialization::load_objects_in_layer(retry, &score_board, &tiled_map, "collectibles");

        let gun = if tiled_map.contains_layer("gun") {
            let gun_object = tiled_map
                .layers
                .get("gun")
                .unwrap()
                .objects
                .first()
                .unwrap();
            Some(GameObject {
                world_x: gun_object.world_x,
                world_y: gun_object.world_y,
                width: gun_object.world_w,
                height: gun_object.world_h,
                name: gun_object.name.clone(),
                collected: None,
                progress: 0.0,
            })
        } else {
            None
        };
        // let gun = if tiled_map.contains_layer("gun") {
        //     initialization::load_objects_in_layer(
        //         retry, &score_board, &tiled_map, "gun").first().cloned()
        // }
        // else {
        //     None
        // };

        // let jetpack = if tiled_map.contains_layer("jetpack") {
        //     initialization::load_objects_in_layer(
        //         retry, &score_board, &tiled_map, "jetpack").first().cloned()
        // }
        // else {
        //     None
        // };

        let jetpack = if tiled_map.contains_layer("jetpack") {
            let jetpack_object = tiled_map
                .layers
                .get("jetpack")
                .unwrap()
                .objects
                .first()
                .unwrap();
            Some(GameObject {
                world_x: jetpack_object.world_x,
                world_y: jetpack_object.world_y,
                width: jetpack_object.world_w,
                height: jetpack_object.world_h,
                name: jetpack_object.name.clone(),
                collected: if score_board.jetpack_captured {
                    Some(true)
                } else {
                    None
                },
                progress: 0.0,
            })
        } else {
            None
        };

        let monsters: Vec<Monster> = if retry {
            score_board.monsters.clone()
        } else if tiled_map.contains_layer("monsters") {
            Monster::load_monsters(&tiled_map)
        } else {
            vec![]
        };

        let warp_zone_rect = initialization::load_collision_zone_in_layer(&tiled_map, "warp_zone");

        let game_world = GameWorld {
            world,
            height_tiles: height as i32,
            width_tiles: width as i32,
        };

        Self {
            game_world,
            game_state: GameState::new(&tiled_map, cheat, is_warp_zone),
            game_camera: GameCamera::new(),
            animations: Animations::load_deadly_objects(&tiled_map),
            player,
            collectibles,
            door: initialization::load_object_in_layer(&tiled_map, "door").unwrap(),
            score_board,
            explosions: vec![],
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
        self.game_camera.set_active();

        if tiled_map.contains_layer("night") {
            tiled_map.draw_tiles(
                "night",
                Rect::new(
                    0.0,
                    0.0,
                    (self.game_world.width_tiles * 32) as f32,
                    (self.game_world.height_tiles * 32) as f32,
                ),
                None,
            );
        }

        let pos = self.game_world.world.actor_pos(self.player.collider);

        //Update camera position to follow the player
        self.game_camera
            .update(pos, self.score_board.level, self.game_world.width_tiles);

        // Update scoreboard position based on camera
        self.score_board.position = self.game_camera.get_score_board_position(pos.y);

        //handle the player falling out of the game so we bring him from top
        if pos.y > screen_height() && !self.player.is_dead {
            self.game_world
                .world
                .set_actor_position(self.player.collider, vec2(pos.x, 0.0));
        }

        CollisionManager::handle_collecting_valuables(
            &mut self.collectibles,
            pos,
            &mut self.score_board,
            &resources,
        );

        self.collectibles
            .retain(|jewellery| !jewellery.collected.unwrap_or(false));

        if CollisionManager::check_warp_zone_collision(&self.warp_zone_rect, pos) {
            self.score_board.jetpack_captured = false;
            storage::store(self.score_board.clone());
            return Some(SceneChange::WarpZone);
        }

        if CollisionManager::check_gun_collision(
            &mut self.player,
            &self.gun,
            &mut self.score_board,
            pos,
        ) {
            play_sound_once(resources.get_sound("gotspecial"));
        }

        if CollisionManager::check_jetpack_collision(
            &mut self.player,
            &mut self.jetpack,
            &mut self.score_board,
            pos,
        ) {
            play_sound_once(resources.get_sound("jetPackActivated"));
        }

        if CollisionManager::check_door_collision(
            &self.door,
            &mut self.score_board,
            self.game_state.is_warp_zone,
            pos,
        ) {
            storage::store(self.score_board.clone());
            play_sound_once(resources.get_sound("win"));
            stop_sound(resources.get_sound("jetPackActivated"));
            return Some(SceneChange::Separator);
        }

        self.explosions
            .retain(|(explosion, _)| explosion.config.emitting);

        CollisionManager::handle_collision_with_deadly(
            &self.animations.deadly_objects,
            &mut self.player,
            &mut self.explosions,
            &mut self.game_state.player_explosion_active,
            &mut self.game_state.player_explosion_timer,
            &resources,
            pos,
        );

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
            return Some(SceneChange::Game {
                level: self.score_board.level,
                retry: !self.game_state.is_warp_zone,
                cheat: self.game_state.cheat,
                warp_zone: false,
            });
        }

        self.game_state.update();

        if tiled_map.contains_layer("tree_collider") {
            tiled_map.draw_tiles(
                "tree_collider",
                Rect::new(
                    0.0,
                    0.0,
                    (self.game_world.width_tiles * 32) as f32,
                    (self.game_world.height_tiles * 32) as f32,
                ),
                None,
            );
        }

        self.animations.update();

        self.player.update(&mut self.game_world.world);

        let screen_left = self.game_camera.camera.target.x - screen_width() / 2.0;
        let screen_right = self.game_camera.camera.target.x + screen_width() / 2.0;

        handle_monster_collisions(
            &mut self.monsters,
            &mut self.player,
            &mut self.score_board,
            &mut self.explosions,
            &mut self.game_state,
            &resources,
            &mut self.game_world.world,
            pos,
        );

        self.player.bullets.retain(|bullet| {
            if self
                .game_world
                .world
                .collide_solids(Vec2::new(bullet.x, bullet.y), 20, 10)
                == Tile::Solid
            {
                return false;
            }

            bullet.x < screen_right && bullet.x > screen_left && !bullet.collided
        });

        None
    }
    fn draw(&mut self) {
        let tiled_map = storage::get::<Map>();
        let resources = storage::get::<Resources>();

        self.score_board.draw();

        renderer::draw_tiles(
            &tiled_map,
            self.game_world.width_tiles,
            self.game_world.height_tiles,
        );
        renderer::draw_collectibles(&self.collectibles, &tiled_map);
        renderer::draw_door(&self.door, &tiled_map);

        for (explosion, coords) in &mut self.explosions {
            explosion.draw(vec2(coords.x, coords.y));
        }

        // if tiled_map.contains_layer("tree_collider") {
        //     tiled_map.draw_tiles(
        //         "tree_collider",
        //         Rect::new(0.0, 0.0, (self.game_world.width_tiles * 32) as f32,
        // (self.game_world.height_tiles * 32) as f32),         None,
        //     );
        // }

        renderer::draw_animations(&tiled_map, &self.animations);

        if let Some(g) = &self.gun {
            renderer::draw_gun(
                &tiled_map,
                g,
                &resources,
                self.player.has_gun,
                self.game_state.message_coord,
                self.game_camera.camera.target.x,
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
                self.game_camera.camera.target.x,
            );
        }

        renderer::draw_door_enable_banner(
            self.score_board.game_won,
            &resources,
            self.game_state.message_coord,
            self.game_camera.camera.target.x,
        );

        if tiled_map.contains_layer("fallthroughtiles") {
            tiled_map.draw_tiles(
                "fallthroughtiles",
                Rect::new(
                    0.0,
                    0.0,
                    (self.game_world.width_tiles * 32) as f32,
                    (self.game_world.height_tiles * 32) as f32,
                ),
                None,
            );
        }
    }
}
