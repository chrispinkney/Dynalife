mod map;
mod player;
mod map_builder;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::player::*;
    pub use crate::map_builder::*;
}

use prelude::*;

struct State {
    map: Map,
    player: Player
}

impl State {
    fn new() -> Self {
        Self {
            map: Map::new(), // generate a map
            player: Player::new(Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)),

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
    let context = BTermBuilder::simple80x50()
        .with_title("Dynalife")
        .with_fps_cap(30.0)
        .build()?; // finalize initialization into context. also pass errors to parent fx with ? if unable to initialize

    // implicitly run the main loop and return using context object and state. main and main_loop return a BError.
    main_loop(context, State::new())
}
