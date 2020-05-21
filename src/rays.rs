use std::convert::From;

use crate::vector::Vec2f;

pub struct Ray {
    pub dir: Vec2f,
    pub column_index: u32
}

impl Ray {
    pub fn dst_to_grid_line(&self, ray_pos: &Vec2f) -> Vec2f {
        let dst_to_grid_line = ray_pos.truncate() - ray_pos;
        let (mut x_dst, mut y_dst) = dst_to_grid_line.into();
        
        if x_dst == 0.0 {
            x_dst = self.dir.x().signum();
        } else {
            x_dst += if self.dir.x() > 0.0 { 1.0 } else { 0.0 };
        }

        if y_dst == 0.0 {
            y_dst = self.dir.y().signum();
        } else {
            y_dst += if self.dir.y() > 0.0 { 1.0 } else { 0.0 };
        }

        Vec2f::new(x_dst, y_dst)
    }

    fn to_partial_map_index(ray_dir: f32, ray_pos: f32) -> usize {
        if ray_pos == ray_pos.trunc() {
            if ray_dir > 0.0 {
                ray_pos as usize
            } else {
                ray_pos as usize - 1
            }
        } else {
            ray_pos as usize
        }
    }

    pub fn to_map_index(&self, pos: &Vec2f, map_height: usize) -> usize {
        Ray::to_partial_map_index(self.dir.x(), pos.x()) +
            Ray::to_partial_map_index(self.dir.y(), pos.y()) * map_height
    }
}

pub struct RayGenerator {
    current_ray: Vec2f,
    horizontal_offset_per_column: Vec2f,

    column_count: u32,
    current_ray_column_index: i32,
}

impl RayGenerator {
    pub fn new(dir: Vec2f, screen_width: u32, fov_ang: f32) -> RayGenerator {
        let camera_plane = (dir * fov_ang.to_radians().tan()).rotate(-90.0);

        let horizontal_offset_per_column = camera_plane * 2.0 / f32::from(screen_width as u16);
        let current_ray = dir - camera_plane - horizontal_offset_per_column;

        RayGenerator {
            current_ray,
            horizontal_offset_per_column,

            column_count: screen_width,
            current_ray_column_index: -1,
        }
    }

    fn calc_next_ray_dir(&mut self) -> Option<Vec2f> {
        self.current_ray_column_index += 1;
        self.current_ray = self.current_ray + self.horizontal_offset_per_column;

        if (self.current_ray_column_index as u32) < self.column_count  {
            return Some(self.current_ray);
        }

        None
    }
}

impl Iterator for RayGenerator {
    type Item = Ray;

    fn next(&mut self) -> Option<Self::Item> {
        match self.calc_next_ray_dir() {
            Some(dir) => Some(Ray {
                dir,
                column_index: self.current_ray_column_index as u32,
            }),
            None => None,
        }
    }
}
