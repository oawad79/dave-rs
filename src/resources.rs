use macroquad::prelude::*;
use macroquad_tiled as tiled;

pub(crate) struct Resources {
    pub tiled_map: tiled::Map,
}

impl Resources {
    pub async fn load() -> Self {
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

        Resources { tiled_map }
    }
}
