use macroquad::experimental::camera::mouse;
use macroquad::input::KeyCode::Space;
use macroquad::prelude::*;
use macroquad::rand::*;

struct Ball {
    x: f32,
    y: f32,
    x_velocity: f32,
    y_velocity: f32,
    radius: f32,
    mass: f32,
    restitution: f32,
    ball_color: Color,
}
struct WindowData {
    width: f32,
    height: f32,
}

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

//Handles contact with border, bouncing settling, rolling settling, general speed decay, and wall friction
fn border_collision(ball: &mut Ball, window: &WindowData) {
    //Handles contact with all window borders, bounce settling, and window settling
    
    let bounce_settle_threshold: f32 = 0.001;
    let roll_settle_threshold: f32 = 0.1;
    let ground_friction: f32 = 0.9975;

    //Ball contact with the bottom window boarder
    if ball.y + ball.radius > window.height {
        ball.y = window.height - ball.radius;
        ball.y_velocity *= -ball.restitution;

        //Ball bounce settling when under the settle threashold
        if ball.y_velocity.abs() < bounce_settle_threshold {
            ball.y_velocity = 0.0;
        }

        //Ball roll settling when under the settle threashold
        if ball.x_velocity.abs() < roll_settle_threshold {
            ball.x_velocity = 0.0;
        }

    }

    //Ball friction with the floor
    if ball.y + ball.radius > window.height {
        ball.x_velocity *= ground_friction;
    }

    //Ball contact with the top window boarder
    if ball.y - ball.radius < 0.0 {
        ball.y = ball.radius;
        ball.y_velocity *= -ball.restitution;
    }

    // Ball contact with the side window boarder
    if ball.x - ball.radius < 0.0 {
        ball.x = ball.radius;
        ball.x_velocity *= -ball.restitution;
    } else if ball.x + ball.radius > window.width {
        ball.x = window.width - ball.radius;
        ball.x_velocity *= -ball.restitution;
    }
}
fn gravitational_acceleration(ball: &mut Ball, gravity: f32) {
    //Gradually increases falling speed and pushes the ball down based on the velocity
    
    let terminal_velocity = generate_terminal_velocity(ball);

    if ball.y_velocity < terminal_velocity * 20.0 {
    ball.y_velocity += gravity;
}
    //Adds the current speed to change the position downward
    ball.y += ball.y_velocity;
}
fn horizontal_movement (ball: &mut Ball) {
    //Adds the horizontal speed to the horizontal position

    ball.x += ball.x_velocity
}
fn speed_decay(ball: &mut Ball) {
    //Has a slow speed decay to the x velocity that acts similar to the gravity forcing the ball into the ground

    ball.x_velocity *= 0.999;
}

//Generation aspects of the simulator
fn generate_starting_balls(count: u64, width: f32, height: f32) -> Vec<Ball> {
    //Randomly generates a balls of a certain count inside of the inputted area

    (0..count)
        .map(|_| {
            let radius: f32 = gen_range(10.0, 20.0);

            Ball {
                x: gen_range(0.0, width),
                y: gen_range(0.0, height),
                x_velocity: gen_range(-3.0, 3.0),
                y_velocity: gen_range(-3.0, 3.0),
                radius: radius,
                mass: gen_range(0.01, 0.03) * radius * radius,
                restitution: 0.9,
                ball_color: generate_colors(),
            }
        })
        .collect()
}
fn generate_ball(x: f32, y: f32) -> Ball {
    //Generates a single ball per function call at a specificed coordinate


    let radius: f32 = gen_range(10.0, 20.0);

            Ball {
                x: x,
                y: y,
                x_velocity: gen_range(-3.0, 3.0),
                y_velocity: gen_range(-3.0, 3.0),
                radius: radius,
                mass: gen_range(0.01, 0.03) * radius * radius,
                restitution: 0.9,
                ball_color: generate_colors(),
            }
}
fn generate_colors() -> Color {
    //Generate a new color for each ball with a maximum opacity
    
    Color {
        r: gen_range(0.0,1.0),
        g: gen_range(0.0,1.0),
        b: gen_range(0.0,1.0),
        a: 1.0,
    }
}
fn generate_terminal_velocity(ball: &mut Ball) -> f32 {

    let air_density: f32 = 1.225;
    let drag_coefficient: f32 = 0.47;
    let gravity_accel: f32 = 9.8; // real-world gravity, separate from your per-frame "gravity" constant
    let area: f32 = std::f32::consts::PI * ball.radius * ball.radius;

    (2.0 * ball.mass * gravity_accel / (air_density * drag_coefficient * area)).sqrt()
}

