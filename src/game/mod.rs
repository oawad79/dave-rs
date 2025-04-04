use std::vec;

use animations::Animations;
use camera::GameCamera;
use collectibles::CollectibleType;
use collision::{
    Collidable,
    CollisionManager,
};
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
use macroquad_particles::Emitter;
use macroquad_platformer::World;
use macroquad_tiled::Map;
use monster::Monster;
use player::Player;
use score_board::{
    GameObject,
    ScoreBoard,
};

use crate::{
    Scene,
    SceneChange,
    resources::Resources,
};

mod animations;
mod bullet;
pub mod camera;
mod collectibles;
mod collision;
mod game_state;
mod initialization;
mod monster;
pub mod player;
mod renderer;
pub mod score_board;

static EXPLOSION_DURATION: f32 = 2.0;

pub struct GameWorld {
    pub world: World,
    pub height_tiles: i32,
    pub width_tiles: i32,
}

#[allow(clippy::struct_field_names)]
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
            if retry {
                score_board.jetpack_timer
            } else {
                0.0
            },
        );

        let collectibles =
            initialization::load_objects_in_layer(retry, &score_board, &tiled_map, "collectibles");

        let gun = initialization::load_gun_in_layer(&tiled_map);

        let jetpack = initialization::load_jetpack(&score_board, &tiled_map);

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

    fn persist_state(&mut self) {
        self.score_board.collectibles = self.collectibles.clone();
        self.score_board.monsters = self.monsters.clone();
        self.score_board.jetpack_captured = self.player.has_jetpack;
        self.score_board.jetpack_timer = self.player.jetpack_timer;
    }

    fn update_camera_and_positions(&mut self, pos: Vec2) {
        // Set the camera to follow the player
        self.game_camera.set_active();

        // Update camera position to follow the player
        self.game_camera
            .update(pos, self.score_board.level, self.game_world.width_tiles);

        // Handle the player falling out of the game so we bring him from top
        if pos.y > screen_height() && !self.player.is_dead {
            self.game_world
                .world
                .set_actor_position(self.player.collider, vec2(pos.x, 0.0));
        }
    }

    fn handle_collectibles(&mut self, pos: Vec2) {
        let resources = storage::get::<Resources>();

        CollisionManager::handle_collecting_valuables(
            &mut self.collectibles,
            pos,
            &mut self.score_board,
            &resources,
        );

        self.collectibles
            .retain(|jewellery| !jewellery.collected.unwrap_or(false));
    }

    fn check_warp_zone(&mut self, pos: Vec2) -> Option<SceneChange> {
        if CollisionManager::check_warp_zone_collision(self.warp_zone_rect.as_ref(), pos) {
            self.score_board.jetpack_captured = false;
            storage::store(self.score_board.clone());
            return Some(SceneChange::WarpZone);
        }
        None
    }

    fn handle_power_ups(&mut self, pos: Vec2) {
        let resources = storage::get::<Resources>();

        if CollisionManager::check_gun_collision(
            &mut self.player,
            self.gun.as_ref(),
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
    }

    fn check_door_collision(&mut self, pos: Vec2) -> Option<SceneChange> {
        let resources = storage::get::<Resources>();

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
        None
    }

    fn update_explosions(&mut self) {
        self.explosions
            .retain(|(explosion, _)| explosion.config.emitting);
    }

    fn handle_player_collisions(&mut self, pos: Vec2) {
        let resources = storage::get::<Resources>();

        CollisionManager::handle_collision_with_deadly(
            &self.animations.deadly_objects,
            &mut self.player,
            &mut self.explosions,
            &mut self.game_state.player_explosion_active,
            &mut self.game_state.player_explosion_timer,
            &resources,
            pos,
        );
    }

    fn check_player_death(&mut self) -> Option<SceneChange> {
        let resources = storage::get::<Resources>();

        if !self.game_state.player_explosion_active && self.player.is_dead {
            if self.score_board.lives == 0 {
                play_sound_once(resources.get_sound("gameoverman"));
                return Some(SceneChange::EntryScreen);
            }

            self.score_board.lives -= 1;
            if !self.game_state.is_warp_zone {
                self.persist_state();
            }

            storage::store(self.score_board.clone());
            return Some(SceneChange::Game {
                level: self.score_board.level,
                retry: !self.game_state.is_warp_zone,
                cheat: self.game_state.cheat,
                warp_zone: false,
            });
        }
        None
    }

    fn update_game_entities(&mut self, pos: Vec2) {
        self.game_state.update();
        self.animations.update();
        self.player.update(&mut self.game_world.world);

        for monster in &mut self.monsters {
            if monster.is_alive() {
                monster.update(pos);
            }
        }
    }

    fn handle_monster_collisions(&mut self) {
        let resources = storage::get::<Resources>();

        CollisionManager::handle_collisions(
            &mut self.monsters,
            &mut self.player,
            &mut self.score_board,
            &mut self.explosions,
            &mut self.game_state,
            &resources,
        );
    }

    fn update_bullets(&mut self) {
        let pos = self.game_world.world.actor_pos(self.player.collider);
        let resources = storage::get::<Resources>();

        self.player.bullets.retain(|bullet| {
            Player::should_retain_bullet(&self.game_world, &self.game_camera, bullet)
        });

        for monster in &mut self.monsters {
            monster
                .bullets
                .retain(|bullet| Monster::should_retain_bullet(&self.game_world, pos, bullet));

            for bullet in &mut monster.bullets {
                CollisionManager::check_monster_bullet_hit(
                    &resources,
                    pos,
                    bullet,
                    &mut self.player,
                    &mut self.explosions,
                    &mut self.game_state,
                );
            }
        }
    }
}

