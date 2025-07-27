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

    pub left : i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,

    h: i32,
    w: i32,
    pub rect: Rect,
}




impl Paddle {



    // Sets out bound of paddle based on x and y value, then sets dimension
    pub fn new(x:i32, y:i32, w:i32, h:i32) -> Self{
      
        Self{
            left: x - (w/2),
            right: x + (w/2),
            top: y - (h/2),
            bottom: y + (h/2),
            h:h,
            w:w,
            rect: Rect::new(x,y,w as u32, h as u32)
        }

    }

    // Check to see if direction enum type matches, move paddle in given direction
    pub fn slide(&mut self, direction: Direction){

        match direction {
            Direction::Up => {

                self.rect.y = self.rect.y - 10;
                self.top = self.rect.y - (self.h/2);
                self.bottom = self.rect.y + (self.h/2);
            },
            Direction::Down => {
                self.rect.y = self.rect.y + 10;
                self.top = self.rect.y - (self.h/2);
                self.bottom = self.rect.y + (self.h/2);
            },
            _ => {}
        }
/*
        println!("Top {}", self.top);
        println!("Bottom {}", self.bottom);
        println!("Left {}", self.left);
        println!("Right {}", self.right);
        println!("Rect.x {} -- Rect.y {}", self.rect.x, self.rect.y);
*/
    }
}
