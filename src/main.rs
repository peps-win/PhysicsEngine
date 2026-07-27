use ::rand::Rng;
use macroquad::{prelude::*};

struct Ball {
    x: f32,
    y: f32,
    x_velocity: f32,
    y_velocity: f32,
    radius: f32,
    mass: f32,
    restitution: f32,
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
//Handles contact with border, bouncing settling, rolling settling, and wall friction
fn border_collision(ball: &mut Ball, window: &WindowData) {
    let bounce_settle_threshold: f32 = 0.1;
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
    ball.x_velocity *= ground_friction;

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
fn ball_collision(ball1: &Ball, ball2: &Ball) -> bool {
    let dx = ball1.x - ball2.x;
    let dy = ball1.y - ball2.y;
    let radius_sum = ball1.radius + ball2.radius;
    (dx * dx + dy * dy) <= radius_sum * radius_sum
}
fn gravitational_acceleration(ball: &mut Ball, gravity: f32, terminal_velocity: f32) {
    if ball.y_velocity < terminal_velocity {
        ball.y_velocity += gravity;
    }
    ball.y += ball.y_velocity;   // no negation
}
fn horizontal_movement (ball: &mut Ball) {
    ball.x += ball.x_velocity
}
fn generate_balls(count: u64, width: f32, height: f32) -> Vec<Ball> {
    let mut rng = ::rand::thread_rng();
    (0..count) 
        .map(|_| Ball {
            x: rng.gen_range(0.0..width),
            y: rng.gen_range(0.0..height),
            x_velocity: rng.gen_range(-3.0..3.0),
            y_velocity: rng.gen_range(-3.0..3.0),
            radius: rng.gen_range(10.0..20.0),
            mass: rng.gen_range(3.0..15.0),
            restitution: 0.7,
        })
        .collect()
}
fn resolve_ball_overlap(ball1: &mut Ball, ball2: &mut Ball, distance: f32, x_col_norm: f32, y_col_norm: f32) {
    //Calculate the overlap
    let overlap = (ball1.radius + ball2.radius) - distance;

    ball1.x += overlap/2.0 * x_col_norm;
    ball1.y += overlap/2.0 * y_col_norm;
    ball2.x -= overlap/2.0 * x_col_norm;
    ball2.y -= overlap/2.0 * y_col_norm;
}
fn find_collision_impulse(ball1: &Ball, ball2: &Ball, velocity_along_normal: f32) -> f32 {
    let e: f32 = ball1.restitution*ball2.restitution;
    return -(1.0 + e) * velocity_along_normal / (1.0 / ball1.mass + 1.0 / ball2.mass);
}
fn change_velocity_from_collision(ball1: &mut Ball, ball2: &mut Ball, collision_impulse: f32, x_col_norm: f32, y_col_norm: f32) {
    ball1.x_velocity += (collision_impulse / ball1.mass) * x_col_norm;
    ball1.y_velocity += (collision_impulse / ball1.mass) * y_col_norm;
    ball2.x_velocity -= (collision_impulse / ball2.mass) * x_col_norm;
    ball2.y_velocity -= (collision_impulse / ball2.mass) * y_col_norm;
}
fn ball_collision_correction(ball1: &mut Ball, ball2: &mut Ball)  {
    //Setup for finding the collision normal
    //X axis distance
    let dx: f32 = ball1.x - ball2.x;
    //Y axis distance
    let dy: f32 = ball1.y - ball2.y;
    //Direct line distance
    let dist: f32 = (dx*dx+dy*dy).sqrt();
    
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
fn speed_decay(ball: &mut Ball) {
    ball.x_velocity *= 0.999
}

#[macroquad::main("BasicShapes")]
async fn main() {
    //Modifies that gain speed
    let gravity: f32 = 0.1;
    //Max Speed post aceleration
    let terminal_velocity: f32 = 5.0;


    //Defines a example ball
    let mut balls = generate_balls(1, 800.0, 600.0);

    loop {
        clear_background(LIGHTGRAY);

        let window = WindowData {
            width: screen_width(),
            height: screen_height(),
        };
        

        for ball in balls.iter_mut() {
            gravitational_acceleration(ball, gravity, terminal_velocity);
            horizontal_movement(ball);
            border_collision(ball, &window);
            draw_circle(ball.x, ball.y, ball.radius, YELLOW);
            speed_decay(ball);
        }

        //Checks for collisions inbetween each ball and every other ball every frame 
        for i in 0..balls.len() {
            let (left, right) = balls.split_at_mut(i+1);
            let ball_i = &mut left[i];
            for ball_j in right.iter_mut() {
                if ball_collision(ball_i, ball_j) == true {
                    ball_collision_correction(ball_i, ball_j);
                }
                
            }
        }

        next_frame().await
    }
}
