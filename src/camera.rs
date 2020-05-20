use std::convert::From;
use std::f32;

use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;

use crate::game::Game;
use crate::game::GameComponent;
use crate::game::GameOpts;
use crate::vector::Vec2f;

use crate::map;

pub struct Camera {
    fov_ang: f32,
    screen_width: u32,
    camera_view: Surface<'static>,
    last_position_drawn_from: Vec2f,
    last_dir_drawn_from: Vec2f,
}

impl Camera {
    pub fn new(fov_ang: f32, game_opts: &GameOpts) -> Camera {
        let camera_view = Surface::new(
            game_opts.screen_width,
            game_opts.screen_height,
            PixelFormatEnum::RGB24,
        )
        .unwrap();

        Camera {
            fov_ang,
            screen_width: game_opts.screen_width,
            camera_view,
            last_position_drawn_from: Vec2f::new(f32::MAX, f32::MAX),
            last_dir_drawn_from: Vec2f::new(f32::MAX, f32::MAX),
        }
    }

    fn calculate_distance_to_wall(&self, pos: &Vec2f, ray_dir: &Vec2f) -> Vec2f {
        let rounded_pos = Vec2f::new(pos.x() as u32 as f32, pos.y() as u32 as f32);

        let mut distance_to_x_wall = rounded_pos.x() - pos.x();
        if distance_to_x_wall == 0.0 {
            distance_to_x_wall = ray_dir.x().signum();
        } else {
            distance_to_x_wall += if ray_dir.x() > 0.0 { 1.0 } else { 0.0 };
        }

        let mut distance_to_y_wall = rounded_pos.y() - pos.y();
        if distance_to_y_wall == 0.0 {
            distance_to_y_wall = ray_dir.y().signum();
        } else {
            distance_to_y_wall += if ray_dir.y() > 0.0 { 1.0 } else { 0.0 };
        }

        Vec2f::new(distance_to_x_wall, distance_to_y_wall)
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
        let rays = Rays::new(self.last_dir_drawn_from, self.screen_width, self.fov_ang);

        for (ray_dir, column_index) in rays {
            let mut block_color = Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            };

            let org_pos = *game.player().pos();
            let mut pos = *game.player().pos();

            let max_viewing_distance: f32 = 15.0;
            let sqr_max_veiwing_distance = max_viewing_distance.powf(2.0);

            loop {
                if (pos - org_pos).sqr_len() >= sqr_max_veiwing_distance {
                    break;
                }

                let dst_to_wall = self.calculate_distance_to_wall(&pos, &ray_dir);

                let ray_continuation_coeffs =
                    Vec2f::new(dst_to_wall.x() / ray_dir.x(), dst_to_wall.y() / ray_dir.y());

                let ray_continuation_coeff =
                    if ray_continuation_coeffs.x().abs() < ray_continuation_coeffs.y().abs() {
                        ray_continuation_coeffs.x().abs()
                    } else {
                        ray_continuation_coeffs.y().abs()
                    };

                pos = pos + ray_dir * ray_continuation_coeff;

                let x_index = if pos.x() == pos.x().trunc() {
                    if ray_dir.x() > 0.0 {
                        pos.x() as usize
                    } else {
                        pos.x() as usize - 1
                    }
                } else {
                    pos.x() as usize
                };

                let y_index = if pos.y() == pos.y().trunc() {
                    if ray_dir.y() > 0.0 {
                        pos.y() as usize
                    } else {
                        pos.y() as usize - 1
                    }
                } else {
                    pos.y() as usize
                };

                let map_index = x_index + y_index * 20;
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
            let dst = (pos - org_pos).len();
            let block_size = (camera_view_height as f32) / dst;
            let block_size: u32 = if block_size > (camera_view_height / 2) as f32 {
                camera_view_height / 2
            } else {
                block_size as u32
            };

            let fogging = (dst / max_viewing_distance * 255.0) as i32;
            let fogging = clamp(fogging, 0, 255);

            let block_line_bot = camera_view_height / 2 - block_size;
            let block_line_top = camera_view_height / 2 + block_size;

            self.camera_view.with_lock_mut(|buf| {
                for y in 0..camera_view_height {
                    let index = (y * camera_view_width * 3 + column_index * 3) as usize;
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

struct Rays {
    current_ray: Vec2f,
    horizontal_offset_per_column: Vec2f,
    max_ray_count: u32,
    generated_ray_count: u32,
}

impl Rays {
    fn new(dir: Vec2f, screen_width: u32, fov_ang: f32) -> Rays {
        let mut camera_plane = dir * fov_ang.to_radians().tan();
        camera_plane = camera_plane.rotate(-90.0);

        let horizontal_offset_per_column = camera_plane * 2.0 / f32::from(screen_width as u16);
        let current_ray = dir - camera_plane - horizontal_offset_per_column;

        Rays {
            current_ray,
            horizontal_offset_per_column,
            max_ray_count: screen_width,
            generated_ray_count: 0,
        }
    }

    fn calc_next_ray(&mut self) -> Option<Vec2f> {
        self.generated_ray_count += 1;
        self.current_ray = self.current_ray + self.horizontal_offset_per_column;

        if self.generated_ray_count < self.max_ray_count {
            return Some(self.current_ray);
        }

        None
    }
}

impl Iterator for Rays {
    type Item = (Vec2f, u32);

    fn next(&mut self) -> Option<Self::Item> {
        match self.calc_next_ray() {
            Some(ray) => Some((ray, self.generated_ray_count)),
            None => None,
        }
    }
}
