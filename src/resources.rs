use std::collections::HashMap;

use include_dir::{
    Dir,
    include_dir,
};
use macroquad::{
    audio::{
        Sound,
        load_sound_from_bytes,
    },
    prelude::{
        collections::storage,
        coroutines::start_coroutine,
        *,
    },
};

static PROJECT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets");

pub struct Resources {
    pub levels: Vec<String>,
    pub warp_zones: HashMap<i32, String>,
    pub intro_map_json: String,
    pub separator_map_json: String,
    pub warp_zone_separator_map_json: String,
    pub done_map_json: String,
    pub font: Font,
    pub numbers: Vec<Texture2D>,
    pub monsters: Vec<Texture2D>,
    pub sounds_keys: HashMap<String, Sound>,
    pub textures_keys: HashMap<String, Texture2D>,
}

impl Resources {
    async fn new() -> Result<Self, macroquad::Error> {
        let sounds_keys: HashMap<String, Sound> = Self::load_embedded_sounds("sounds/*.wav").await;
        let textures_keys = Self::load_embedded_textures("images/*.png");

        let mut levels: Vec<String> = Vec::new();
        let mut levels_files: Vec<_> = PROJECT_DIR
            .find("level*.json")
            .expect("Failed to load levels")
            .collect();
        levels_files.sort_by(|a, b| {
            let num_a: u32 = a.path().file_stem().unwrap().to_str().unwrap()[5..]
                .parse()
                .unwrap();
            let num_b: u32 = b.path().file_stem().unwrap().to_str().unwrap()[5..]
                .parse()
                .unwrap();
            num_a.cmp(&num_b)
        });

        for entry in &levels_files {
            levels.push(Self::load_embedded_string(
                entry.path().file_name().unwrap().to_str().unwrap(),
            ));
        }

        let mut warp_zones: HashMap<i32, String> = HashMap::new();
        let warp_zones_files: Vec<_> = PROJECT_DIR
            .find("warp_level*.json")
            .expect("Failed to load warp levels")
            .collect();
        for entry in &warp_zones_files {
            warp_zones.insert(
                entry.path().file_stem().unwrap().to_str().unwrap()[10..]
                    .parse()
                    .unwrap(),
                Self::load_embedded_string(entry.path().file_name().unwrap().to_str().unwrap()),
            );
        }

        let intro_map_json = Self::load_embedded_string("intro.json");
        let separator_map_json = Self::load_embedded_string("seperator.json");
        let done_map_json = Self::load_embedded_string("done.json");
        let warp_zone_separator_map_json = Self::load_embedded_string("warp.json");

        let font = load_ttf_font_from_bytes(
            PROJECT_DIR
                .get_file("fonts/MightySouly-lxggD.ttf")
                .unwrap()
                .contents(),
        )
        .unwrap();

        let mut numbers: Vec<Texture2D> = Vec::new();
        for i in 0..=9 {
            numbers.push(Self::load_embedded_texture(&format!("images/num{i}.png")));
        }

        let mut monsters: Vec<Texture2D> = Vec::new();
        for i in 1..=8 {
            monsters.push(Self::load_embedded_texture(&format!(
                "images/monster{i}.png"
            )));
        }

        //build_textures_atlas();

        Ok(Self {
            levels,
            warp_zones,
            intro_map_json,
            separator_map_json,
            warp_zone_separator_map_json,
            done_map_json,
            font,
            numbers,
            monsters,
            sounds_keys,
            textures_keys,
        })
    }

    fn load_embedded_string(path: &str) -> String {
        let file = PROJECT_DIR
            .get_file(path)
            .unwrap_or_else(|| panic!("Unable to load string : {path}"));
        str::from_utf8(file.contents()).unwrap().to_string()
    }

    fn load_embedded_textures(path: &str) -> HashMap<String, Texture2D> {
        let mut textures_keys: HashMap<String, Texture2D> = HashMap::new();
        PROJECT_DIR.find(path).unwrap().for_each(|entry| {
            textures_keys.insert(
                entry
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap(),
                Self::load_embedded_texture(entry.path().display().to_string().as_str()),
            );
        });

        textures_keys
    }

    fn load_embedded_texture(path: &str) -> Texture2D {
        let file = PROJECT_DIR.get_file(path).unwrap();
        Texture2D::from_file_with_format(file.contents(), Some(ImageFormat::Png))
    }

    async fn load_embedded_sounds(path: &str) -> HashMap<String, Sound> {
        let mut sounds_keys: HashMap<String, Sound> = HashMap::new();
        for entry in PROJECT_DIR.find(path).unwrap() {
            let f = PROJECT_DIR
                .get_file(entry.path().display().to_string())
                .unwrap();
            sounds_keys.insert(
                f.path()
                    .file_stem()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap(),
                load_sound_from_bytes(f.contents()).await.unwrap(),
            );
        }
        sounds_keys
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
