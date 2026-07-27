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

fn border_collision(ball: &mut Ball, window: &WindowData) {
    let settle_threshold: f32 = 1.0;

    // bottom edge
    if ball.y + ball.radius > window.height {
        ball.y = window.height - ball.radius;
        ball.y_velocity *= -ball.restitution;

        // When bounce is esentially 0 then rest
        if ball.y_velocity.abs() < settle_threshold {
            ball.y_velocity = 0.0;
        }
    }

    // top edge
    if ball.y - ball.radius < 0.0 {
        ball.y = ball.radius;
        ball.y_velocity *= -ball.restitution;
    }

    // left / right edges (same idea, using x_velocity)
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
fn gravitational_acceleration (ball: &mut Ball, gravity: f32, terminal_velocity: f32) {
    if terminal_velocity <= ball.y_velocity {
                ball.y_velocity += -gravity
            }

            ball.y += -ball.y_velocity;
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
            mass: rng.gen_range(5.0..20.0),
            restitution: 0.7,
        })
        .collect()
}
fn resolve_ball_overlap(ball1: &mut Ball, ball2: &mut Ball, distance: f32, x_col_norm: f32, y_col_norm: f32) {
    //Calculate the overlap
    let overlap = (ball1.radius + ball2.radius) - distance;

    ball1.x -= overlap/2.0 * x_col_norm;
    ball1.y -= overlap/2.0 * y_col_norm;
    ball2.x += overlap/2.0 * x_col_norm;
    ball2.y += overlap/2.0 * y_col_norm;
}

#[macroquad::main("BasicShapes")]
async fn main() {
    //Modifies that gain speed
    let gravity: f32 = 0.1;
    //Max Speed post aceleration
    let terminal_velocity: f32 = -5.0;


    //Defines a example ball
    let mut balls = generate_balls(2, 800.0, 600.0);

    loop {
        clear_background(LIGHTGRAY);

        let window = WindowData {
            width: screen_width(),
            height: screen_height(),
        };
        

        for ball in balls.iter_mut() {
            gravitational_acceleration(ball, gravity, terminal_velocity);
            border_collision(ball, &window);
            horizontal_movement(ball);
            draw_circle(ball.x, ball.y, ball.radius, YELLOW);
        }

        //Checks for collisions inbetween each ball and every other ball every frame 
        for i in 0..balls.len() {
            let (left, right) = balls.split_at_mut(i+1);
            let ball_i = &mut left[i];
            for ball_j in right.iter_mut() {
                if ball_collision(ball_i, ball_j) == true {
                    //Setup for finding the collision normal

                    //X axis distance
                    let dx: f32 = ball_i.x - ball_j.x;
                    //Y axis distance
                    let dy: f32 = ball_i.y - ball_j.y;
                    //Direct line distance
                    let dist: f32 = (dx*dx+dy*dy).sqrt();
                    
                    //Calculate the collision normal

                    //X axis collision normal
                    let nx: f32 = dx/dist;
                    //Y axis collision normal
                    let ny: f32 = dy/dist;

                    //Resolve any overlap inbetween the 2 collided balls
                    resolve_ball_overlap(ball_i, ball_j, dist, nx, ny);

                    //Find the relative velocity across axes
                    
                    //X axis relative velocity
                    let xrv: f32 = ball_i.x_velocity - ball_j.x_velocity;
                    let yrv: f32 = ball_i.y_velocity - ball_j.y_velocity;
                }
                
            }
        }

        next_frame().await
    }
}