extern crate sdl2;

use std::error::Error;

mod camera;
mod game;
mod map;
mod player;
mod rays;
mod render_precedence;
mod vector;

use game::Game;
use game::GameOpts;

const TITLE: &'static str = "Rustic FPS";
const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 360;

const GAME_OPTS: GameOpts = GameOpts {
    title: TITLE,
    screen_width: SCREEN_WIDTH,
    screen_height: SCREEN_HEIGHT,
};

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut game = Game::new(GAME_OPTS)?;

    game.run();

    Ok(())
}
