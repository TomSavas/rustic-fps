use std::f32;

use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;

use crate::game::Game;
use crate::game::GameComponent;
use crate::game::GameOpts;
use crate::vector::Vec2f;
use crate::rays::RayGenerator;
use crate::textures::TextureLoader;

use crate::map;

pub struct Camera {
    fov_ang: f32,
    view_dst: f32,
    sqr_view_dst: f32,
    screen_width: u32,

    tex_loader: TextureLoader,

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

            tex_loader: TextureLoader::new_eager(),

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
            let mut tex = self.tex_loader.texture("bluestone");
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
                    if map::MAP[map_index] == 2 {
                        tex = self.tex_loader.texture("eagle")
                    } else if map::MAP[map_index] == 3 {
                        tex = self.tex_loader.texture("redbrick");
                    }

                    break;
                }
            }

            let camera_view_width = self.camera_view.width() as i32;
            let camera_view_height = self.camera_view.height() as i32;
            let dst = (ray_pos - game.player().pos()).project_onto(game.player().dir());
            let block_size = ((camera_view_height as f32) / dst) as i32;

            let dst_for_fogging = if dst < self.view_dst / 2.0 {
                0.0
            } else {
                dst - (self.view_dst / 2.0)
            };
            let fogging = (dst_for_fogging / self.view_dst * 2.0 * 255.0) as i32;
            let fogging = clamp(fogging, 0, 255);

            let block_line_bot = camera_view_height / 2 - block_size;
            let block_line_top = camera_view_height / 2 + block_size;

            let tex_x_index = if ray_pos.x().fract() == 0.0 {
                ray_pos.y().fract() 
            } else { 
                ray_pos.x().fract() 
            };
            //let tex = self.tex_loader.texture("eagle").unwrap();
            let tex = tex.unwrap();
            let tex_x_index = tex_x_index * tex.surface().width() as f32;
            let tex_x_index = tex_x_index as u32;

            let max_view_dst = self.view_dst;

            self.camera_view.with_lock_mut(|buf| {
                for y in 0..camera_view_height {
                    let index = (y * camera_view_width * 3 + ray.column_index as i32 * 3) as usize;
                    if block_line_bot < y && y < block_line_top && dst < max_view_dst {
                        let tex_y_index = (y - block_line_bot) as f32 / (block_line_top - block_line_bot) as f32;
                        let tex_y_index = (tex_y_index * tex.surface().height() as f32) as u32;
                        let color = tex.pixel(tex_x_index, tex_y_index);

                        buf[index] = clamp(color.r as i32 - fogging, 0, 255) as u8;
                        buf[index + 1] = clamp(color.g as i32 - fogging, 0, 255) as u8;
                        buf[index + 2] = clamp(color.b as i32 - fogging, 0, 255) as u8;
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
