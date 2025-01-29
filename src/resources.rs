use macroquad::{audio::{load_sound, Sound}, prelude::*};
use macroquad_tiled as tiled;

pub(crate) struct Resources {
    pub tiled_map: tiled::Map,
    pub sound_collect: Sound,
    pub sound_jump: Sound,
    pub sound_walk: Sound,
}

impl Resources {
    pub async fn load() -> Result<Resources, macroquad::Error> {
        let tileset = load_texture("examples/mytileset.png").await.unwrap();
        tileset.set_filter(FilterMode::Nearest);

        let player = load_texture("examples/dave_walk.png").await.unwrap();
        player.set_filter(FilterMode::Nearest);

        let player_idle = load_texture("examples/dave_idle.png").await.unwrap();
        player_idle.set_filter(FilterMode::Nearest);

        let player_jump = load_texture("examples/dave_jump.png").await.unwrap();
        player_jump.set_filter(FilterMode::Nearest);

        let diamond = load_texture("examples/diamond.png").await.unwrap();
        diamond.set_filter(FilterMode::Nearest);

        let door = load_texture("examples/door.png").await.unwrap();
        door.set_filter(FilterMode::Nearest);

        let tuple = load_texture("examples/tuple.png").await.unwrap();
        tuple.set_filter(FilterMode::Nearest);

        let sound_collect = load_sound("examples/getitem.wav").await?;
        let sound_jump = load_sound("examples/jump.wav").await?;
        let sound_walk = load_sound("examples/hd-walk.wav").await?;

        let tiled_map_json = load_string("examples/level1.json").await.unwrap();
   
        let tiled_map = tiled::load_map(
            &tiled_map_json,
            &[
                ("mytileset.png", tileset),
                ("dave_walk.png", player),
                ("dave_idle.png", player_idle),
                ("dave_jump.png", player_jump),
                ("diamond.png", diamond),
                ("door.png", door),
                ("tuple.png", tuple),        
            ],
            &[],
        )
        .unwrap();

        Ok(Resources { tiled_map, sound_collect, sound_jump, sound_walk })
    }

    // pub async fn load() -> Result<(), macroquad::Error> {
    //     let resources_loading = start_coroutine(async move {
    //         let resources = Resources::new().await.unwrap();
    //         storage::store(resources);
    //     });

    //     while !resources_loading.is_done() {
    //         clear_background(BLACK);
    //         let text = format!(
    //             "Loading resources {}",
    //             ".".repeat(((get_time() * 2.) as usize) % 4)
    //         );
    //         draw_text(
    //             &text,
    //             screen_width() / 2. - 160.,
    //             screen_height() / 2.,
    //             40.,
    //             WHITE,
    //         );
    //         next_frame().await;
    //     }

    //     Ok(())
    // }
}
