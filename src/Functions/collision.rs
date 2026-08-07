use crate::structs::structs::*;
use crate::*;

//Ball collision functions
pub fn ball_collision_detection(ball1: &Ball, ball2: &Ball) -> bool {
    //Detection of a collision between balls

    let dx = ball1.x - ball2.x;
    let dy = ball1.y - ball2.y;
    let radius_sum = ball1.radius + ball2.radius;
    (dx * dx + dy * dy) <= radius_sum * radius_sum
}
pub fn resolve_ball_overlap(
    ball1: &mut Ball,
    ball2: &mut Ball,
    distance: f32,
    x_col_norm: f32,
    y_col_norm: f32,
) {
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
pub fn find_collision_impulse(ball1: &Ball, ball2: &Ball, velocity_along_normal: f32) -> f32 {
    //Gets the collision impulse to calculate the amount of bounce from balls

    let e: f32 = ball1.restitution * ball2.restitution;
    return -(1.0 + e) * velocity_along_normal / (1.0 / ball1.mass + 1.0 / ball2.mass);
}
pub fn change_velocity_from_collision(
    ball1: &mut Ball,
    ball2: &mut Ball,
    collision_impulse: f32,
    x_col_norm: f32,
    y_col_norm: f32,
) {
    //Uses the collision impulse to change the ball's velocity

    ball1.x_velocity += (collision_impulse / ball1.mass) * x_col_norm;
    ball1.y_velocity += (collision_impulse / ball1.mass) * y_col_norm;
    ball2.x_velocity -= (collision_impulse / ball2.mass) * x_col_norm;
    ball2.y_velocity -= (collision_impulse / ball2.mass) * y_col_norm;
}
pub fn ball_collision_correction(ball1: &mut Ball, ball2: &mut Ball) {
    //Uses all past ball collision functions to get the numbers to plug into the change_velocity_from_collision function

    //Setup for finding the collision normal
    //X axis distance
    let dx: f32 = ball1.x - ball2.x;
    //Y axis distance
    let dy: f32 = ball1.y - ball2.y;
    //Direct line distance
    let dist: f32 = (dx * dx + dy * dy).sqrt().max(0.0001);

    //Calculate the collision normal
    //X axis collision normal
    let nx: f32 = dx / dist;
    //Y axis collision normal
    let ny: f32 = dy / dist;

    //Resolve any overlap inbetween the 2 collided balls
    resolve_ball_overlap(ball1, ball2, dist, nx, ny);

    //Find the relative velocity across axes and normal
    //X axis relative velocity
    let xrv: f32 = ball1.x_velocity - ball2.x_velocity;
    //Y axis relative velocity
    let yrv: f32 = ball1.y_velocity - ball2.y_velocity;
    //Normal relative velocity
    let velocity_along_normal: f32 = xrv * nx + yrv * ny;
    //Breaks out of loop if balls traveling apart already
    if velocity_along_normal > 0.0 {
        return;
    }

    //Finds the impulse of the 2 balls collision to calculate the bounce
    let collision_impulse: f32 = find_collision_impulse(ball1, ball2, velocity_along_normal);

    //Make changes to the velocity from the collision
    change_velocity_from_collision(ball1, ball2, collision_impulse, nx, ny);
}
pub fn find_surrounding_balls(selected_ball: &Ball, balls: &Vec<Ball>) -> Vec<u32> {
    let mut surrounding_balls: Vec<u32> = Vec::new();
    
    //Go through every ball in balls and calculate the distance between the selected ball and them
    for (i, ball) in balls.iter().enumerate() {
        //Setup for finding the collision normal
        //X axis distance
        let dx: f32 = ball.x - selected_ball.x;
        //Y axis distance
        let dy: f32 = ball.y - selected_ball.y;
        //Direct line distance
        let dist: f32 = (dx * dx + dy * dy).sqrt().max(0.0001);

        //Pushes the ball to the Vector of surrounding balls if its radius is touching plus a margin
        if dist <= selected_ball.radius + ball.radius + 10.0 {
            surrounding_balls.push(i as u32);
        }
    }
    surrounding_balls
}
pub fn resolve_all_collisions(balls: &mut Vec<Ball>) {
    //Pulls all collision code together into one function that can be called to resolve collisions among every ball
    
    
    let fps: i32 = get_fps();
            let iterations: i32 = (fps / 2) * 3;

            for _ in 0..iterations {
                for i in 0..balls.len() {

                    let (left, right) = balls.split_at_mut(i + 1);
                    let ball_i = &mut left[i];
                    for ball_j in right.iter_mut() {
                        if ball_collision_detection(ball_i, ball_j) == true {
                            ball_collision_correction(ball_i, ball_j);
                        }
                    }
                }
            }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_ball(x: f32, y: f32, radius: f32, restitution: f32) -> Ball {
        Ball {
            x,
            y,
            x_velocity: 0.0,
            y_velocity: 0.0,
            radius,
            mass: 1.0,
            restitution,
            ball_color: macroquad::color::WHITE,
        }
    }
    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.0001
    }
    
    //Testing for the ball_collision function
    #[test]
    fn balls_overlapping_true() {
        let ball1 = make_ball(0.0, 0.0, 10.0, 0.0);
        let ball2 = make_ball(0.0, 5.0, 10.0, 0.0);

        assert!(ball_collision_detection(&ball1, &ball2));
    }
    #[test]
    fn balls_overlapping_false() {
        let ball1 = make_ball(0.0, 0.0, 10.0, 0.0);
        let ball2 = make_ball(0.0, 25.0, 10.0, 0.0);

        assert!(!ball_collision_detection(&ball1, &ball2));
    }
    #[test]
    fn balls_exact_same_position_true() {
        let ball1 = make_ball(0.0, 0.0, 10.0, 0.0);
        let ball2 = make_ball(0.0, 0.0, 10.0, 0.0);

        assert!(ball_collision_detection(&ball1, &ball2));
    }

    //Testing for the find_collision_impulse function
    #[test]
    fn collision_impulse_equals_mass_and_restitution() {
        let ball1 = make_ball(0.0, 0.0, 10.0, 1.0); // mass 1.0, restitution baked into helper — adjust as needed
        let ball2 = make_ball(0.0, 5.0, 10.0, 1.0);

        // e = 1.0 * 1.0 = 1.0
        // impulse = -(1.0 + 1.0) * 5.0 / (1.0/1.0 + 1.0/1.0) = -2.0 * 5.0 / 2.0 = -5.0
        let impulse = find_collision_impulse(&ball1, &ball2, 5.0);
        assert_eq!(impulse, -5.0);
    } 
    #[test]
    fn find_collision_impulse_zero_velocity_is_zero() {
        let ball1 = make_ball(0.0, 0.0, 10.0, 0.8);
        let ball2 = make_ball(0.0, 5.0, 10.0, 0.8);

        let impulse = find_collision_impulse(&ball1, &ball2, 0.0);
        assert_eq!(impulse, 0.0);
    }
    #[test]
    fn find_collision_impulse_perfectly_elastic() {
        let ball1 = make_ball(0.0, 0.0, 10.0, 1.0);
        let ball2 = make_ball(0.0, 5.0, 10.0, 1.0);

        // e = 1.0 * 1.0 = 1.0
        // impulse = -(1.0 + 1.0) * 4.0 / (1.0/1.0 + 1.0/1.0) = -2.0 * 4.0 / 2.0 = -4.0
        let impulse = find_collision_impulse(&ball1, &ball2, 4.0);
        assert_eq!(impulse, -4.0);
    }
    #[test]
    fn find_collision_impulse_perfectly_inelastic() {
        let ball1 = make_ball(0.0, 0.0, 10.0, 0.0);
        let ball2 = make_ball(0.0, 5.0, 10.0, 0.0);

        // e = 0.0 * 0.0 = 0.0
        // impulse = -(1.0 + 0.0) * 4.0 / (1.0/1.0 + 1.0/1.0) = -1.0 * 4.0 / 2.0 = -2.0
        let impulse = find_collision_impulse(&ball1, &ball2, 4.0);
        assert_eq!(impulse, -2.0);
    }
    #[test]
    fn find_collision_impulse_unequal_mass() {
        let ball1 = make_ball(0.0, 0.0, 10.0, 0.8);
        let mut ball2 = make_ball(0.0, 5.0, 10.0, 0.8);
        ball2.mass = 3.0;

        let impulse = find_collision_impulse(&ball1, &ball2, 5.0);
        assert!(approx_eq(impulse, -6.15));
    }
    #[test]
    fn find_collision_impulse_negative_velocity() {
        let ball1 = make_ball(0.0, 0.0, 10.0, 0.8);
        let ball2 = make_ball(0.0, 5.0, 10.0, 0.8);

        // e = 0.64
        // impulse = -(1.64) * (-3.0) / (1.0 + 1.0) = 4.92 / 2.0 = 2.46
        let impulse = find_collision_impulse(&ball1, &ball2, -3.0);
        assert_eq!(impulse, 2.46);
    }
}