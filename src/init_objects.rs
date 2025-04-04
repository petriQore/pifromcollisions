use macroquad::prelude::*;
use crate::helper::*;

pub struct Objects{
    pub left_square: MyRectangle,
    pub right_square: MyRectangle,
    pub floor: MyRectangle
}

pub fn init_objects() -> Objects {
    let left_square = MyRectangle{
        x: screen_width()/4.0,
        y: screen_height()/2.0,
        w: 50.0,
        h: 50.0,
        v_x: 0.0,
        v_y: 0.0,
        mass: 1.0,    
        clr: LIGHTGRAY
    };

    let right_square = MyRectangle{
        x: screen_width()/2.0,
        y: screen_height()/2.0,
        w: 50.0,
        h: 50.0,
        v_x: -INITIAL_X_VELOCITY,
        v_y: 0.0,
        mass: 100.0,    
        clr: BLUE
    };    

    let floor = MyRectangle{
        x: 0.0,
        y: screen_height()-5.0,
        w: screen_width(),
        h: 10.0,
        v_x: 0.0,
        v_y: 0.0,
        mass: 1.0,    
        clr: RED
    };

    Objects{
        left_square,
        right_square,
        floor   
    } 
}