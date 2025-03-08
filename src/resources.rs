use std::collections::HashMap;

use macroquad::{audio::{load_sound, Sound}, prelude::{collections::storage, coroutines::start_coroutine, *}};
use slotmap::{new_key_type, SlotMap};
use glob::glob;

new_key_type! {
    pub struct SoundKey;
}

pub struct Resources {
    pub tileset: Texture2D,
    pub levels: Vec<String>,
    pub intro_map_json: String,
    pub separator_map_json: String,
    pub player_idle: Texture2D,
    pub player_walk: Texture2D,
    pub player_jump: Texture2D,
    pub collectibles: Texture2D,
    pub tuple: Texture2D,
    pub door: Texture2D,
    pub fire1: Texture2D,
    pub banner: Texture2D,
    pub font: Font,
    pub quit_texture: Texture2D,
    pub score_texture: Texture2D,
    pub level_texture: Texture2D,
    pub daves_texture: Texture2D,
    pub dave_face: Texture2D,
    pub numbers: Vec<Texture2D>,
    pub thin: Texture2D,
    pub deadly_grass_texture: Texture2D,
    pub water_texture: Texture2D,
    pub explosion: Texture2D,
    pub gun_icon: Texture2D,
    pub gun_text: Texture2D,
    pub go_thru: Texture2D,
    pub monsters: Vec<Texture2D>,
    pub bullet: Texture2D,
    pub monster_bullet: Texture2D,
    pub jetpack2: Texture2D,
    pub jetpack_text: Texture2D,
    pub tuple_r: Texture2D,
    sounds: SlotMap<SoundKey, Sound>,
    pub sounds_keys: HashMap<String, SoundKey>

}

impl Resources {
    async fn new() -> Result<Resources, macroquad::Error> {
        let mut sounds = SlotMap::with_key();
        let mut sounds_keys = HashMap::new();

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
        
        let tileset = load_texture("mytileset.png").await.unwrap();
        tileset.set_filter(FilterMode::Nearest);

        let player_walk = load_texture("dave_walk.png").await.unwrap();
        player_walk.set_filter(FilterMode::Nearest);

        let player_idle = load_texture("dave_idle.png").await.unwrap();
        player_idle.set_filter(FilterMode::Nearest);

        let player_jump = load_texture("dave_jump.png").await.unwrap();
        player_jump.set_filter(FilterMode::Nearest);

        let collectibles = load_texture("collectibles.png").await.unwrap();
        collectibles.set_filter(FilterMode::Nearest);

        let door = load_texture("door.png").await.unwrap();
        door.set_filter(FilterMode::Nearest);

        let tuple = load_texture("tuple.png").await.unwrap();
        tuple.set_filter(FilterMode::Nearest);

        let tuple_r = load_texture("tuple_r.png").await.unwrap();
        tuple_r.set_filter(FilterMode::Nearest);

        let go_thru = load_texture("door_enable_banner.png").await.unwrap();
        go_thru.set_filter(FilterMode::Nearest);

        let fire1 = load_texture("fire1-sheet.png").await.unwrap();
        fire1.set_filter(FilterMode::Nearest);

        let banner = load_texture("dangerousdave1-sheet.png").await.unwrap();
        banner.set_filter(FilterMode::Nearest);

        let water_texture = load_texture("water1-sheet.png").await.unwrap();
        water_texture.set_filter(FilterMode::Nearest);

        let explosion = load_texture("explosion.png").await.unwrap();
        explosion.set_filter(FilterMode::Nearest);

        let gun_icon = load_texture("gun_icon.png").await.unwrap();
        gun_icon.set_filter(FilterMode::Nearest);

        let gun_text = load_texture("gun.png").await.unwrap();
        gun_text.set_filter(FilterMode::Nearest);


        let monster1 = load_texture("monster1.png").await.unwrap();
        monster1.set_filter(FilterMode::Nearest);

        let monster2 = load_texture("monster2.png").await.unwrap();
        monster2.set_filter(FilterMode::Nearest);

        let bullet = load_texture("bullet.png").await.unwrap();
        bullet.set_filter(FilterMode::Nearest);

        let monster_bullet = load_texture("monster_bullet.png").await.unwrap();
        monster_bullet.set_filter(FilterMode::Nearest);

        let jetpack2 = load_texture("jetpack2.png").await.unwrap();
        jetpack2.set_filter(FilterMode::Nearest);

        let jetpack_text = load_texture("jetpack.png").await.unwrap();
        jetpack_text.set_filter(FilterMode::Nearest);

        let mut levels: Vec<String> = Vec::new();
        for i in 1..=4 {
            let level = load_string(&format!("level{}.json", i)).await.unwrap();
            levels.push(level);
        }

        let intro_map_json = load_string("intro.json").await.unwrap();
        let separator_map_json = load_string("seperator.json").await.unwrap();

        let font = load_ttf_font("fonts/MightySouly-lxggD.ttf").await.unwrap();
        
        let quit_texture = load_texture("quit.png").await.unwrap();

        let score_texture = load_texture("score.png").await.unwrap();

        let level_texture = load_texture("level.png").await.unwrap();

        let daves_texture = load_texture("daves.png").await.unwrap();

        let dave_face = load_texture("DaveFace.png").await.unwrap();

        let thin = load_texture("thin.png").await.unwrap();

        let deadly_grass_texture = load_texture("deadly.png").await.unwrap();

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
            tileset,
            levels,
            intro_map_json,
            separator_map_json,
            player_idle,
            player_walk,
            player_jump,
            collectibles,
            tuple,
            door,
            fire1,
            banner,
            font,
            quit_texture,
            score_texture,
            level_texture,
            daves_texture,
            dave_face,
            numbers,
            thin,
            deadly_grass_texture,
            water_texture,
            explosion,
            gun_icon, 
            gun_text,
            go_thru,
            monsters,
            bullet,
            monster_bullet,
            jetpack2,
            jetpack_text,
            tuple_r,
            sounds,
            sounds_keys
        })
    }

    pub fn get_sound(&self, sound_key: &str) -> Option<&Sound> {
        let x = self.sounds_keys.get(sound_key).unwrap();
        self.sounds.get(*x)
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
