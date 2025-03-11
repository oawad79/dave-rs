use std::collections::HashMap;

use macroquad::{audio::{load_sound, Sound}, prelude::{collections::storage, coroutines::start_coroutine, *}};
use glob::glob;

pub struct Resources {
    pub levels: Vec<String>,
    pub intro_map_json: String,
    pub separator_map_json: String,
    pub font: Font,
    pub numbers: Vec<Texture2D>,
    pub monsters: Vec<Texture2D>,
    pub sounds_keys: HashMap<String, Sound>,
    pub textures_keys: HashMap<String, Texture2D>,
}

impl Resources {
    async fn new() -> Result<Self, macroquad::Error> {
        let mut sounds_keys = HashMap::new();
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
                        sound
                    );
                }
                Err(e) => panic!("{e:?}"),
            }
        }

        // Load textures
        for entry in glob("assets/images/*.png").expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    let texture = load_texture(
                            format!("images/{}", path.file_name().unwrap().to_str().unwrap()).as_str()
                        ).await?;
                    textures_keys.insert(
                        path.file_stem().unwrap().to_os_string().into_string().unwrap(), 
                        texture
                    );
                }
                Err(e) => panic!("{e:?}"),
            }
        }
        
        let mut levels: Vec<String> = Vec::new();
        
        let mut levels_files: Vec<_> = glob("assets/level*.json").expect("Failed to load levels").collect();
        levels_files.sort_by(|a, b| a.iter().cmp(b));

        for entry in levels_files {
            match entry {
                Ok(path) => {
                    let level = load_string(path.file_name().unwrap().to_str().unwrap()).await.unwrap();
                    levels.push(level);
                },
                Err(e) => panic!("{e:?}")
            }     
        }

        let intro_map_json = load_string("intro.json").await.unwrap();
        let separator_map_json = load_string("seperator.json").await.unwrap();

        let font = load_ttf_font("fonts/MightySouly-lxggD.ttf").await.unwrap();
        
        let mut numbers: Vec<Texture2D> = Vec::new();
        for i in 0..=9 {
            numbers.push(load_texture(&format!("images/num{i}.png")).await.unwrap());
        }

        let mut monsters: Vec<Texture2D> = Vec::new();
        for i in 1..=3 {
            monsters.push(load_texture(&format!("images/monster{i}.png")).await.unwrap());
        }
        
        build_textures_atlas();

        Ok(Self { 
            levels,
            intro_map_json,
            separator_map_json,
            font,
            numbers,
            monsters,
            sounds_keys,
            textures_keys
        })
    }

    pub fn get_sound(&self, sound_key: &str) -> &Sound {
        self.sounds_keys.get(sound_key).unwrap()
    }

    pub fn get_texture(&self, texture_key: &str) -> &Texture2D {
        self.textures_keys.get(texture_key).unwrap()
    }

   
    pub async fn load() -> Result<(), macroquad::Error> {
        let resources_loading = start_coroutine(async {
            let resources = Self::new().await.unwrap();
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
