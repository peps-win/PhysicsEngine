use macroquad::prelude::*;
struct Ball {
    x: f32,
    y: f32,
    x_velocity: f32,
    y_velocity: f32,
    radius: f32,
    restitution: f32,
}
struct WindowData {
    width: f32,
    height: f32,
}

fn border_collision(ball: &mut Ball, window: &WindowData, gravity: f32) {
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
fn gravitational_acceleration (ball: &mut Ball, gravity: f32, TerminalVelocity: f32) {
    if TerminalVelocity <= ball.y_velocity {
                ball.y_velocity += -gravity
            }

            ball.y += -ball.y_velocity;
}
fn horizontal_movement (ball: &mut Ball) {
    ball.x += ball.x_velocity
}

#[macroquad::main("BasicShapes")]
async fn main() {
    //Modifies that gain speed
    let gravity: f32 = 0.1;
    //Max Speed post aceleration
    let TerminalVelocity: f32 = -5.0;

    //Defines a example ball
    let mut ball1 = Ball {
        x: 100.0,
        y: 0.0,
        x_velocity: -1.0,
        y_velocity: -1.0,
        radius: 15.0,
        restitution: 0.7,
    };

    loop {
        clear_background(LIGHTGRAY);

        let mut window1 = WindowData {
            width: screen_width(),
            height: screen_height(),
        };
        
        draw_circle(ball1.x, ball1.y, ball1.radius, YELLOW);

        gravitational_acceleration (&mut ball1, gravity, TerminalVelocity);
        border_collision(&mut ball1, &window1, gravity);
        horizontal_movement(&mut ball1);
        next_frame().await
    }
}