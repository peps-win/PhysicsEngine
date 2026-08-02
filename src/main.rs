use macroquad::input::KeyCode::Space;
use macroquad::prelude::*;
use macroquad::rand::*;

mod functions;
mod structs;

use crate::structs::structs::*;

use functions::collision::*;
use functions::control::*;
use functions::generation::*;
use functions::motion::*;
use functions::screen_text::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Physics Engine".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main("Physics Engine")]
async fn main() {
    //Modifies that gain speed
    let gravity: f32 = 0.3;

    //Controls to starting spawn
    let mut balls = generate_starting_balls(25, 800.0, 600.0);

    //Ball held under mouse
    let mut held_ball: Option<usize> = None;

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

        //Finds mouse current x and y
        let (x, y) = mouse_position();
        
        //Controls the state of the pause
        if is_key_pressed(Space) {
            paused = !paused;
        }

        //Draws the FPS, and ball count on the screen
        draw_screen_text(&balls);

        //BALL CONTROL
        ball_spawning(x, y, &mut balls);
        ball_movement(&mut held_ball, x, y, &mut balls);
        ball_removal(&mut held_ball, x, y, &mut balls);
        remove_all_balls(&mut balls);
        ball_attraction(&mut balls, x, y);
        ball_dispelation(&mut balls, x, y);
        radius_modification(&mut held_ball, &mut balls, x, y);

        //If paused then all simulation elements stop running
        if paused != true {
            //Checks for collisions inbetween each ball and every other ball
            //Adjust iterations based on current FPS to control stability vs performance
            resolve_all_collisions(&mut balls);

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

        next_frame().await
    }
}
