use std::collections::HashMap;

use macroquad::prelude::{animation::{AnimatedSprite, Animation}, *};
use macroquad_tiled::{Map, Object};

pub struct Animations {
    pub deadly_objects: HashMap<String, (Option<AnimatedSprite>, Vec<Object>)>
}

impl Animations {

    pub fn load_deadly_objects(tiled_map: &Map) -> Animations {
        let (animated_fire, fires) = 
                            Animations::load_animation(tiled_map, "fire", 3);
        let (animated_water, waters) = 
                            Animations::load_animation(tiled_map, "water", 5);
        let (animated_grass, grasses) = 
                            Animations::load_animation(tiled_map, "grass", 4);

        let deadly_objects = HashMap::from([
            ("fire1-sheet".to_string(), (animated_fire, fires)),
            ("water1-sheet".to_string(), (animated_water, waters)),
            ("deadly".to_string(), (animated_grass, grasses)),
        ]);

        Animations {
            deadly_objects
        }
    }

    pub fn update(&mut self) {
        for (_, (animated, _)) in self.deadly_objects.iter_mut() {
            if let Some(animated) = animated {
                animated.update();
            }
        }
    }

    fn create_animation(name: &str, frames: u32) -> AnimatedSprite {
        let mut ani = AnimatedSprite::new(
            32,
            32,
            &[
                Animation {
                    name: name.to_string(),
                    row: 0,
                    frames,
                    fps: 4,
                }
            ],
            true,
        );

        ani.set_animation(0);
        ani
    }

    pub fn load_animation(tiled_map: &Map, name: &str, frames: u32) -> (Option<AnimatedSprite>, Vec<Object>) {
        let mut objects = vec![];
        let mut animated_object: Option<AnimatedSprite> = None;
        
        if tiled_map.layers.contains_key(name) {
            animated_object = Some(Self::create_animation(name, frames));
            
            let object_layer = tiled_map.layers.get(name).unwrap();
            objects.extend(object_layer.objects.iter().cloned());
        }

        (animated_object, objects)
    }
}