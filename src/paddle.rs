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


// Paddle struct, screen height and width and rect object
pub struct Paddle {

    pub rect: Rect,
    pub scr_h: i32,
    pub scr_w: i32
}




impl Paddle {



    // Simple constructor
    pub fn new(x:i32, y:i32, w:i32, h:i32, sc_h: i32, sc_w: i32) -> Self{
      
        Self{
            scr_h: sc_h,
            scr_w: sc_w,
            rect: Rect::new(x,y,w as u32, h as u32)
        }

    }

    // Check to see if direction enum type matches, move paddle in given direction
    pub fn slide(&mut self, direction: Direction){

        // Also check to make sure paddle does not go outside screen height bounds
        match direction {
            Direction::Up => {
                if self.rect.top() - 10 > 0 {

                    self.rect.set_y(self.rect.y - 10);
                }
            },
            Direction::Down => {
                if self.rect.bottom() + 10 < self.scr_h {

                    self.rect.set_y(self.rect.y + 10);
                }
            },
            _ => {}
        }

    }
}
