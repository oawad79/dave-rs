use std::collections::HashMap;

use macroquad::{audio::{load_sound, Sound}, prelude::{collections::storage, coroutines::start_coroutine, *}};
use slotmap::{new_key_type, SlotMap};
use glob::glob;

new_key_type! {
    pub struct SoundKey;
    pub struct TextureKey;
}

pub struct Resources {
    pub levels: Vec<String>,
    pub intro_map_json: String,
    pub separator_map_json: String,
    pub font: Font,
    pub numbers: Vec<Texture2D>,
    pub monsters: Vec<Texture2D>,
    sounds: SlotMap<SoundKey, Sound>,
    pub sounds_keys: HashMap<String, SoundKey>,
    textures: SlotMap<TextureKey, Texture2D>,
    pub textures_keys: HashMap<String, TextureKey>,

}

impl Resources {
    async fn new() -> Result<Resources, macroquad::Error> {
        let mut sounds = SlotMap::with_key();
        let mut sounds_keys = HashMap::new();

        let mut textures = SlotMap::with_key();
        let mut textures_keys = HashMap::new();

        // Load sounds
        for entry in glob("assets/sounds/*.wav").expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    let sound = load_sound(
                            format!("sounds/{}", path.file_name().unwrap().to_str().unwrap()).as_str()
                        ).await?;
                    sounds_keys.insert(
                        path.file_stem().unwrap().to_os_string().into_string().unwrap(), 
                        sounds.insert(sound)
                    );
                }
                Err(e) => panic!("{:?}", e),
            }
        }

        // Load textures
        for entry in glob("assets/*.png").expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    let texture = load_texture(
                            format!("{}", path.file_name().unwrap().to_str().unwrap()).as_str()
                        ).await?;
                    textures_keys.insert(
                        path.file_stem().unwrap().to_os_string().into_string().unwrap(), 
                        textures.insert(texture)
                    );
                }
                Err(e) => panic!("{:?}", e),
            }
        }
        
        let mut levels: Vec<String> = Vec::new();
        for i in 1..=4 {
            let level = load_string(&format!("level{}.json", i)).await.unwrap();
            levels.push(level);
        }

        let intro_map_json = load_string("intro.json").await.unwrap();
        let separator_map_json = load_string("seperator.json").await.unwrap();

        let font = load_ttf_font("fonts/MightySouly-lxggD.ttf").await.unwrap();
        
        let mut numbers: Vec<Texture2D> = Vec::new();
        for i in 0..=9 {
            numbers.push(load_texture(&format!("num{}.png", i)).await.unwrap());
        }

        let mut monsters: Vec<Texture2D> = Vec::new();
        for i in 1..=2 {
            monsters.push(load_texture(&format!("monster{}.png", i)).await.unwrap());
        }
        
        //build_textures_atlas();

        Ok(Resources { 
            levels,
            intro_map_json,
            separator_map_json,
            font,
            numbers,
            monsters,
            sounds,
            sounds_keys,
            textures,
            textures_keys
        })
    }

    pub fn get_sound(&self, sound_key: &str) -> Option<&Sound> {
        let x = self.sounds_keys.get(sound_key).unwrap();
        self.sounds.get(*x)
    }

    pub fn get_texture(&self, texture_key: &str) -> Option<&Texture2D> {
        let x = self.textures_keys.get(texture_key).unwrap();
        self.textures.get(*x)
    }

    pub async fn load() -> Result<(), macroquad::Error> {
        let resources_loading = start_coroutine(async {
            let resources = Resources::new().await.unwrap();
            storage::store(resources);
        });
        
        while !resources_loading.is_done() {
            clear_background(BLACK);
            draw_text(
                &format!(
                    "Loading resources {}",
                    ".".repeat(((get_time() * 2.0) as usize) % 4)
                ),
                screen_width() / 2.0 - 160.0,
                screen_height() / 2.0,
                40.,
                WHITE,
            );
    
            next_frame().await;
        }

        Ok(())
    }

}
