use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use sdl2::video::Window;

use crate::camera::Camera;
use crate::map::Map;
use crate::player::Player;
use crate::vector::Vec2f;
use crate::render_precedence;

pub struct GameOpts {
    pub title: &'static str,
    pub screen_width: u32,
    pub screen_height: u32,
}

struct GameSdlCtx {
    event_pump: EventPump,
    canvas: Canvas<Window>
}

pub struct Game {
    game_opts: GameOpts,
    game_sdl_ctx: GameSdlCtx,
    map: Rc<RefCell<Map>>,
    pub player: Rc<RefCell<Player>>,
    camera: Rc<RefCell<Camera>>,
    components: Vec<Rc<RefCell<dyn GameComponent>>>
}

pub trait GameComponent {
    fn update(&mut self, game: &Game, logic_dt: u32) {}

    fn draw(&mut self, game: &Game, render_dt: u32) -> Option<&Surface> {
        None
    }

    /// Returns None if the event has been handled
    fn handle_event(&mut self, event: Event) -> Option<Event> {
        Some(event)
    }

    /// Handles some subset of events and returns a new event vector
    /// with the handled events removed from it
    fn handle_events(&mut self, events: Vec<Event>) -> Vec<Event> {
        events.iter().cloned().filter_map(|e| self.handle_event(e)).collect()
    }

    /// Returns a target rect on the window
    fn target_rect(&self) -> Option<Rect> {
        None
    }

    fn render_precendce(&self) -> render_precedence::RenderPrecedence {
        render_precedence::RenderPrecedence::CameraView
    }

    fn numeric_render_precedence(&self) -> u32 {
        render_precedence::to_numeric(self.render_precendce())
    }
}

impl Game {
    pub fn new(game_opts: GameOpts) -> Result<Game, Box<dyn Error>> {
        let sdl_ctx = sdl2::init()?;
        let video_subsys = sdl_ctx.video()?;

        let event_pump = sdl_ctx.event_pump()?;

        let window = video_subsys
            .window(game_opts.title, game_opts.screen_width, game_opts.screen_height) 
            .position_centered()
            .build()?;

        let canvas = window
            .into_canvas()
            .build()?;

        let map = Rc::new(RefCell::new(Map::new(game_opts.screen_width, game_opts.screen_height)));
        let player = Rc::new(RefCell::new(Player::new(Vec2f::new(6.6, 5.0))));
        let camera = Rc::new(RefCell::new(Camera::new(45.0, &game_opts, Rc::clone(&player))));

        let mut game = Game {
            map: Rc::clone(&map),
            player: Rc::clone(&player),
            camera: Rc::clone(&camera),
            components: vec![],

            game_opts,
            game_sdl_ctx: GameSdlCtx {
                event_pump,
                canvas
            }
        };

        game.components.push(Rc::clone(&(map as Rc<RefCell<dyn GameComponent>>)));
        game.components.push(Rc::clone(&(player as Rc<RefCell<dyn GameComponent>>)));
        game.components.push(Rc::clone(&(camera as Rc<RefCell<dyn GameComponent>>)));

        game.components.sort_by(|a, b| a.borrow().numeric_render_precedence().cmp(&b.borrow().numeric_render_precedence()));

        Ok(game)
    }

    pub fn run(&mut self) {
        let mut frame_start_time = std::time::Instant::now();
        let mut frame_times = vec![];
        let mut index = 0;
        loop {
            let events: Vec<Event> = self.game_sdl_ctx.event_pump.poll_iter().collect();
            if self.exit_requested(&events) {
                break
            }

            let dt = frame_start_time.elapsed().as_micros();
            frame_times.push(dt);
            if frame_times.len() > 200 {
                frame_times[index] = frame_times.last().unwrap().clone();
                index = (index + 1) % 200;
                frame_times.pop();
            }
            let avg_dt = frame_times.iter().sum::<u128>() / frame_times.len() as u128;
            let fps = 1_000_000 / dt;
            let avg_fps = 1_000_000 / avg_dt;
            print!("\rFrame time: {} us, avg frame time: {} us, fps: {}, avg_fps; {}",
                dt, avg_dt, fps, avg_fps);
            frame_start_time = std::time::Instant::now();

            self.handle_events(events, 0);
            self.update(0);

            self.game_sdl_ctx.canvas.clear();
            self.draw(0);
            self.game_sdl_ctx.canvas.present();
        }
    }

    fn handle_events(&mut self, mut events: Vec<Event>, _event_dt: u32) {
        for component in self.components.iter_mut() {
            events = component.borrow_mut().handle_events(events);
        }
    }

    fn update(&self, logic_dt: u32) {
        for component in self.components.iter() {
            component.borrow_mut().update(&self, logic_dt);
        }
    }

    fn draw(&mut self, render_dt: u32) {
        let mut display_surface = sdl2::surface::Surface::new(self.game_opts.screen_width, self.game_opts.screen_height, PixelFormatEnum::RGB24)
            .unwrap();

        for component in self.components.iter().rev() {
            let target_rect = component.borrow().target_rect();
            let mut component = component.borrow_mut();
            let surface = component.draw(&self, render_dt);

            if let Some(surface) = surface {
                surface.blit_scaled(None, &mut display_surface, target_rect)
                    .expect("Failed blitting a dingdong");
            }
        }

        let texture_creator = self.game_sdl_ctx.canvas.texture_creator();
        let display_texture = texture_creator.create_texture_from_surface(&display_surface)
            .expect("Failed to create a texture from display surface");

        self.game_sdl_ctx.canvas.copy(&display_texture, None, None)
            .expect("Failed to copy display texture to canvas");
    }

    fn exit_requested(&self, events: &Vec<Event>) -> bool {
        for event in events {
            match event {
                Event::Quit {..}
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return true
                },
                _ => return false
            }
        }

        false
    }
}
