use macroquad::prelude::*;

pub struct GameCamera {
    pub camera: Camera2D,
}

impl GameCamera {
    pub fn new() -> Self {
        let camera = Camera2D::from_display_rect(Rect::new(0.0, 384.0, 608.0, -384.0));
        Self { camera }
    }

    pub fn set_active(&self) {
        set_camera(&self.camera);
    }

    pub fn update(&mut self, player_pos: Vec2, level: u32, width_tiles: i32) {
        // Don't update camera position for level 1 (except warp zone which is level 0)
        if level <= 1 && level != 0 {
            return;
        }

        let screen_width = screen_width();
        let target_x = if (player_pos.x > screen_width / 2.0)
            && (player_pos.x < (width_tiles * 32) as f32 - screen_width / 3.4)
        {
            player_pos.x
        } else if player_pos.x > 200.0
            && player_pos.x
                < (width_tiles * 32) as f32
                    - (if screen_width > 1000.0 {
                        screen_width / 5.0
                    } else {
                        screen_width / 3.0
                    })
        {
            player_pos.x + 170.0
        } else if player_pos.x < 200.0 {
            305.0
        } else {
            self.camera.target.x
        };

        self.camera.target.x = (target_x - self.camera.target.x).mul_add(0.1, self.camera.target.x);
    }
}
