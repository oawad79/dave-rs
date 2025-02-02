use macroquad::{audio::{load_sound, Sound}, prelude::{collections::storage, coroutines::start_coroutine, *}};

pub struct Resources {
    pub tileset: Texture2D,
    pub tiled_map_json: String,
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
}

impl Resources {
    async fn new() -> Result<Resources, macroquad::Error> {
        let tileset = load_texture("examples/mytileset.png").await.unwrap();
        tileset.set_filter(FilterMode::Nearest);

        let player_walk = load_texture("examples/dave_walk.png").await.unwrap();
        player_walk.set_filter(FilterMode::Nearest);

        let player_idle = load_texture("examples/dave_idle.png").await.unwrap();
        player_idle.set_filter(FilterMode::Nearest);

        let player_jump = load_texture("examples/dave_jump.png").await.unwrap();
        player_jump.set_filter(FilterMode::Nearest);

        let collectibles = load_texture("examples/collectibles.png").await.unwrap();
        collectibles.set_filter(FilterMode::Nearest);

        let door = load_texture("examples/door.png").await.unwrap();
        door.set_filter(FilterMode::Nearest);

        let tuple = load_texture("examples/tuple.png").await.unwrap();
        tuple.set_filter(FilterMode::Nearest);

        let cup = load_texture("examples/cup.png").await.unwrap();
        cup.set_filter(FilterMode::Nearest);

        let sound_collect = load_sound("examples/getitem.wav").await?;
        let sound_jump = load_sound("examples/jump.wav").await?;
        let sound_walk = load_sound("examples/hd-walk.wav").await?;
        let sound_falling = load_sound("examples/fall.wav").await?;
        let sound_cup = load_sound("examples/trophy.wav").await?;
        let sound_win = load_sound("examples/win.wav").await?;

        let tiled_map_json = load_string("examples/level1.json").await.unwrap();
   
        Ok(Resources { 
            tileset,
            tiled_map_json,
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
            sound_win 
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