//Ball collision functions
fn ball_collision(ball1: &Ball, ball2: &Ball) -> bool {
    //Detection of a collision between balls

    let dx = ball1.x - ball2.x;
    let dy = ball1.y - ball2.y;
    let radius_sum = ball1.radius + ball2.radius;
    (dx * dx + dy * dy) <= radius_sum * radius_sum
}
fn resolve_ball_overlap(ball1: &mut Ball, ball2: &mut Ball, distance: f32, x_col_norm: f32, y_col_norm: f32) {
    //Finds the overlap of balls and snaps them to a new coordinate
    
    //Calculate the overlap
    let overlap = (ball1.radius + ball2.radius) - distance;
    //Find total colliding balls mass
    let total_mass = ball1.mass + ball2.mass;
    //Finds the ratios of mass for each ball to the total collision mass
    let ratio1 = ball2.mass / total_mass;
    let ratio2 = ball1.mass / total_mass;

    ball1.x += overlap * ratio1 * x_col_norm;
    ball1.y += overlap * ratio1 * y_col_norm;
    ball2.x -= overlap * ratio2 * x_col_norm;
    ball2.y -= overlap * ratio2 * y_col_norm;
}
fn find_collision_impulse(ball1: &Ball, ball2: &Ball, velocity_along_normal: f32) -> f32 {
    //Gets the collision impulse to calculate the amount of bounce from balls

    let e: f32 = ball1.restitution*ball2.restitution;
    return -(1.0 + e) * velocity_along_normal / (1.0 / ball1.mass + 1.0 / ball2.mass);
}
fn change_velocity_from_collision(ball1: &mut Ball, ball2: &mut Ball, collision_impulse: f32, x_col_norm: f32, y_col_norm: f32) {
    //Uses the collision impulse to change the ball's velocity

    ball1.x_velocity += (collision_impulse / ball1.mass) * x_col_norm;
    ball1.y_velocity += (collision_impulse / ball1.mass) * y_col_norm;
    ball2.x_velocity -= (collision_impulse / ball2.mass) * x_col_norm;
    ball2.y_velocity -= (collision_impulse / ball2.mass) * y_col_norm;
}
fn ball_collision_correction(ball1: &mut Ball, ball2: &mut Ball)  {
    //Uses all past ball collision functions to get the numbers to plug into the change_velocity_from_collision function

    //Setup for finding the collision normal
    //X axis distance
    let dx: f32 = ball1.x - ball2.x;
    //Y axis distance
    let dy: f32 = ball1.y - ball2.y;
    //Direct line distance
    let dist: f32 = (dx*dx+dy*dy).sqrt().max(0.0001);
    
    //Calculate the collision normal
    //X axis collision normal
    let nx: f32 = dx/dist;
    //Y axis collision normal
    let ny: f32 = dy/dist;

    //Resolve any overlap inbetween the 2 collided balls
    resolve_ball_overlap(ball1, ball2, dist, nx, ny);

    //Find the relative velocity across axes and normal
    //X axis relative velocity
    let xrv: f32 = ball1.x_velocity - ball2.x_velocity;
    //Y axis relative velocity
    let yrv: f32 = ball1.y_velocity - ball2.y_velocity;
    //Normal relative velocity
    let velocity_along_normal: f32 = xrv*nx + yrv*ny;
    //Breaks out of loop if balls traveling apart already
    if velocity_along_normal > 0.0 { return;}

    //Finds the impulse of the 2 balls collision to calculate the bounce
    let collision_impulse: f32 = find_collision_impulse(ball1, ball2, velocity_along_normal);

    //Make changes to the velocity from the collision
    change_velocity_from_collision(ball1, ball2, collision_impulse, nx, ny);  
}

//Control functions
fn is_mouse_inside_ball(mouse_x: f32, mouse_y: f32, ball: &mut Ball) -> bool {
    //Detection of the mouse inside of a ball

    let dx = mouse_x - ball.x;
    let dy = mouse_y - ball.y;
    (dx * dx + dy * dy) <= ball.radius*ball.radius
}
fn set_ball_to_mouse(mouse_x: f32, mouse_y: f32, ball: &mut Ball) {


    //Changes the speed that the ball reacts 0.0 for no movement 1.0 for snapping
    //Closer to 0.0 the looser it is and worse at latching on to mouse
    let follow_speed: f32 = 0.9;

    //Finds distance of ball to mouse
    let dx = mouse_x - ball.x;
    let dy = mouse_y - ball.y;

    //Scale the velocity change by the distance which will support throwing of the balls
    ball.x_velocity = dx * 0.3;
    ball.y_velocity = dy * 0.3;
    ball.x += (mouse_x - ball.x) * follow_speed;
    ball.y += (mouse_y - ball.y) * follow_speed;
}
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

        //BALL SPAWNING

        //Balls spawning once per frame when right clicked 
        //Generates a lot of balls very fast
        if is_mouse_button_down(MouseButton::Right) == true {
            balls.push(generate_ball(x, y));
        }

        
        //BALL GRABBING

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