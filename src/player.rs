use crate::prelude::*;

pub struct Player {
    pub position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    /// Calculates the screen position of the player, uses set to draw
    /// the @ symbol (the player) at that screen location.
    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(1); // sets player to console 2 (player console)
        ctx.set(
            self.position.x - camera.left_x,
            self.position.y - camera.top_y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }

    /// Function to respond to keyboard commands from the user. Calculates and determines
    /// if the player can enter a desired tile. Focuses camera on player.
    pub fn update(&mut self, ctx: &mut BTerm, map : &Map, camera: &mut Camera) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Down => Point::new(0, 1),
                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::W => Point::new(0, -1),
                VirtualKeyCode::S => Point::new(0, 1),
                VirtualKeyCode::A => Point::new(-1, 0),
                VirtualKeyCode::D => Point::new(1, 0),
                _ => Point::zero(),
            };

            let new_position = self.position + delta;
            if map.can_enter_tile(new_position) {
                self.position = new_position;
                camera.on_player_move(new_position);
            }
        }
    }
}
