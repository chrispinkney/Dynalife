mod map;
mod map_builder;
mod player;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
}

use prelude::*;

struct State {
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);

        Self {
            map : map_builder.map,
            player: Player::new(map_builder.player_start),
        }
    }
}

impl GameState for State {
    // provides a window into the currently running bracket-terminal—accessing information like mouse position and keyboard input, and sending commands to draw to the window.
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls(); // clears the game window every tick
        self.player.update(ctx, &self.map); // update player location every tick
        self.map.render(ctx); // render the map every tick
        self.player.render(ctx); // render the player every tick
    }
}

fn main() -> BError {
    let context = BTermBuilder::new() // creates generic terminal which we can manipulate
        .with_title("Dynalife")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") // map layer
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT,"dungeonfont.png") // player layer
        .build()?; // finalize initialization into context. also pass errors to parent fx with ? if unable to initialize

    // implicitly run the main loop and return using context object and state. main and main_loop return a BError.
    main_loop(context, State::new())
}
