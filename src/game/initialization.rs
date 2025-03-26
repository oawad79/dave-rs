use macroquad_platformer::Tile;
use macroquad_tiled::{load_map, Map};

use crate::{resources::Resources, score_board::{GameObject, ScoreBoard}};

pub fn should_attach_player(tiled_map: &Map) -> bool {
    tiled_map.layers.get("player").unwrap().objects.first().unwrap().properties.contains_key("attach")
}

pub fn load_objects_in_layer(retry: bool, 
    score_board: &ScoreBoard, 
    tiled_map: &Map, 
    layer_name: &str) -> Vec<GameObject> {

    let objects_layer = tiled_map.layers.get(layer_name).unwrap();

    if retry { score_board.collectibles.clone() } 
    else { 
        objects_layer.objects
        .iter()
        .map(|entry| 
            GameObject {
                world_x: entry.world_x,
                world_y: entry.world_y,
                width: entry.world_w,
                height: entry.world_h,
                name: entry.name.clone(),
                collected: None,
                progress: 0.0
            }
    ).collect::<Vec<GameObject>>()}

}

pub fn load_static_colliders(layer_name: &str, tiled_map: &Map, tyle_type: Tile) -> Vec<Tile> {
    let mut static_colliders = vec![];
    for (_x, _y, tile) in tiled_map.tiles(layer_name, None) {
        static_colliders.push(if tile.is_some() {
            tyle_type
        } else {
            Tile::Empty
        });
    }
    static_colliders
}


pub fn load_map_data(resources: &Resources, level: u32, is_warp_zone: bool) -> Map {
    let map_data = if is_warp_zone {
        resources.warp_zones.get(&i32::try_from(if level == 0 {10} else {level}).unwrap()).unwrap()
    }
    else {
        &resources.levels[(if level == 0 {9} else {level - 1}) as usize]
    };

    load_map(
        map_data,
        &[
            ("images/mytileset.png", resources.get_texture("mytileset").clone()),
            ("images/dave_walk.png", resources.get_texture("dave_walk").clone()),
            ("images/dave_idle.png", resources.get_texture("dave_idle").clone()),
            ("images/dave_jump.png", resources.get_texture("dave_jump").clone()),
            ("images/collectibles.png", resources.get_texture("collectibles").clone()),
            ("images/door.png", resources.get_texture("door").clone()),
            ("images/tuple.png", resources.get_texture("tuple").clone()),
            ("images/tuple_r.png", resources.get_texture("tuple_r").clone()),   
            ("images/deadly.png", resources.get_texture("deadly").clone()),     
            ("images/fire1-sheet.png", resources.get_texture("fire1-sheet").clone()),
            ("images/water1-sheet.png", resources.get_texture("water1-sheet").clone()),
            ("images/door_enable_banner.png", resources.get_texture("door_enable_banner").clone()),
            ("images/gun_icon.png", resources.get_texture("gun_icon").clone()),
            ("images/gun.png", resources.get_texture("gun").clone()),
            ("images/jetpack2.png", resources.get_texture("jetpack2").clone()),
            ("images/player_jetpack.png", resources.get_texture("player_jetpack").clone()),
            ("images/stars-sheet.png", resources.get_texture("stars-sheet").clone()),
            ("images/tree.png", resources.get_texture("tree").clone()),
            ("images/climb-sheet.png", resources.get_texture("climb-sheet").clone()),
        ],
        &[],
    )
    .unwrap()

}

// pub fn initialize_world(tiled_map: &Map) -> (World, i32, i32) {
//     // Logic to set up the world
// }

// pub fn create_player(world: &mut World, tiled_map: &Map, has_gun: bool, has_jetpack: bool) -> Player {
//     // Player initialization logic
// }
