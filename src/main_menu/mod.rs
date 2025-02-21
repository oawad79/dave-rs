use macroquad::prelude::{animation::{AnimatedSprite, Animation}, collections::storage, *};
use macroquad_tiled::{load_map, Map, Object};

use crate::{resources::Resources, Scene, SceneChange};

pub struct MainMenu {
    animated_fire: AnimatedSprite,
    animated_banner: AnimatedSprite,
    fires: Vec<Object>,
    banner: Vec<Object>,
}

impl MainMenu {
    pub fn new() -> MainMenu {
        let resources = storage::get::<Resources>();

        let tiled_map = load_map(
            &resources.intro_map_json,
            &[
                ("fire1-sheet.png", resources.fire1.clone()),
                ("dangerousdave1-sheet.png", resources.banner.clone()),
                ("king.png", resources.king.clone()),
                ("mytileset.png", resources.tileset.clone())
                
            ],
            &[],
        )
        .unwrap();
        
        let fire_layer = tiled_map.layers.get("fire").unwrap();
        let fires = fire_layer.objects.clone();

        let banner_layer = tiled_map.layers.get("banner").unwrap();
        let banner = banner_layer.objects.clone();

        storage::store(tiled_map);

        let mut animated_fire = animated_fire();
        animated_fire.set_animation(0);

        let mut animated_banner = animated_banner();
        animated_banner.set_animation(0);

        MainMenu {  
            animated_fire,
            animated_banner,
            fires,
            banner       
        }
    }
}

impl Scene for MainMenu {
    fn update(&mut self) -> Option<SceneChange> {
        
        if is_key_down(KeyCode::Space) {
            return Some(SceneChange::Game);
        }

        self.animated_fire.update();
        self.animated_banner.update();

        None
    }

    fn draw(&self) {
        let tiled_map = storage::get::<Map>();
        let resources = storage::get::<Resources>();

        let shift_by = (screen_width() / 2.0) - 350.0;

        tiled_map.spr_ex(
            "fire1-sheet",
            self.animated_fire.frame().source_rect,
            Rect::new(
                self.fires[0].world_x + shift_by,
                self.fires[0].world_y - 80.0,
                32.0,
                32.0,
            ),
        );

        tiled_map.spr_ex(
            "fire1-sheet",
            self.animated_fire.frame().source_rect,
            Rect::new(
                self.fires[1].world_x + shift_by,
                self.fires[1].world_y - 90.0,
                32.0,
                32.0,
            ),
        );

        tiled_map.spr_ex(
            "dangerousdave1-sheet",
            self.animated_banner.frame().source_rect,
            Rect::new(
                self.banner[0].world_x + shift_by,
                self.banner[0].world_y - 95.0,
                256.0,
                96.0,
            ),
        );
        
        draw_text_ex(
            "A Rust version BY oawad",
            235.0,
            100.0,
            TextParams {
                font: Some(&resources.font), 
                font_size: 12,     
                color: WHITE,  

                ..Default::default()
            },
        );

        draw_text_ex(
            "Press SPACEBAR to Start",
            240.0,
            310.0,
            TextParams {
                font: Some(&resources.font), 
                font_size: 12,
                font_scale: 1.2,
                color: WHITE, 
                ..Default::default()
            },
        );

        tiled_map
            .draw_tiles("logo", Rect::new(shift_by, 0.0, 320.0, 320.0), None);
    }
}

fn animated_fire() -> AnimatedSprite {
    AnimatedSprite::new(
        32,
        32,
        &[
            Animation {
                name: "fire".to_string(),
                row: 0,
                frames: 3,
                fps: 4,
            }
        ],
        true,
        
    )
}

fn animated_banner() -> AnimatedSprite {
    AnimatedSprite::new(
        256,
        96,
        &[
            Animation {
                name: "banner".to_string(),
                row: 0,
                frames: 4,
                fps: 4,
            }
        ],
        true,
        
    )
}