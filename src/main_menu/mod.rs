use macroquad::prelude::{animation::{AnimatedSprite, Animation}, collections::storage, *};
use macroquad_tiled::{load_map, Map, Object};

use crate::{resources::Resources, Scene, SceneChange};

pub struct MainMenu {
    animated_fire: AnimatedSprite,
    fires: Vec<Object>,
}

impl MainMenu {
    pub fn new() -> MainMenu {
        let resources = storage::get::<Resources>();

        let tiled_map = load_map(
            &resources.intro_map_json,
            &[
                ("fire1-sheet.png", resources.fire1.clone()),
                ("dangerousdave1-sheet.png", resources.logo1.clone()),
                ("king.png", resources.king.clone()),
                ("mytileset.png", resources.tileset.clone())
                
            ],
            &[],
        )
        .unwrap();
        
        let fire_layer = tiled_map.layers.get("fire").unwrap();
        let fires = fire_layer.objects.clone();

        storage::store(tiled_map);
        let mut animated = animated_fire();
        animated.set_animation(0);

        MainMenu {  
            animated_fire: animated,
            fires       
        }
    }
}

impl Scene for MainMenu {
    fn update(&mut self) -> Option<SceneChange> {
        
        if is_key_down(KeyCode::Space) {
            return Some(SceneChange::Game);
        }

        self.animated_fire.update();

        None
    }

    fn draw(&self) {
        let tiled_map = storage::get::<Map>();
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
        
        //on click, log mouse position
        if is_mouse_button_down(MouseButton::Left) {
            macroquad::logging::info!("mouse x: {:?}, mouse y: {:?}", mouse_position().0, mouse_position().1);
            macroquad::logging::info!("fire x: {:?}, fire y: {:?}", self.fires[0].world_x, self.fires[0].world_y);
        }

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