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
    pub h_v: i32,

    pub scr_h: i32,
    pub scr_w: i32,
    pub left_hit: bool,
    pub right_hit: bool,
}

impl Ball {

    // Set the outer bound of ball based on x and y, then set dimension
    pub fn new(x: i32, y: i32, w: i32, h:i32, sc_h: i32, sc_w: i32) -> Self {
        Self{
            rect: Rect::new(x,y,w as u32, h as u32),
            h_dir: Direction::Left,
            v_dir: Direction::Up,
            h_v: 3,
            v_v: 0,

            scr_h: sc_h,
            scr_w: sc_w,

            left_hit: false,
            right_hit: false,
        }


    }

    pub fn shift(&mut self, h: &Direction, v: &Direction, v_velocity: i32, h_velocity: i32){

        
            if(self.rect.left() <= 0 || self.rect.right() >= self.scr_w){
                self.rect.x = self.scr_w/2;
                self.v_v = 0;
                self.h_v = 3;
            }



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

                if self.rect.top() - v_velocity > 0 {

                    self.rect.set_y(self.rect.y - v_velocity);
                }else {
                    self.v_dir = Direction::Down;
                }
           },
            Direction::Down => {
                if self.rect.bottom() + v_velocity < self.scr_h{

                    self.rect.set_y(self.rect.y + v_velocity);
                }else {
                    self.v_dir = Direction::Up;
                }
            },
            _ => {}
        }

    }

    pub fn collision_check(&mut self, left_paddle: &Paddle, right_paddle: &Paddle){
        let h = self.h_dir;
        let v = self.v_dir;
        let vv = self.v_v;
        let hv = self.h_v;

        self.shift(&h, &v, vv, hv);


/*
        println!("BEGIN");

        println!("Ball rect: {}", self.rect.x);
        println!("Ball right : {}", self.rect.right());       
        println!("Ball top : {}", self.rect.top());       
        println!("Ball bottom : {}", self.rect.bottom());       
        
        println!("Paddle left : {}", right_paddle.rect.left());
        
        println!("Paddle top : {}", right_paddle.rect.top());
        println!("Paddle bottom : {}", right_paddle.rect.bottom());
*/ 
        
        println!("paddle {}", left_paddle.rect.right());
        println!("ball {}", self.rect.left());
        if left_paddle.rect.right() - 20 > self.rect.left() && (left_paddle.rect.top() <= self.rect.top() && left_paddle.rect.bottom() >= self.rect.bottom() && !self.left_hit && (left_paddle.rect.left() < self.rect.left()) )  {


            self.h_dir = Direction::Right;
            self.left_hit = true;
            self.right_hit = false;

            if (self.rect.top() - left_paddle.rect.top()) <=  50 {
                self.h_v = 5;
                self.v_v = 3;
                self.v_dir = Direction::Down;
            }else if self.rect.top() - left_paddle.rect.top() <= 100 {
                self.h_v = 5;
                self.v_v = 0;
            }else {
                self.h_v = 5;
                self.v_v = 3;
                self.v_dir = Direction::Up;
            }


        } 
         if right_paddle.rect.left() + 20 < self.rect.right() && (right_paddle.rect.top() <= self.rect.top() && right_paddle.rect.bottom() >= self.rect.bottom() && !self.right_hit)  {

            self.right_hit = true;
            self.left_hit = false;
            self.h_dir = Direction::Left;

            if (self.rect.top() - right_paddle.rect.top()) <=  50 {
                self.h_v = 5;
                self.v_v = 3;
                self.v_dir = Direction::Down;
            }else if self.rect.top() - right_paddle.rect.top() <= 100 {
                self.h_v = 5;
                self.v_v = 0;
            }else {
                self.h_v = 5;
                self.v_v = 3;
                self.v_dir = Direction::Up;
            }




        }
        
    }

}
