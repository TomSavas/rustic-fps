# Rustic FPS
There's a pun in there somewhere and I'm not the one who's going to find it.

This is a simple à la Wolfenstein 3D game/demo/learning project with the main 
goal of trying out rust. Also, due to boredom I decided to do the rendering
on CPU. That is the CPU creates a texture and passes it to SDL to draw it on
the screen.

## Preview 
![Preview](Previews/preview.gif)

## Prerequisites
As this uses [rust-sdl2](https://github.com/Rust-SDL2/rust-sdl2) all its
[dependencies](https://github.com/Rust-SDL2/rust-sdl2#requirements) have to
be satisfied for this to work properly.

### Textures
Wolfenstein 3D textures are under id Software's copyright so I cannot redistribute
them. Unfortunatelly, If you want to run this with original textures, you'll have
to hunt them down yourself.

## Todos
- ~~Get it actually rendering something~~
- ~~Fix movement to be smoother~~
- ~~Get rid of the fisheye effect~~
- ~~Texturing~~
- ~~Add fps limiting~~
- Fix fps limiting lol
- Render some text on screen
- Fix texture artifacts
- For God's sake, make it render at more than 120 fps on my laptop at FHD, is this
  too much to ask out of this plate of spaghetti?
- Make an actual game out of it?
- Clean up the mess that is this entire codebase

## Licence 
This project is licenced under MIT Licence - see the [LICENCE](LICENCE) file for details.
