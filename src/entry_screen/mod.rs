use macroquad::prelude::{animation::{AnimatedSprite, Animation}, collections::storage, *};
use macroquad_tiled::{load_map, Map, Object};

use crate::{resources::Resources, Scene, SceneChange};

pub struct EntryScreen {
    animated_fire: AnimatedSprite,
    animated_banner: AnimatedSprite,
    fires: Vec<Object>,
    banner: Vec<Object>,
    collects: Vec<Object>
}

impl EntryScreen {
    pub fn new() -> EntryScreen {
        let resources = storage::get::<Resources>();

        let tiled_map = load_map(
            &resources.intro_map_json,
            &[
                ("fire1-sheet.png", resources.get_texture("fire1-sheet").unwrap().clone()),
                ("dangerousdave1-sheet.png", resources.get_texture("dangerousdave1-sheet").unwrap().clone()),
                ("mytileset.png", resources.get_texture("mytileset").unwrap().clone()),
                ("collectibles.png", resources.get_texture("collectibles").unwrap().clone()),
                
            ],
            &[],
        )
        .unwrap();
        
        let fire_layer = tiled_map.layers.get("fire").unwrap();
        let fires = fire_layer.objects.clone();

        let collect_layer = tiled_map.layers.get("collectibles").unwrap();
        let collects = collect_layer.objects.clone();

        let banner_layer = tiled_map.layers.get("banner").unwrap();
        let banner = banner_layer.objects.clone();

        storage::store(tiled_map);

        let mut animated_fire = animated_fire();
        animated_fire.set_animation(0);

        let mut animated_banner = animated_banner();
        animated_banner.set_animation(0);

        EntryScreen {  
            animated_fire,
            animated_banner,
            fires,
            banner,
            collects       
        }
    }
}

impl Scene for EntryScreen {
    fn update(&mut self) -> Option<SceneChange> {
        
        if is_key_down(KeyCode::Space) {
            return Some(SceneChange::Game{level: 1, retry: false, cheat: false});
        }

        self.animated_fire.update();
        self.animated_banner.update();

        None
    }

    fn draw(&self) {
        let tiled_map = storage::get::<Map>();
        let resources = storage::get::<Resources>();

        for fire in &self.fires {
            tiled_map.spr_ex(
                "fire1-sheet",
                self.animated_fire.frame().source_rect,
                Rect::new(
                    fire.world_x,
                    fire.world_y - 32.0,
                    32.0,
                    32.0,
                ),
            );
        }
       
        for collect in &self.collects {
            tiled_map.spr_ex(
                "collectibles",
                Rect::new(192.0, 0.0, 32.0, 32.0),
                Rect::new(collect.world_x, collect.world_y - 32.0, 32.0, 32.0),
            );
        }

        tiled_map.spr_ex(
            "dangerousdave1-sheet",
            self.animated_banner.frame().source_rect,
            Rect::new(
                self.banner[0].world_x,
                self.banner[0].world_y - 96.0,
                256.0,
                96.0,
            ),
        );
        
        draw_text_ex(
            "A Rust version BY oawad",
            260.0,
            115.0,
            TextParams {
                font: Some(&resources.font), 
                font_size: 12,     
                color: WHITE,  

                ..Default::default()
            },
        );

        draw_text_ex(
            "Press SPACEBAR to Start",
            250.0,
            370.0,
            TextParams {
                font: Some(&resources.font), 
                font_size: 12,
                font_scale: 1.2,
                color: WHITE, 
                ..Default::default()
            },
        );

        tiled_map
            .draw_tiles("logo", Rect::new(0.0, 0.0, 640.0, 384.0), None);
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