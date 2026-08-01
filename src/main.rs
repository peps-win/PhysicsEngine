use macroquad::input::KeyCode::R;
use macroquad::input::KeyCode::Space;
use macroquad::input::MouseButton::Left;
use macroquad::prelude::*;
use macroquad::rand::*;

mod Functions;
mod Structs;

use crate::Structs::structs::*;

use Functions::collision::*;
use Functions::control::*;
use Functions::generation::*;
use Functions::motion::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Physics Engine".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: true,
        ..Default::default()
    }
}


//FUNCTION DECLARATIONS

//Control functions
#[macroquad::main("Physics Engine")]
async fn main() {
    //Modifies that gain speed
    let gravity: f32 = 0.3;

    //Controls to starting spawn
    let mut balls = generate_starting_balls(25, 800.0, 600.0);

    let mut held_ball: Option<usize> = None;

    //Initalizes a variable for a timer for printing
    let mut print_timer: f32 = 0.0;

    //Intitalizes the paused variable for pause game state
    let mut paused: bool = false;

    loop {
        //Sets window background color
        clear_background(LIGHTGRAY);

        let window = WindowData {
            //Returns current screen width and height
            //Used for the ball window collision function
            width: screen_width(),
            height: screen_height(),
        };
        
        //Returns the current mouse position
        let (x,y) = mouse_position();

        //Controls the state of the pause
        if is_key_pressed(Space) {
            paused = !paused;
        }

        //Draws the fps on the screen
        draw_fps();


        //BALL CONTROL

        //Balls spawning once per frame when right clicked 
        //Generates a lot of balls very fast
        if is_mouse_button_down(MouseButton::Right) == true {
            balls.push(generate_ball(x, y));
        }

        //Ball movement
        //Adds any balls when mouse is pressed on it to the held_ball vector
        if is_mouse_button_pressed(MouseButton::Left) {
            //.iter_mut gives a mutable reference to each ball in
            //.position returns the position of a ball that is under the mouse
            held_ball = balls.iter_mut().position(|ball| is_mouse_inside_ball(x, y, ball));
        }
        //Balls can be moved when pressed inside of
        if is_mouse_button_down(MouseButton::Left) == true {
            //The index i gives a mutable reference to the specific held ball
            if let Some(i) = held_ball {
                set_ball_to_mouse(x, y, &mut balls[i]);
            } else {
                held_ball = None;
            }
        }

        //Ball Removal
        if is_mouse_button_pressed(Left) & is_key_pressed(R ) {
            
        }

        //If paused then all simulation elements stop running
        if paused != true {

            //Checks for collisions inbetween each ball and every other ball
            //Adjust iterations based on current FPS to control stability vs performance
            let fps: i32 = get_fps();
            let iterations: i32 = (fps/2)*3;

            for _ in 0..iterations {
                for i in 0..balls.len() {
                    let (left, right) = balls.split_at_mut(i+1);
                    let ball_i = &mut left[i];
                    for ball_j in right.iter_mut() {
                        if ball_collision(ball_i, ball_j) == true {
                            ball_collision_correction(ball_i, ball_j);
                        }
                    }
                }
            }

            //Controls the independant movement of the ball
            for ball in balls.iter_mut() {
                gravitational_acceleration(ball, gravity);
                horizontal_movement(ball);
                border_collision(ball, &window);
                speed_decay(ball);
                draw_circle(ball.x, ball.y, ball.radius, ball.ball_color);
            }
    } else {
        //If the simulation is paused it still draws the balls to make sure they do not disappear
        for ball in balls.iter_mut() {
                draw_circle(ball.x, ball.y, ball.radius, ball.ball_color);
            }
    }
        //Prints the ball count to terminal once per second
        print_timer += get_frame_time();
        if print_timer >= 1.0 {
        println!("Ball count: {}", balls.len());
        print_timer = 0.0;
        }

        next_frame().await
    }
}