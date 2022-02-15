use crate::settings;


pub fn simulate(mut actions : Vec<settings::engine_option>) -> i64 {
    
    let mut curve : [f64; settings::WIDTH];
    if settings::CURVE_SETTING as i64 == settings::CURVE::V as i64{
        curve = settings::absolute_slope(1.0);
    }else if settings::CURVE_SETTING  as i64 == settings::CURVE::QUAD as i64 {
        curve = settings::quad(1.0,0.0,0.0);
    }else {
        curve = settings::flat();
    }

    let mut pos : f64 = settings::WIDTH as f64/2.0;
    let mut vel : f64 = 0.0;
    let mut accel : f64 = 0.0;
    loop {
        pos = pos + vel*settings::DELTA_T;
        if pos < 0.0 {
            pos = 0.0;
        } else if pos >= settings::WIDTH as f64 {
            break;
        }
        vel = vel + (accel*settings::DELTA_T);
        let engine_action = match actions.pop()  {
            Some(v) => (v as i64),
            None => -2
        };
        if engine_action == -2 {
            break;
        } else {
            accel = (settings::ENGINE_POWER*(engine_action as f64)-curve[pos as usize].sin()*settings::GRAVITY);
        }

    }
    pos as i64

}

fn main(){
    let my_actions = vec![
    settings::engine_option::LEFT,
    settings::engine_option::RIGHT,
    settings::engine_option::RIGHT,
    settings::engine_option::RIGHT,
    settings::engine_option::RIGHT,
    settings::engine_option::RIGHT,
    settings::engine_option::RIGHT,
    settings::engine_option::RIGHT,
    settings::engine_option::RIGHT,
    settings::engine_option::RIGHT,
    settings::engine_option::RIGHT,
    settings::engine_option::RIGHT,
    settings::engine_option::RIGHT,
    settings::engine_option::RIGHT,
    settings::engine_option::RIGHT,
    settings::engine_option::RIGHT,
    settings::engine_option::RIGHT,
    settings::engine_option::RIGHT,
    settings::engine_option::RIGHT
    ];
    let final_pos = simulate(my_actions);
    println!("{}", final_pos);
}