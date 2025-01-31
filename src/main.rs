mod player;
mod resources;

use macroquad::{
    audio::play_sound_once, 
    prelude::collections::storage
};
use resources::Resources;

use macroquad_platformer::{Tile, World};
use player::Player;
use macroquad::prelude::*;

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

type Cup = Diamond;

fn window_conf() -> Conf {
    Conf {
        window_title: "Dave".to_owned(),
        fullscreen: false,
        platform: miniquad::conf::Platform {
            linux_backend: miniquad::conf::LinuxBackend::WaylandOnly,
            ..Default::default()
        },
        window_width: 1000,
        window_height: 650,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let _ = Resources::load().await;
    
    let resources = storage::get::<Resources>();
    
    let mut static_colliders = vec![];
    for (_x, _y, tile) in resources.tiled_map.tiles("platform", None) {
        static_colliders.push(if tile.is_some() {
            Tile::Solid
        } else {
            Tile::Empty
        });
    }

    let mut world = World::new();
    world.add_static_tiled_layer(static_colliders, 32., 32., 19, 1);

    let actor = world.add_actor(vec2(70.0, 250.0), 32, 32);

    let tiled_map = &resources.tiled_map;
    
    let mut player = Player::new(actor);

    let objects_layer = resources.tiled_map.layers.get("collectibles").unwrap();
    let mut diamonds:Vec<Diamond> = objects_layer
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
        )
        .collect::<Vec<Diamond>>();
    
    let door = resources.tiled_map.layers.get("door").unwrap().objects.first().unwrap();
    let cup = tiled_map.layers.get("cup").unwrap().objects.first().unwrap();    
    let mut trophy: Cup = Cup {
        world_x: cup.world_x,
        world_y: cup.world_y,
        world_w: cup.world_w,
        world_h: cup.world_h,
        tile_x: cup.tile_x,
        tile_y: cup.tile_y,
        tile_w: cup.tile_w,
        tile_h: cup.tile_h,
        name: cup.name.clone(),
        collected: false,
    };

    let camera = Camera2D::from_display_rect(Rect::new(0.0, 320.0, 608.0, -320.0));
    
    let mut game_won = false;
    loop {
        clear_background(BLACK);

        set_camera(&camera);

        let delta = get_frame_time();

        tiled_map
            .draw_tiles("platform", Rect::new(0.0, 0.0, 608.0, 320.0), None);

        for diamond in &diamonds {
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
                door.world_x,
                door.world_y - 32.0,
                32.0,
                32.0,
            ),
        );

        if !trophy.collected {
            tiled_map.spr_ex(
                "cup",
                Rect::new(
                    0.0,
                    0.0,
                    32.0,
                    32.0,
                ),
                Rect::new(
                    cup.world_x,
                    cup.world_y - 32.0,
                    32.0,
                    32.0,
                ),
            );
        }
        let pos = world.actor_pos(player.collider);

        // Check for collision between player and diamonds
        for diamond in diamonds.iter_mut() {
            let diamond_rect = Rect::new(
                diamond.world_x,
                diamond.world_y - 32.0,
                32.0,
                32.0,
            );

            if player.overlaps(pos, &diamond_rect) {
                diamond.collected = true;
                play_sound_once(&resources.sound_collect);
            }
        }

        diamonds.retain(|diamond| !diamond.collected);
        
        // Check for collision between player and cup
        if !trophy.collected && player.overlaps(pos, &Rect::new(
            cup.world_x,
            cup.world_y - 32.0,
            32.0,
            32.0,
        )) {
            trophy.collected = true;
            play_sound_once(&resources.sound_cup);
        }

        // Check for collision between player and door
        if !game_won && trophy.collected && player.overlaps(pos, &Rect::new(
            door.world_x,
            door.world_y - 32.0,
            32.0,
            32.0,
        )) {
            game_won = true;
            play_sound_once(&resources.sound_win);
        }
        
        player.update(delta, &mut world);

        next_frame().await
    }
}