impl Scene for Game {
    fn update(&mut self) -> Option<SceneChange> {
        let pos = self.game_world.world.actor_pos(self.player.collider);

        // Update camera and positions
        self.update_camera_and_positions(pos);

        // Handle collectibles
        self.handle_collectibles(pos);

        // Check warp zone collision
        if let Some(scene_change) = self.check_warp_zone(pos) {
            return Some(scene_change);
        }

        // Handle power-ups (gun and jetpack)
        self.handle_power_ups(pos);

        // Check door collision
        if let Some(scene_change) = self.check_door_collision(pos) {
            return Some(scene_change);
        }

        // Update explosions
        self.update_explosions();

        // Handle player collisions with deadly objects
        self.handle_player_collisions(pos);

        // Check player death
        if let Some(scene_change) = self.check_player_death() {
            return Some(scene_change);
        }

        // Update game entities
        self.update_game_entities(pos);

        // Handle monster collisions
        self.handle_monster_collisions();

        // Update bullets
        self.update_bullets();

        None
    }

    fn draw(&mut self) {
        let tiled_map = storage::get::<Map>();
        let resources = storage::get::<Resources>();

        set_default_camera();
        self.score_board.draw();

        let pos = self.game_world.world.actor_pos(self.player.collider);
        self.update_camera_and_positions(pos);

        renderer::draw_tiles(
            &tiled_map,
            self.game_world.width_tiles,
            self.game_world.height_tiles,
        );

        renderer::draw_layer_if_exists(&tiled_map, &self.game_world, "night");

        for monster in &self.monsters {
            if monster.is_alive() {
                monster.draw();
            }
        }

        renderer::draw_collectibles(&self.collectibles, &tiled_map);
        renderer::draw_door(&self.door, &tiled_map);

        for (explosion, coords) in &mut self.explosions {
            explosion.draw(vec2(coords.x, coords.y));
        }

        renderer::draw_layer_if_exists(&tiled_map, &self.game_world, "tree_collider");

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
                self.player.jetpack_progress,
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

        renderer::draw_layer_if_exists(&tiled_map, &self.game_world, "fallthroughtiles");

        self.player.draw();
    }
}
