extern crate sdl2;

//use std::time;
use std::error::Error;

//use sdl2::pixels::PixelFormatEnum;
//use sdl2::rect::Rect;
//use sdl2::event::Event;
//use sdl2::keyboard::Keycode;

//mod vector;
//mod raycast;
//mod game;

mod vector;
mod game;
mod map;
mod player;
mod camera;
mod render_precedence;

use game::Game;
use game::GameOpts;

const TITLE: &'static str = "Rustic Doom";
const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

const GAME_OPTS: GameOpts = GameOpts {
    title: TITLE,
    screen_width: SCREEN_WIDTH,
    screen_height: SCREEN_HEIGHT
};

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut game = Game::new(GAME_OPTS)?;

    game.run();

    Ok(())
}

/*
const MAP_WIDTH: usize = 20;
const MAP_HEIGHT: usize = 20;
const MAP: [u16; MAP_WIDTH*MAP_HEIGHT] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1,
    1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 1,
    1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

struct GameSurfaces<'a> {
    camera_surface: sdl2::surface::Surface<'a>,
    map_surface: sdl2::surface::Surface<'a>,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Rustic Doom-Clone", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut game_textures = GameSurfaces {
        camera_surface: sdl2::surface::Surface::new(SCREEN_WIDTH, SCREEN_HEIGHT, PixelFormatEnum::RGB24).unwrap(),
        map_surface: sdl2::surface::Surface::new(MAP_WIDTH as u32, MAP_HEIGHT as u32, PixelFormatEnum::RGB24).unwrap(),

    };

    game_textures.map_surface.with_lock_mut(|buffer: &mut [u8]| {
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let index = y * MAP_HEIGHT + x;
                let buffer_index = index * 3;
                if MAP[index] == 1 {
                    buffer[buffer_index] = 255 as u8;
                    buffer[buffer_index + 1] = 255 as u8;
                    buffer[buffer_index + 2] = 255;
                } else if MAP[index] == 2 {
                    buffer[buffer_index] = 255 as u8;
                    buffer[buffer_index + 1] = 0 as u8;
                    buffer[buffer_index + 2] = 0;
                } else {
                    buffer[buffer_index] = 0 as u8;
                    buffer[buffer_index + 1] = 0 as u8;
                    buffer[buffer_index + 2] = 0 as u8;
                }
            }
        }
    });

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        let mut now = time::SystemTime::now();
        //update();
        print!("\rLogic frametime: {} us. ", now.elapsed()?.as_micros());
        now = time::SystemTime::now();
        render(&mut game_textures, &mut canvas);
        print!("Render frametime: {} us", now.elapsed()?.as_micros());
    }
    Ok(())
}

fn render(game_surfaces: &mut GameSurfaces, canvas: &mut sdl2::render::WindowCanvas) {
    //render_camera();
    //render_map(game_surfaces);

    canvas.clear();

    let mut display_surface = sdl2::surface::Surface::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, PixelFormatEnum::RGB24).unwrap();
    game_surfaces.camera_surface.blit(None, &mut display_surface, None).expect("Failed blitting camera surface");
    game_surfaces.map_surface.blit_scaled(None, &mut display_surface,
        Some(Rect::new(SCREEN_WIDTH as i32 - SCREEN_WIDTH as i32/5, 0, SCREEN_WIDTH/5, SCREEN_HEIGHT/5)))
        .expect("Failed blitting map surface");

    let texture_creator = canvas.texture_creator();
    let display_texture = texture_creator.create_texture_from_surface(&display_surface).unwrap();
    //let display_texture = texture_creator.create_texture_from_surface(&game_surfaces.map_surface).unwrap();

    canvas.copy(&display_texture, None, None)
        .expect("Failed to copy camera texture to canvas");
    canvas.present();
}
*/
