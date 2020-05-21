use std::f32;

use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;

use crate::game::Game;
use crate::game::GameComponent;
use crate::game::GameOpts;
use crate::vector::Vec2f;
use crate::rays::RayGenerator;

use crate::map;

pub struct Camera {
    fov_ang: f32,
    view_dst: f32,
    sqr_view_dst: f32,
    screen_width: u32,

    camera_view: Surface<'static>,
    last_position_drawn_from: Vec2f,
    last_dir_drawn_from: Vec2f,
}

impl Camera {
    pub fn new(fov_ang: f32, view_dst: f32, game_opts: &GameOpts) -> Camera {
        let camera_view = Surface::new(
            game_opts.screen_width,
            game_opts.screen_height,
            PixelFormatEnum::RGB24,
        )
        .unwrap();

        Camera {
            fov_ang,
            view_dst,
            sqr_view_dst: view_dst.powf(2.0),
            screen_width: game_opts.screen_width,

            camera_view,
            last_position_drawn_from: Vec2f::new(f32::MAX, f32::MAX),
            last_dir_drawn_from: Vec2f::new(f32::MAX, f32::MAX),
        }
    }
}

impl GameComponent for Camera {
    fn draw(&mut self, game: &Game, _: u32) -> Option<&Surface> {
        if self.last_position_drawn_from == *game.player().pos()
            && self.last_dir_drawn_from == *game.player().dir()
        {
            return Some(&self.camera_view);
        }
        self.last_position_drawn_from = *game.player().pos();
        self.last_dir_drawn_from = *game.player().dir();

        for ray in RayGenerator::new(self.last_dir_drawn_from, self.screen_width, self.fov_ang) {
            let mut block_color = Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            };

            let mut ray_pos = *game.player().pos();

            loop {
                if (game.player().pos() - ray_pos).sqr_len() >= self.sqr_view_dst {
                    break;
                }

                let ray_continuation_coeff = ray
                    .dst_to_grid_line(&ray_pos)
                    .div_coeffs(&ray.dir)
                    .get_smaller_abs_coeff();

                ray_pos = ray_pos + ray.dir * ray_continuation_coeff;

                let map_index = ray.to_map_index(&ray_pos, 20);
                if map::MAP[map_index] != 0 {
                    if map::MAP[map_index] == 1 {
                        block_color = Color::RGB(255, 0, 0);
                    } else if map::MAP[map_index] == 2 {
                        block_color = Color::RGB(0, 255, 0);
                    } else if map::MAP[map_index] == 3 {
                        block_color = Color::RGB(0, 0, 255);
                    }

                    break;
                }
            }

            let camera_view_width = self.camera_view.width();
            let camera_view_height = self.camera_view.height();
            let dst = (ray_pos - game.player().pos()).project_onto(game.player().dir());
            let block_size = (camera_view_height as f32) / dst;
            let block_size: u32 = if block_size > (camera_view_height / 2) as f32 {
                camera_view_height / 2
            } else {
                block_size as u32
            };

            let fogging = (dst / self.view_dst * 255.0) as i32;
            let fogging = clamp(fogging, 0, 255);

            let block_line_bot = camera_view_height / 2 - block_size;
            let block_line_top = camera_view_height / 2 + block_size;

            self.camera_view.with_lock_mut(|buf| {
                for y in 0..camera_view_height {
                    let index = (y * camera_view_width * 3 + ray.column_index * 3) as usize;
                    if block_line_bot < y && y < block_line_top {
                        buf[index] = clamp(block_color.r as i32 - fogging, 0, 255) as u8;
                        buf[index + 1] = clamp(block_color.g as i32 - fogging, 0, 255) as u8;
                        buf[index + 2] = clamp(block_color.b as i32 - fogging, 0, 255) as u8;
                    } else {
                        buf[index] = 0;
                        buf[index + 1] = 0;
                        buf[index + 2] = 0;
                    }
                }
            })
        }

        Some(&self.camera_view)
    }
}

fn clamp<T>(value: T, min: T, max: T) -> T
where
    T: PartialOrd,
{
    if value < min {
        return min;
    } else if value > max {
        return max;
    }

    value
}
