extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;

// Enum for direction paddle is traveling
pub enum Direction {
    Up, 
    Down
}


// Paddle struct, bounds and rect object
pub struct Paddle {
    left : i32,
    right: i32,
    top: i32,
    bottom: i32,

    pub rect: Rect,
}




impl Paddle {



    pub fn new(x:i32, y:i32, h:i32, w:i32) -> Self{
      
        Self{
            left: x + (w/2),
            right: x - (w/2),
            top: y + (h/w),
            bottom: y - (w/2),
            rect: Rect::new(x,y,h as u32, w as u32)
        }

    }

    pub fn slide(&mut self, direction: Direction){

        match direction {
            Direction::Up => {
                self.top = self.top - 10;
                self.bottom = self.bottom + 10;
                self.rect.y = self.rect.y - 10;
            },
            Direction::Down => {
                self.top = self.top + 10;
                self.bottom = self.bottom - 10;
                self.rect.y = self.rect.y + 10;
            },
            _ => {}
        }
    }
}
