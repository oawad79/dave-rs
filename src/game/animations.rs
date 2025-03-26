use macroquad::prelude::{animation::{AnimatedSprite, Animation}, *};
use macroquad_tiled::{Map, Object};

pub struct Animations {
    pub animated_fire: Option<AnimatedSprite>,
    pub animated_water: Option<AnimatedSprite>,
    pub animated_grass: Option<AnimatedSprite>,
    pub fires: Vec<Object>,
    pub waters: Vec<Object>,
    pub grasses: Vec<Object>,
}

impl Animations {

    pub fn new(tiled_map: &Map) -> Self {
        let (animated_fire, fires) = 
                            Animations::load_animation(tiled_map, "fire", 3);
        let (animated_water, waters) = 
                            Animations::load_animation(tiled_map, "water", 5);
        let (animated_grass, grasses) = 
                            Animations::load_animation(tiled_map, "grass", 4);

        

        Self {
            animated_fire,
            animated_water,
            animated_grass,
            fires,
            waters,
            grasses,
        }
    }

    pub fn update(&mut self) {
        if self.animated_fire.is_some() {
            self.animated_fire.as_mut().unwrap().update();
        }

        if self.animated_water.is_some() {
            self.animated_water.as_mut().unwrap().update();
        }

        if self.animated_grass.is_some() {
            self.animated_grass.as_mut().unwrap().update();
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