use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::rect::Rect;


use crate::paddle::Paddle;

// Simple direction enum, needed copy function for some reason (rust is weird, I forget exactly but
// I think I needed it when I started passing the direction to a function...)
#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up, 
    Down,
    Left,
    Right
}


// Ball struct has rect, horizontal and vertical direction as well as velocity
// Also has screen height and width as well as bool for left and right paddle being hit
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

    // Create rect to place image on, store horizontal and vertical direction and velocity.
    // Store screen height and width, and bools for left paddle hit or right paddle hit
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


    // Shift function to move ball, check to see if ball has gone off side first
    
    pub fn shift(&mut self, h: &Direction, v: &Direction, v_velocity: i32, h_velocity: i32){

        
            if(self.rect.left() <= 0 || self.rect.right() >= self.scr_w){
                self.rect.x = self.scr_w/2;
                self.v_v = 0;
                self.h_v = 3;
            }



    // Next two matches, one to check vertical direction and other to check horizontal
        match h{

            Direction::Left => {
                self.rect.set_x( (self.rect.x - h_velocity)  );

            },
            Direction::Right => {
                self.rect.set_x(self.rect.x + h_velocity);
           },
            _ => {}
        }
    // Also check to see if ball has gone too far down or up, and simply rerverse vertical
    // direction
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

    // Collision check for each paddle
    pub fn collision_check(&mut self, left_paddle: &Paddle, right_paddle: &Paddle){
        let h = self.h_dir;
        let v = self.v_dir;
        let vv = self.v_v;
        let hv = self.h_v;


        // This is where shift for the ball is called each frame, should change later
        self.shift(&h, &v, vv, hv);

       
        // Make sure ball has hit paddle and paddle has not yet been hit (to avoid double hit which
        // can change direction twice)
        if left_paddle.rect.right() - 20 > self.rect.left() && (left_paddle.rect.top() <= self.rect.top() && left_paddle.rect.bottom() >= self.rect.bottom() && !self.left_hit && (left_paddle.rect.left() < self.rect.left()) )  {


            // Will be called each hit no matter where hit occured
            self.h_dir = Direction::Right;
            self.left_hit = true;
            self.right_hit = false;


            // Temporary checks to see area of paddle hit, which changes angle
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
        // Same as left but for right paddle
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
