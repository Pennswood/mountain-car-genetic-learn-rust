#[derive(Debug, Copy, Clone, PartialEq)]
pub enum engine_option {
    LEFT= -1,
    IDLE = 0,
    RIGHT = 1
}


pub enum CURVE{
    V = 0,
    QUAD = 1,
    FLAT = 2,
    OTHER = 3
}
pub const CURVE_SETTING : CURVE = CURVE::V;

pub const ENGINE_POWER : f64 = 250.0; //Gravity = 1000
pub const GRAVITY: f64 = 1000.0;
pub const WIDTH : usize = 262144;
pub const DELTA_T : f64 = 0.01/8.;

pub const MAX_ITER : usize = WIDTH*32;


pub fn flat() -> [f64; WIDTH] { // slope*|x| from -1<x<1 mapped linearly to 0 -- WIDTH 
    let mut output : [f64; WIDTH] = [0.0; WIDTH];
    for i in 0..WIDTH {
        output[i] = 0.0;
    }
    output
}

pub fn absolute_slope(slope : f64) -> [f64; WIDTH] { // slope*|x| from -1<x<1 mapped linearly to 0 -- WIDTH 
    let mut output : [f64; WIDTH] = [0.0; WIDTH];
    for i in 0..WIDTH {
        if i < WIDTH/2{
            output[i] = -slope;
        } else {
            output[i] = slope;
        }
    }
    output
}

pub fn quad(a : f64, b : f64, c: f64) -> [f64; WIDTH] { //ax^2 + bx + c from -1<x<1 mapped to 0 -- WIDTH
    let mut output : [f64; WIDTH] = [0.0; WIDTH];
    for i in 0..WIDTH {
        let x : f64 = (i as f64 - (WIDTH/2) as f64)/(WIDTH as f64)*2.0;
        output[i] = a*f64::powi(x, 2) + b*x + c;
    }
    output
}

pub fn u8_to_enum_arr(mut input : u8) -> [engine_option;4] {
    let mut bool_arr : [bool;8] = [false;8];
    for i in 0..8 {
        if input%2 == 1{
            bool_arr[i] = true;
        } else {
            bool_arr[i] = false;
        }
        input = input/2;
    }
    let mut output : [engine_option;4] = [engine_option::IDLE;4];
    for i in 0..4{
        if !bool_arr[i*2] { //engine off
            output[i] = (engine_option::IDLE);
        }else if bool_arr[i*2+1]{ //to the right
            output[i] = (engine_option::RIGHT);
        }else {
            output[i] = (engine_option::LEFT);
        }
    }
    output
}