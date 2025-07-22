extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;

mod paddle;
use crate::paddle::Direction;



use sdl2::image::{self, LoadTexture, InitFlag};
mod ball;

pub fn main() {

    // Set up window, set context and set up canvas for rendering
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Rust Pong", 800, 600).position_centered().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();



    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG);
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("../../assets/Poke.png").unwrap();


    // Set background color, clear buffer and present new buffer
    canvas.set_draw_color(Color::RGB(0,255,255));
    canvas.clear();
    canvas.present();


    // Assign var with event queue
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut i = 0;

    // Creating left and right paddle based on class
    let mut left_paddle = paddle::Paddle::new(100,100,50,200); 
    let mut right_paddle = paddle::Paddle::new(650, 100,50,200);


    // Bools for direction for each paddle, as well as bool for running rendering loop
    let mut down = false;
    let mut up = false;
    let mut RU = false;
    let mut RD = false;
    let mut running = true;


    let mut tst = Rect::new(400, 100, 100, 100);


    // Main rendering loop
    'running: loop {
        if !running {
            break 'running
        }
        i = (i + 1) % 255;

        canvas.set_draw_color(Color::RGB(i, 0, 255 - i));
        canvas.clear();
       
        // Event handler and paddle_handler for moving paddles
        event_handler(&mut event_pump, &mut down,&mut up,&mut running, &mut RU, &mut RD);
        paddle_handler(&mut left_paddle, &mut right_paddle, up, down, RU, RD);

        // Set background, draw both paddles, then present buffer, sleep for duration, then re run
        // loop if running variable still true
       canvas.set_draw_color(Color::RGB(i,i,i));
        canvas.fill_rect(left_paddle.rect).unwrap();
        canvas.fill_rect(right_paddle.rect).unwrap();
        canvas.copy(&texture, None, tst);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}



fn event_handler(event_pump:&mut sdl2::EventPump, down: &mut bool, up: &mut bool, running: &mut bool, RU: &mut bool, RD: &mut bool) {

        for event in event_pump.poll_iter(){
            match event{
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    *running = false;                    
                    
                },

                Event::KeyDown {keycode: Some(Keycode::W), ..} => {
                    *up = true;
                    *down = false;
                },

                Event::KeyDown {keycode: Some(Keycode::S), ..} => {
                    *down = true;
                    *up = false;
                },
                Event::KeyDown {keycode: Some(Keycode::Up), ..} => {
                    *RU = true;
                    *RD = false;
                },
                Event::KeyDown {keycode: Some(Keycode::Down), ..} => {
                    *RD = true;
                    *RU = false;
                },

                _ => {}
            }
        }


}

fn paddle_handler(left_paddle: &mut paddle::Paddle, right_paddle:&mut paddle::Paddle, LU: bool, LD: bool, RU: bool, RD: bool) {
    if LU {
        left_paddle.slide(Direction::Up);
    }else if LD {
        left_paddle.slide(Direction::Down);
    }

    if RU {
        right_paddle.slide(Direction::Up);
    }else if RD {
        right_paddle.slide(Direction::Down);
    }
}
