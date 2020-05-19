use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::surface::Surface;

use crate::render_precedence::RenderPrecedence;
use crate::game;

const MAP: [u16; 400] = [
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

pub struct Map {
    screen_width: u32,
    screen_height: u32,
    map_surface: Surface<'static>
}

impl Map {
    pub fn new(screen_width: u32, screen_height: u32) -> Map {
        let mut map_surface = sdl2::surface::Surface::new(20, 20, PixelFormatEnum::RGB24).unwrap();

        map_surface.with_lock_mut(|buffer: &mut [u8]| {
            for y in 0..20 {
                for x in 0..20 {
                    let index = y * 20 + x;
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


        Map { screen_width, screen_height, map_surface }
    }
}

impl game::GameComponent for Map {
    fn update(&mut self, _game: &game::Game, _logic_dt: u32) {}

    fn draw(&mut self, _game: &game::Game, _render_dt: u32) -> Option<&Surface> {
        Some(&self.map_surface)
    }

    fn target_rect(&self) -> Option<Rect> {
        Some(Rect::new(self.screen_width as i32 - self.screen_width as i32/5, 0, self.screen_width/5, self.screen_height/5))
    }

    fn render_precendce(&self) -> RenderPrecedence {
        RenderPrecedence::Map
    }
}
