use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::env;

use sdl2::image::LoadSurface;
use sdl2::pixels::Color;
use sdl2::surface::Surface;

pub struct Texture {
    surface: Surface<'static>   
}

impl Texture {
    pub fn new(tex_path: &str) -> Option<Texture> {
        let surface: Surface = LoadSurface::from_file(Path::new(tex_path)).ok()?;

        Some(Texture {
            surface: surface
        })
    }

    pub fn surface(&self) -> &Surface {
        &self.surface
    }

    pub fn pixel(&self, x: u32, y: u32) -> Color {
        let pixel_index = 3 * (x + y * self.surface.height());
        let pixel_index = pixel_index as usize;

        let pixel_buf = self.surface.without_lock().unwrap();
        Color::RGB (
            pixel_buf[pixel_index],
            pixel_buf[pixel_index + 1],
            pixel_buf[pixel_index + 2],
        )
    }
}

pub struct TextureLoader {
    textures: HashMap<String, Texture>
}

impl TextureLoader {
    pub fn new() -> Self {
        TextureLoader {
            textures: HashMap::new()
        }
    }
    
    /// Loads all the textures found in textures folder
    pub fn new_eager() -> Self {
        let mut tex_loader = TextureLoader {
            textures: HashMap::new()
        };
        tex_loader.load_all();

        tex_loader
    }

    // Returns an option purely to enable nice ? syntax
    fn load_all(&mut self) {
        let tex_paths = self.tex_paths();
        for tex_path in tex_paths {
            let tex_name = tex_path
                .split('/')
                .last()
                .and_then(|s| Some(s.split('.')))
                .and_then(|mut s| s.next());

            if let None = tex_name {
                continue;
            }

            let tex_name = String::from(tex_name.unwrap());

            self.load(&tex_path, &tex_name);
        }
    }

    fn load(&mut self, tex_path: &str, tex_name: &str) -> Option<&Texture> {
        print!("Loading tex \"{}\" from {}... ", tex_name, tex_path);
        
        let texture = Texture::new(&tex_path); 
        match texture {
            Some(tex) => {
                println!("Success");
                self.textures.insert(String::from(tex_name), tex);
                self.textures.get(tex_name)
            },
            None => {
                println!("Fail");
                None
            }
        }
    }

    fn tex_paths(&self) -> Vec<String> {
        // TODO: write a build script that moves assets somewhere near the binary
        let mut tex_names = vec![];

        let mut tex_dir_path = env::current_exe().unwrap();
        tex_dir_path.pop();
        tex_dir_path.pop();
        tex_dir_path.pop();
        tex_dir_path.push(Path::new("textures"));

        for entry in fs::read_dir(tex_dir_path.into_boxed_path()).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let entry_path = String::from(path.to_str().unwrap());

            if fs::metadata(entry.path()).unwrap().is_file() &&
               entry_path.ends_with(".png") {
                tex_names.push(entry_path);
            }
        }

        tex_names
    }

    pub fn load_texture(&mut self, tex_name: &str) -> Option<&Texture> {
        if self.textures.contains_key(tex_name) {
            return self.textures.get(tex_name)
        }

        let mut tex_path = env::current_exe().unwrap();
        tex_path.pop();
        tex_path.pop();
        tex_path.pop();
        tex_path.push(Path::new("textures"));
        let tex_filename = tex_name.to_owned() + ".png";
        tex_path.push(Path::new(&tex_filename));

        self.load(tex_path.to_str().unwrap(), tex_name)
    }

    pub fn texture(&self, tex_name: &str) -> Option<&Texture> {
        self.textures.get(tex_name)
    }
}
