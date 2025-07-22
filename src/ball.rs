use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::rect::Rect;

struct Ball {
    left: i32,
    right: i32,
    top: i32, 
    bottom: i32,

    rect: Rect,
    canvas: sdl2::render::Canvas<sdl2::video::Window> 
}

impl Ball {


    pub fn new(x: i32, y: i32, h: i32, w:i32, canvas: sdl2::render::Canvas<sdl2::video::Window>) -> Self {
        Self{
            left: x + (w/2),
            right: x - (w/2),
            top: y + (h/w),
            bottom: y - (w/2),
            rect: Rect::new(x,y,h as u32, w as u32),
            canvas: canvas,
        }

        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.load_texture("../../assets/Poke.png").unwrap();



    }
}
