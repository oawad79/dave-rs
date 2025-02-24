use macroquad::{audio::{load_sound, Sound}, prelude::{collections::storage, coroutines::start_coroutine, *}};

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
    pub cup: Texture2D,
    pub door: Texture2D,
    pub sound_collect: Sound,
    pub sound_jump: Sound,
    pub sound_walk: Sound,
    pub sound_falling: Sound,
    pub sound_cup: Sound,
    pub sound_win: Sound,
    pub fire1: Texture2D,
    pub banner: Texture2D,
    pub king: Texture2D,
    pub font: Font,
    pub quit_texture: Texture2D,
    pub score_texture: Texture2D,
    pub level_texture: Texture2D,
    pub daves_texture: Texture2D,
    pub dave_face: Texture2D,
    pub numbers: Vec<Texture2D>,
    pub thin: Texture2D,
    pub deadly_grass_texture: Texture2D,
}

impl Resources {
    async fn new() -> Result<Resources, macroquad::Error> {
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

        let cup = load_texture("cup.png").await.unwrap();
        cup.set_filter(FilterMode::Nearest);

        let fire1 = load_texture("fire1-sheet.png").await.unwrap();
        fire1.set_filter(FilterMode::Nearest);

        let banner = load_texture("dangerousdave1-sheet.png").await.unwrap();
        banner.set_filter(FilterMode::Nearest);

        let king = load_texture("king.png").await.unwrap();
        king.set_filter(FilterMode::Nearest);

        let sound_collect = load_sound("getitem.wav").await?;
        let sound_jump = load_sound("jump.wav").await?;
        let sound_walk = load_sound("hd-walk.wav").await?;
        let sound_falling = load_sound("fall.wav").await?;
        let sound_cup = load_sound("trophy.wav").await?;
        let sound_win = load_sound("win.wav").await?;

        let mut levels: Vec<String> = Vec::new();
        for i in 1..=2 {
            let level = load_string(&format!("level{}.json", i)).await.unwrap();
            levels.push(level);
        }

        //let tiled_map_json = load_string("level1.json").await.unwrap();
        let intro_map_json = load_string("intro.json").await.unwrap();
        let separator_map_json = load_string("seperator.json").await.unwrap();

        let font = load_ttf_font("fonts/NotoSans-Regular.ttf").await.unwrap();
        
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
            cup,
            door,
            sound_collect, 
            sound_jump, 
            sound_walk, 
            sound_falling, 
            sound_cup, 
            sound_win,
            fire1,
            banner,
            king,
            font,
            quit_texture,
            score_texture,
            level_texture,
            daves_texture,
            dave_face,
            numbers,
            thin,
            deadly_grass_texture
        })
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
