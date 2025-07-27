use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::rect::Rect;


use crate::paddle::Paddle;

#[derive(Copy, Clone, Debug)]
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
    h: i32,
    w: i32,

    pub rect: Rect,
    pub h_dir: Direction,
    pub v_dir: Direction,
}

impl Ball {

    // Set the outer bound of ball based on x and y, then set dimension
    pub fn new(x: i32, y: i32, w: i32, h:i32) -> Self {
        Self{
            left: (x - (w/2) ) ,
            right: x + (w/2),
            top: y - (h/2) ,
            bottom: y + (h/2) - ((h as f32 * 0.4) as i32),
            h:h,
            w:w,
            rect: Rect::new(x,y,w as u32, h as u32),
            h_dir: Direction::Left,
            v_dir: Direction::Up,
        }


    }

    pub fn shift(&mut self, h: &Direction, v: &Direction, v_velocity: i32, h_velocity: i32){

        match h{

            Direction::Left => {
                self.rect.x = self.rect.x - h_velocity;
                self.left = (self.rect.x - (self.w/2) ) + ((self.w as f32 * 0.4) as i32);
                self.right = self.rect.x + (self.w/2);
            },
            Direction::Right => {
                self.rect.x = self.rect.x + h_velocity;
                self.left = self.rect.x + (self.w/2);
                self.right = self.rect.x - (self.w/2);
           },
            _ => {}
        }

        match v {

            Direction::Up => {
                self.rect.y = self.rect.y - v_velocity;
                self.top = (self.rect.y - (self.h/2) );
                self.bottom = (self.rect.y + (self.h/2)) ;
 
//                self.top = (self.rect.y - (self.h/2) )+ ((self.h as f32 * 0.4) as i32);
 //               self.bottom = (self.rect.y + (self.h/2)) - ((self.h as f32 * 0.4) as i32);
            },
            Direction::Down => {
                self.rect.y = self.rect.y + v_velocity;
                self.top = self.rect.y - (self.h/2);
                self.bottom = self.rect.y + (self.h/2);
            },
            _ => {}
        }

    }

    pub fn collision_check(&mut self, left_paddle: &Paddle, right_paddle: &Paddle){
        let h = self.h_dir;
        let v = self.v_dir;

//        println!(" Ball top {}",self.top); 
 //       println!(" Ball bottom {}", self.bottom);
        self.shift(&h, &v, 0, 3);
        println!("BEGIN");

        println!("Ball rect: {}", self.rect.x);
        println!("Ball left : {}", self.left);       
        println!("Ball top : {}", self.top);       
        println!("Ball bottom : {}", self.bottom);       
        
        println!("Paddle right : {}", left_paddle.right);
        
        println!("Paddle top : {}", left_paddle.top);
        println!("Paddle bottom : {}", left_paddle.bottom);


        if left_paddle.right > self.left && (left_paddle.top < self.top && left_paddle.bottom > self.bottom) {
            println!("tesasdasdasdasdasdasdasdasdasdasdasdasdt");
            self.h_dir = Direction::Right;
            self.v_dir = Direction::Down;

        }

        if(left_paddle.left < self.left && left_paddle.right > self.right){
            println!("Inside");
        }

        
    }

}
