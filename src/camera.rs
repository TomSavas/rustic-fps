use std::cell::RefCell;
use std::convert::From;
use std::rc::Rc;

use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;

use crate::game::Game;
use crate::game::GameComponent;
use crate::game::GameOpts;
use crate::vector::Vec2f;
use crate::player::Player;

pub struct Camera {
    fov_ang: f32,
    screen_width: u32,
    player: Rc<RefCell<Player>>,
    camera_view: Surface<'static>

}

impl Camera {
    pub fn new(fov_ang: f32, game_opts: &GameOpts, player: Rc<RefCell<Player>>) -> Camera {
        let camera_view = Surface::new(game_opts.screen_width, game_opts.screen_height, PixelFormatEnum::RGB24).unwrap();

        Camera { fov_ang,
            screen_width: game_opts.screen_width,
            player,
            camera_view
        }
    }
}

impl GameComponent for Camera {
    fn draw(&mut self, _: &Game, _: u32) -> Option<&Surface> {
        let camera_rect = self.camera_view.rect();
        self.camera_view.fill_rect(camera_rect, Color::RGB(255, 0, 0));

        Some(&self.camera_view)
    }
}

struct Rays {
    current_ray: Vec2f,
    horizontal_offset_per_column: Vec2f,
    max_ray_count: u32,
    generated_ray_count: u32
}

impl Rays {
    fn new(camera: &Camera) -> Rays {
        let mut camera_plane = camera.player.borrow().dir() * camera.fov_ang.to_radians().tan();
        camera_plane = camera_plane.rotate(-90.0);

        let horizontal_offset_per_column = camera_plane / f32::from(camera.screen_width.clone() as u16);
        let current_ray = camera.player.borrow().dir() - camera_plane - horizontal_offset_per_column;

        Rays { current_ray,
            horizontal_offset_per_column,
            max_ray_count: camera.screen_width.clone(),
            generated_ray_count: 0
        }
    }

    fn calc_next_ray(&mut self) -> Option<Vec2f> {
        self.generated_ray_count += 1;
        self.current_ray = self.current_ray + self.horizontal_offset_per_column;

        if self.generated_ray_count < self.max_ray_count {
            return Some(self.current_ray)
        }

        None
    }
}

impl Iterator for Rays {
    type Item = (Vec2f, u32);

    fn next(&mut self) -> Option<Self::Item> {
        match self.calc_next_ray() {
            Some(ray) => Some((ray, self.generated_ray_count)),
            None => None
        }
    }
}
