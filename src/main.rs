extern crate sdl2;
extern crate rand;

use std::time::{ Duration };

use rand::*;

use sdl2::rect::{ Rect, Point };
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;
const SCALE: i32 = 15;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Snake", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut snake = Snake::new();
    let mut food = Food::new();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyUp{ keycode: Some(Keycode::W), .. } => {
                    snake.dir(0, -1);
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    snake.dir(0, 1);
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    snake.dir(-1, 0);
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    snake.dir(1, 0);
                },
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 10));
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        snake.draw(&mut canvas);
        food.draw(&mut canvas);
        if snake.eat(&food.position) {
            food.change_location();
        }
        if snake.death(){
            println!("DEAD");
        }
        snake.update();
        canvas.present();
    }
}

#[derive(Clone, Debug)]
struct Snake {
    head: Point,
    tail: Vec<Point>,
    head_color: Color,
    tail_color: Color,
    total: usize,
    speed_x: i32,
    speed_y: i32,

}

impl Snake {
    fn new() -> Self {
        Snake {
            head: Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2),
            tail : Vec::new(),
            head_color: Color::RGB(255, 0 , 0),
            tail_color: Color::RGB(255, 255 , 0),
            speed_x : 1,
            speed_y : 0,
            total: 0,
        }
    }


    fn draw(&self, canvas: &mut Canvas<Window>) {
        let head_rect = Rect::new(self.head.x, self.head.y, 25, 25);
        canvas.set_draw_color(self.head_color);
        canvas.fill_rect(head_rect).ok();
        canvas.draw_rect(head_rect).ok();

        canvas.set_draw_color(self.tail_color);
        for t in &self.tail {
            let tail_rect = Rect::new(t.x, t.y, 25, 25);
            canvas.fill_rect(tail_rect).ok();
            canvas.draw_rect(tail_rect).ok();
        }
    }

    fn update(&mut self) {
        if self.tail.len() >= 1 {
        for t in 0..self.tail.len()-1 {
            let s = self.tail.len();
            self.tail[s- 1] = Point::new(self.head.x, self.head.y);
            self.tail[t] = self.tail[t+1];
            }
        }

        self.head.x += self.speed_x * SCALE;
        self.head.y += self.speed_y * SCALE;

        if self.head.x > SCREEN_WIDTH {
            self.head.x  = 0;
        } else if self.head.x < 0{
            self.head.x  = SCREEN_WIDTH;
        }

        if self.head.y > SCREEN_HEIGHT {
            self.head.y  = 0;
        } else if self.head.y < 0{
            self.head.y  = SCREEN_HEIGHT;
        }
    }

    fn death(&mut self) -> bool {
            println!("head x: {}, head y: {}", self.head.x, self.head.y);
            for t in &self.tail {
                println!("tail x: {}, tail y: {}", t.x, t.y);
                if self.head.x == t.x+15 || self.head.y  == t.y+15 {
                    return true;
                 }
            }
            false
    }

    fn dir(&mut self, x:i32, y:i32) {
        self.speed_x = x;
        self.speed_y = y;
    }

    fn eat(&mut self, food: &Point ) -> bool {
        if self.head.x + 25 < food.x || self.head.y + 25 < food.y {
            return false;
        }
        if self.head.x > food.x + 10 || self.head.y > food.y + 10 {
            return false;
        }
        self.tail.push(Point::new(self.head.x, self.head.y));
        true
    }
}

struct Food {
    position: Point,
}

impl Food {
    fn new() -> Self {
        let cols = rand::thread_rng().gen_range(SCALE, (SCREEN_WIDTH / SCALE) * SCALE);
        let rows = rand::thread_rng().gen_range(SCALE, (SCREEN_HEIGHT / SCALE) * SCALE);
        let pos = Point::new(cols, rows);

        Food {
            position : pos,
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>) {
        let food_rect = Rect::new(self.position.x, self.position.y, 10 as u32, 10 as u32);
        canvas.set_draw_color(Color::RGB(0,255,0));
        canvas.fill_rect(food_rect).ok();
        canvas.draw_rect(food_rect).ok();
    }

    fn change_location(&mut self) {
        self.position.x = rand::thread_rng().gen_range(SCALE, (SCREEN_WIDTH / SCALE) * SCALE);
        self.position.y = rand::thread_rng().gen_range(SCALE, (SCREEN_HEIGHT / SCALE) * SCALE);
    }
}
