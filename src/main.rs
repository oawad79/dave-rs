mod player;
mod resources;

use macroquad::audio::play_sound;
use macroquad::audio::play_sound_once;
use macroquad::audio::stop_sound;
use macroquad::audio::AudioContext;
use macroquad::audio::PlaySoundParams;
use player::animated_player;
use player::Player;

use resources::Resources;

use macroquad::prelude::*;
use macroquad_platformer::*;

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

#[macroquad::main("Dave")]
async fn main() {
    let resources = Resources::load().await.unwrap();

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
    
    let objects_layer = resources.tiled_map.layers.get("diamonds").unwrap();
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

    let mut player = Player::new(world.add_actor(vec2(60.0, 250.0), 32, 32));

    let mut animated_player = animated_player();

    let camera = Camera2D::from_display_rect(Rect::new(0.0, 320.0, 608.0, -320.0));

    loop {
        clear_background(BLACK);

        set_camera(&camera);

        resources
            .tiled_map
            .draw_tiles("platform", Rect::new(0.0, 0.0, 608.0, 320.0), None);

        for diamond in &diamonds {
            resources.tiled_map.spr_ex(
                "diamond",
                Rect::new(
                    0.0,
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

        let pos = world.actor_pos(player.collider);

        // Check for collision between player and diamonds
        for diamond in diamonds.iter_mut() {
            let diamond_rect = Rect::new(
                diamond.world_x,
                diamond.world_y - 32.0,
                32.0,
                32.0,
            );

            let player_rect = Rect::new(
                pos.x,
                pos.y,
                32.0,
                32.0,
            );

            if player_rect.overlaps(&diamond_rect) {
                diamond.collected = true;
                play_sound_once(&resources.sound_collect);
            }
        }

        diamonds.retain(|diamond| !diamond.collected);
        

        let on_ground = world.collide_check(player.collider, pos + vec2(0., 1.));
        
        // Draw player
        let state: &str;
        let flip: f32;

        if player.speed.x != 0.0 {
            state = if !on_ground {
                animated_player.set_animation(2); // jump
                "dave_jump"
            } else {
                animated_player.set_animation(0); // walk
                "dave_walk"
            };

            if player.speed.x < 0.0 {
                player.facing_left = true;
                flip = -32.0;
            } else {
                player.facing_left = false;
                flip = 32.0;
            }
        } else {
            state = "dave_idle";
            animated_player.set_animation(1); // idle
            flip = if player.facing_left { -32.0 } else { 32.0 };
        }

        resources.tiled_map.spr_ex(
            state,
            animated_player.frame().source_rect,
            Rect::new(
                pos.x + if flip < 0.0 { 32.0 } else { 0.0 },
                pos.y,
                flip,
                32.0,
            ),
        );

        animated_player.update();

        // player movement control
        if !on_ground {
            player.speed.y += 500. * get_frame_time();
        }

        if is_key_down(KeyCode::Right) {
            player.speed.x = 100.0;
            
            //check if sound is Playing
            // play_sound(&resources.sound_walk, PlaySoundParams {
            //     looped: false,
            //     volume: 0.2,
            // });
        
        } else if is_key_down(KeyCode::Left) {
            player.speed.x = -100.0;
            // play_sound(&resources.sound_walk, PlaySoundParams {
            //     looped: false,
            //     volume: 0.2,
            // });
        } else {
            player.speed.x = 0.;
            //stop_sound(&resources.sound_walk);
        }

        if is_key_pressed(KeyCode::Space) && on_ground {
            play_sound_once(&resources.sound_jump);
            player.speed.y = -260.;
        }

        //add mouse click to get location
        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos = mouse_position();
            println!("Mouse position: {:?}", mouse_pos);
        }

        world.move_h(player.collider, player.speed.x * get_frame_time());
        world.move_v(player.collider, player.speed.y * get_frame_time());

        next_frame().await
    }
}
