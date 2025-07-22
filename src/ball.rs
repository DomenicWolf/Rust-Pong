use sdl2::image::{self, LoadTexture, InitFlag};


struct Ball {
    left: i32,
    right: i32,
    top: i32, 
    bottom: i32
}

impl Ball {


    pub fn new() -> Self {
        Self{left: 1, right: 1, top: 1, bottom: 1}

    }
}
