use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::game::GameComponent;
use crate::vector::Vec2f;

pub struct Player {
    pos: Vec2f,
    dir: Vec2f
}

impl Player {
    pub fn new(pos: Vec2f) -> Player {
         Player { pos, dir: Vec2f::new(0.0, 0.0) }
    }

    pub fn pos(&self) -> &Vec2f {
        &self.pos
    }
    
    pub fn dir(&self) -> &Vec2f {
        &self.dir
    }
}

impl GameComponent for Player {
    fn handle_event<'a>(&mut self, event: Event) -> Option<Event> {
        match event {
            Event::KeyDown{ keycode: Some(Keycode::W), .. } => {
                self.pos = self.pos + (self.dir * 0.1);
                None
            },
            Event::KeyDown{ keycode: Some(Keycode::S), .. } => {
                self.pos = self.pos - (self.dir * 0.1);
                None
            },
            _ => Some(event)
        }
    }

}
