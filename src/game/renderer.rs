use macroquad::prelude::{
    animation::AnimatedSprite,
    *,
};
use macroquad_tiled::{
    Map,
    Object,
};

use super::animations::Animations;
use crate::{
    game::{
        CollectibleType,
        score_board::GameObject,
    },
    resources::Resources,
};

/// Draws all collectible items on the map
pub fn draw_collectibles(collectibles: &[GameObject], tiled_map: &Map) {
    for diamond in collectibles {
        let offset = CollectibleType::from(diamond.name.as_str()).data().offset;

        tiled_map.spr_ex(
            "collectibles",
            Rect::new(offset, 0.0, 32.0, 32.0),
            Rect::new(diamond.world_x, diamond.world_y - 32.0, 32.0, 32.0),
        );
    }
}

/// Draws the door on the map
pub fn draw_door(door: &GameObject, tiled_map: &Map) {
    tiled_map.spr_ex(
        "door",
        Rect::new(0.0, 0.0, 32.0, 32.0),
        Rect::new(door.world_x, door.world_y - 32.0, 32.0, 32.0),
    );
}

/// Draws the gun pickup and icon
pub fn draw_gun(
    tiled_map: &Map,
    gun: &GameObject,
    resources: &Resources,
    player_has_gun: bool,
    message_coord: (f32, f32),
    camera_target_x: f32,
) {
    if !player_has_gun {
        tiled_map.spr_ex(
            "gun_icon",
            Rect::new(0.0, 0.0, 32.0, 32.0),
            Rect::new(gun.world_x, gun.world_y - 32.0, 32.0, 32.0),
        );
    } else {
        draw_texture_ex(
            resources.get_texture("gun"),
            message_coord.0 + camera_target_x + 50.0,
            message_coord.1 - 32.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    resources.get_texture("gun").width() * 0.7,
                    resources.get_texture("gun").height() * 0.7,
                )),
                ..Default::default()
            },
        );

        draw_texture_ex(
            resources.get_texture("gun_icon"),
            message_coord.0 + camera_target_x + 110.0,
            message_coord.1 - 32.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    resources.get_texture("gun_icon").width(),
                    resources.get_texture("gun_icon").height(),
                )),
                ..Default::default()
            },
        );
    }
}

/// Draws the jetpack pickup and icon/progress bar
pub fn draw_jetpack(
    tiled_map: &Map,
    jetpack: &GameObject,
    resources: &Resources,
    player_has_jetpack: bool,
    player_progress: f32,
    message_coord: (f32, f32),
    camera_target_x: f32,
) {
    if jetpack.collected.unwrap_or(false) && player_has_jetpack {
        draw_texture_ex(
            resources.get_texture("jetpack_over"),
            message_coord.0 + camera_target_x - 210.0,
            message_coord.1 - 32.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    resources.get_texture("jetpack_over").width() * 0.7,
                    resources.get_texture("jetpack_over").height() * 0.7,
                )),
                ..Default::default()
            },
        );

        let width = resources.get_texture("jetpack_progress").width() * 0.7;
        let height = resources.get_texture("jetpack_progress").height() * 0.7;

        let bar_width = width * player_progress;

        // Define the texture cropping rectangle (shrink from right to left)
        let source_rect = Some(Rect::new(0.0, 0.0, bar_width, height));

        draw_texture_ex(
            resources.get_texture("jetpack_progress"),
            message_coord.0 + camera_target_x - 214.0,
            message_coord.1 - 36.0,
            WHITE,
            DrawTextureParams {
                source: source_rect,
                ..Default::default()
            },
        );

        draw_texture_ex(
            resources.get_texture("jetpack"),
            message_coord.0 + camera_target_x - 410.0,
            message_coord.1 - 32.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    resources.get_texture("jetpack").width() * 0.7,
                    resources.get_texture("jetpack").height() * 0.7,
                )),
                ..Default::default()
            },
        );
    }

    if !jetpack.collected.unwrap_or(false) {
        tiled_map.spr_ex(
            "jetpack2",
            Rect::new(0.0, 0.0, 32.0, 32.0),
            Rect::new(jetpack.world_x, jetpack.world_y - 32.0, 32.0, 32.0),
        );
    }
}

pub fn draw_animations(tiled_map: &Map, animations: &Animations) {
    for (sheet, (animated, objects)) in animations.deadly_objects.iter() {
        draw_animated(tiled_map, sheet, animated, objects);
    }
}

pub fn draw_animated(
    tiled_map: &Map,
    sheet: &str,
    animated: &Option<AnimatedSprite>,
    objects: &[Object],
) {
    if let Some(animated) = animated {
        for object in objects {
            tiled_map.spr_ex(
                sheet,
                animated.frame().source_rect,
                Rect::new(object.world_x, object.world_y - 32.0, 32.0, 32.0),
            );
        }
    }
}

/// Draws the base map tiles
pub fn draw_tiles(tiled_map: &Map, width_tiles: i32, height_tiles: i32) {
    tiled_map.draw_tiles(
        "platform",
        Rect::new(
            0.0,
            0.0,
            (width_tiles * 32) as f32,
            (height_tiles * 32) as f32,
        ),
        None,
    );
}

/// Draws the victory banner when the game is won
pub fn draw_door_enable_banner(
    game_won: bool,
    resources: &Resources,
    message_coord: (f32, f32),
    camera_target_x: f32,
) {
    if game_won {
        draw_texture_ex(
            resources.get_texture("door_enable_banner"),
            message_coord.0 + camera_target_x - 300.0,
            message_coord.1 - 14.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    resources.get_texture("door_enable_banner").width(),
                    resources.get_texture("door_enable_banner").height() * 0.5,
                )),
                ..Default::default()
            },
        );
    }
}
