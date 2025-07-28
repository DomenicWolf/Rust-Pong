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

    pub rect: Rect,
    pub h_dir: Direction,
    pub v_dir: Direction,
    pub v_v: i32,
    pub h_v: i32
}

impl Ball {

    // Set the outer bound of ball based on x and y, then set dimension
    pub fn new(x: i32, y: i32, w: i32, h:i32) -> Self {
        Self{
            rect: Rect::new(x,y,w as u32, h as u32),
            h_dir: Direction::Left,
            v_dir: Direction::Up,
            h_v: 3,
            v_v: 0,
        }


    }

    pub fn shift(&mut self, h: &Direction, v: &Direction, v_velocity: i32, h_velocity: i32){

        match h{

            Direction::Left => {
                self.rect.set_x( (self.rect.x - h_velocity)  );

            },
            Direction::Right => {
                self.rect.set_x(self.rect.x + h_velocity);
           },
            _ => {}
        }

        match v {

            Direction::Up => {
                self.rect.set_y(self.rect.y - v_velocity);
           },
            Direction::Down => {
                self.rect.set_y(self.rect.y + v_velocity);
            },
            _ => {}
        }

    }

    pub fn collision_check(&mut self, left_paddle: &Paddle, right_paddle: &Paddle){
        let h = self.h_dir;
        let v = self.v_dir;
        let vv = self.v_v;
        let hv = self.h_v;

//        println!(" Ball top {}",self.top); 
 //       println!(" Ball bottom {}", self.bottom);
        self.shift(&h, &v, vv, hv);
        println!("BEGIN");

        println!("Ball rect: {}", self.rect.x);
        println!("Ball left : {}", self.rect.left());       
        println!("Ball top : {}", self.rect.top());       
        println!("Ball bottom : {}", self.rect.bottom());       
        
        println!("Paddle right : {}", left_paddle.rect.right());
        
        println!("Paddle top : {}", left_paddle.rect.top());
        println!("Paddle bottom : {}", left_paddle.rect.bottom());


        if left_paddle.rect.right() - 20 > self.rect.left() && (left_paddle.rect.top() <= self.rect.top() && left_paddle.rect.bottom() >= self.rect.bottom())  {
            if (self.rect.top() - left_paddle.rect.top()) <=  50 {
                self.h_v = 5;
                self.v_v = 3;
            }
            println!("tesasdasdasdasdasdasdasdasdasdasdasdasdt");
            self.h_dir = Direction::Right;
            self.v_dir = Direction::Down;

        } 
        else if right_paddle.rect.left() + 20 < self.rect.right() && (right_paddle.rect.top() <= self.rect.top() && left_paddle.rect.bottom() >= self.rect.bottom())  {
            println!("tesasdasdasdasdasdasdasdasdasdasdasdasdt");
            self.h_dir = Direction::Left;
            self.v_dir = Direction::Up;

        }
        
    }

}
