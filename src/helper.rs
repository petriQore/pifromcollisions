use macroquad::prelude::*;
use macroquad::audio::Sound;
use macroquad::audio;

pub const INITIAL_X_VELOCITY: f32 = 300.0;
pub struct MyRectangle {
    pub x: f32,
    pub y: f32, 
    pub w: f32,
    pub h: f32,
    pub v_x: f32,
    pub v_y: f32,
    pub mass: f32,
    pub clr: Color
}

impl MyRectangle{

    pub fn custom_draw_rect(&self) -> () {
    draw_rectangle(self.x, self.y, self.w, self.h, self.clr);
    }

    pub fn gravity(&mut self, floor: &MyRectangle, step: f32){
        let mass = self.w * self.h;

        self.v_y += mass * step;
        self.y += self.v_y * step;

        if self.y+self.h >= floor.y{
            self.y = floor.y - self.h;
            self.v_y = 0.0;

        }    
    }

    pub fn slide(&mut self, other_square: &MyRectangle, floor: &MyRectangle, is_right_square: bool, step: f32, collision_number: &mut i32, hit: &Sound){
        // let mass = self.w * self.h;

        if self.y + self.h == floor.y{
            self.x += self.v_x * step;
        } 

        if self.x <= 0. && !is_right_square {
            self.v_x = -self.v_x;
            *collision_number += 1;
            audio::play_sound_once(hit);

        }
        
        if self.x <= other_square.w && is_right_square {
            self.x = other_square.w;

        }
    }    

}

//https://www.sciencefacts.net/elastic-collision.html
pub fn collisions(left_square: &mut MyRectangle, right_square: &mut MyRectangle, collision_number: &mut i32, hit: &Sound) {
    if left_square.x + left_square.w >= right_square.x {
        *collision_number += 1;
        audio::play_sound_once(hit);
        let old_left_square_v = left_square.v_x;
        let old_right_square_v = right_square.v_x;
        // println!("old_left_square_v: {}, old_right_square_v: {}", old_left_square_v, old_right_square_v);
        
        left_square.v_x = (old_left_square_v * (left_square.mass - right_square.mass) +
                           2.0 * right_square.mass * old_right_square_v) /
                           (left_square.mass + right_square.mass);
                           
        right_square.v_x = (2.0 * left_square.mass * old_left_square_v +
                           old_right_square_v * (right_square.mass - left_square.mass)) /
                           (left_square.mass + right_square.mass);
                           
        // println!("left_square.v_x: {}, right_square.v_x: {}", left_square.v_x, right_square.v_x);
    }
}

pub fn reset(left_square: &mut MyRectangle, right_square: &mut MyRectangle, collision_number: &mut i32, mass: f32) {
    right_square.x = screen_width()/2.0;
    right_square.y = screen_height()/2.0;
    right_square.v_x = -INITIAL_X_VELOCITY;
    right_square.mass = mass;

    left_square.x = screen_width()/4.0;
    left_square.y = screen_height()/2.0;
    left_square.v_x = 0.0;

    *collision_number = 0;
}