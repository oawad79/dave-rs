use macroquad::time::get_frame_time;
use macroquad_tiled::Map;

#[allow(clippy::struct_excessive_bools)]
pub struct GameState {
    pub monster_explosion_active: bool,
    pub monster_explosion_timer: f32,
    pub player_explosion_active: bool,
    pub player_explosion_timer: f32,
    pub message_coord: (f32, f32),
    pub cheat: bool,
    pub is_warp_zone: bool,
}

impl GameState {
    pub fn new(tiled_map: &Map, cheat: bool, is_warp_zone: bool) -> Self {
        let message_coord = (
            tiled_map.layers.get("message").unwrap().objects[0].world_x,
            tiled_map.layers.get("message").unwrap().objects[0].world_y,
        );

        Self {
            monster_explosion_active: false,
            monster_explosion_timer: 0.0,
            player_explosion_active: false,
            player_explosion_timer: 0.0,
            message_coord,
            cheat,
            is_warp_zone,
        }
    }

    pub fn update(&mut self) {
        if self.monster_explosion_active {
            self.monster_explosion_timer -= get_frame_time();
            if self.monster_explosion_timer <= 0.0 {
                self.monster_explosion_active = false;
            }
        }

        if self.player_explosion_active {
            self.player_explosion_timer -= get_frame_time();
            if self.player_explosion_timer <= 0.0 {
                self.player_explosion_active = false;
            }
        }
    }
}
