use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::game::Game;
use crate::game::GameComponent;
use crate::vector::Vec2f;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Forward,
    Backward,
    Left,
    Right,
}

pub struct Player {
    pos: Vec2f,
    dir: Vec2f,
    move_dir_buf: Vec<Direction>,
    rotate_dir_buf: Vec<Direction>,
}

impl Player {
    pub fn new(pos: Vec2f) -> Player {
        Player {
            pos,
            dir: Vec2f::new(-1.0, 0.0),
            move_dir_buf: Vec::new(),
            rotate_dir_buf: Vec::new(),
        }
    }

    pub fn pos(&self) -> &Vec2f {
        &self.pos
    }

    pub fn dir(&self) -> &Vec2f {
        &self.dir
    }

    fn add_dir(&mut self, use_move_dir_buf: bool, dir: Direction) {
        let buf = if use_move_dir_buf {
            &mut self.move_dir_buf
        } else {
            &mut self.rotate_dir_buf
        };
        if let Some(direction) = buf.last() {
            if *direction == dir {
                return;
            }
        }

        self.remove_dir(use_move_dir_buf, dir);
        // We need to shadow the first buf as the line above borrows self mutably for a second time
        let buf = if use_move_dir_buf {
            &mut self.move_dir_buf
        } else {
            &mut self.rotate_dir_buf
        };
        buf.push(dir);
    }

    fn remove_dir(&mut self, use_move_dir_buf: bool, dir: Direction) {
        let buf = if use_move_dir_buf {
            &mut self.move_dir_buf
        } else {
            &mut self.rotate_dir_buf
        };
        *buf = buf.iter().filter(|d| **d != dir).copied().collect();
    }
}

impl GameComponent for Player {
    fn update(&mut self, _: &Game, _: u32) {
        if let Some(dir) = self.rotate_dir_buf.last() {
            match dir {
                Direction::Left => self.dir = self.dir.rotate(2.0),
                Direction::Right => self.dir = self.dir.rotate(-2.0),
                _ => (),
            };
        }

        if let Some(dir) = self.move_dir_buf.last() {
            match dir {
                Direction::Forward => self.pos = self.pos + (self.dir * 0.1),
                Direction::Backward => self.pos = self.pos - (self.dir * 0.1),
                Direction::Left => self.pos = self.pos + (self.dir.rotate(90.0) * 0.1),
                Direction::Right => self.pos = self.pos + (self.dir.rotate(-90.0) * 0.1),
                _ => (),
            };
        }
    }

    fn handle_event(&mut self, event: Event) -> Option<Event> {
        match event {
            Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => match keycode {
                Keycode::W => {
                    self.add_dir(true, Direction::Forward);
                    None
                }
                Keycode::S => {
                    self.add_dir(true, Direction::Backward);
                    None
                }
                Keycode::Q => {
                    self.add_dir(true, Direction::Left);
                    None
                }
                Keycode::E => {
                    self.add_dir(true, Direction::Right);
                    None
                }

                Keycode::A => {
                    self.add_dir(false, Direction::Left);
                    None
                }
                Keycode::D => {
                    self.add_dir(false, Direction::Right);
                    None
                }
                _ => Some(event),
            },
            Event::KeyUp {
                keycode: Some(keycode),
                ..
            } => match keycode {
                Keycode::W => {
                    self.remove_dir(true, Direction::Forward);
                    None
                }
                Keycode::S => {
                    self.remove_dir(true, Direction::Backward);
                    None
                }
                Keycode::Q => {
                    self.remove_dir(true, Direction::Left);
                    None
                }
                Keycode::E => {
                    self.remove_dir(true, Direction::Right);
                    None
                }

                Keycode::A => {
                    self.remove_dir(false, Direction::Left);
                    None
                }
                Keycode::D => {
                    self.remove_dir(false, Direction::Right);
                    None
                }
                _ => Some(event),
            },
            _ => Some(event),
        }
    }
}
