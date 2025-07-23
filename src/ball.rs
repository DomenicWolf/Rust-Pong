use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::rect::Rect;


use crate::paddle::Paddle;


pub enum Direction {
    Up, 
    Down,
    Left,
    Right
}


// Ball struct has outer bound and a rect for position
pub struct Ball {
    left: i32,
    right: i32,
    top: i32, 
    bottom: i32,

    pub rect: Rect,
}

impl Ball {

    // Set the outer bound of ball based on x and y, then set dimension
    pub fn new(x: i32, y: i32, h: i32, w:i32) -> Self {
        Self{
            left: x + (w/2),
            right: x - (w/2),
            top: y + (h/w),
            bottom: y - (w/2),
            rect: Rect::new(x,y,h as u32, w as u32),
        }


    }

    pub fn shift(&mut self, h: Direction, v: Direction){

        match h{

            Direction::Left => {
                self.rect.x = self.rect.x - 4;
                self.left = self.left - 4;
                self.right = self.right + 4;
            },
            Direction::Right => {
                self.rect.x = self.rect.x + 4;
                self.left = self.left + 4;
                self.right = self.right - 4;
            },
            _ => {}
        }
/*
        match v {

            Direction::Up => {
                self.rect.y = self.rect.y - 4;
                self.top = self.top - 4;
                self.bottom = self.bottom + 4;
            },
            Direction::Down => {
                self.rect.y = self.rect.y + 4;
                self.top = self.top + 4;
                self.bottom = self.bottom - 4;
            },
            _ => {}
        }
*/
    }

    pub fn collision_check(&mut self, left_paddle: &Paddle, right_paddle: &Paddle){

       // println!("{}",self.left); 

        self.shift(Direction::Left, Direction::Up);

        println!("{}", self.top);       
        println!("{}", left_paddle.top);
        


        if left_paddle.left < self.left && left_paddle.top < self.top && left_paddle.bottom > self.bottom && left_paddle.right > self.right {
            println!("test");

        }

        
    }

}
