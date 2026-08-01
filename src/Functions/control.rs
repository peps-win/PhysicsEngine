use crate::structs::structs::*;
use crate::*;

//Control functions
pub fn is_mouse_inside_ball(mouse_x: f32, mouse_y: f32, ball: &Ball) -> bool {
    //Detection of the mouse inside of a ball

    let dx = mouse_x - ball.x;
    let dy = mouse_y - ball.y;
    (dx * dx + dy * dy) <= ball.radius * ball.radius
}
pub fn set_ball_to_mouse(mouse_x: f32, mouse_y: f32, ball: &mut Ball) {
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
pub fn ball_spawning(mouse_x: f32, mouse_y: f32, balls: &mut Vec<Ball>) {
    //Balls spawning once per frame when RMB clicked
    //Generates a lot of balls very fast


    if is_mouse_button_down(MouseButton::Right) == true {
        balls.push(generate_ball(mouse_x, mouse_y));
    }
}
pub fn ball_movement(held_ball: &mut Option<usize>, mouse_x: f32, mouse_y: f32, balls: &mut Vec<Ball>) {
    //Ball movement


    //Adds any balls when mouse is pressed on it to the held_ball vector
    if is_mouse_button_pressed(MouseButton::Left) {
        //.iter() gives a  reference to each ball in Balls
        //.position() returns the position of a ball that is under the mouse
        *held_ball = balls
            .iter()
            .position(|ball| is_mouse_inside_ball(mouse_x, mouse_y, ball));
    }
    //Balls can be moved when pressed inside of
    if is_mouse_button_down(MouseButton::Left) == true {
        //The index i gives a mutable reference to the specific held ball
        if let Some(i) = held_ball {
            set_ball_to_mouse(mouse_x, mouse_y, &mut balls[*i]);
        } else {
            *held_ball = None;
        }
    }

}
pub fn ball_removal(held_ball: &mut Option<usize>, mouse_x: f32, mouse_y: f32, balls: &mut Vec<Ball>) {
    //Ball Removal on LMB + 'R'


    if is_key_down(R) && is_mouse_button_down(Left) {
        if let Some(i) = balls
            .iter()
            .position(|ball| is_mouse_inside_ball(mouse_x, mouse_y, ball))
        {
            balls.remove(i);
            *held_ball = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_ball(x: f32, y: f32, radius: f32) -> Ball {
        Ball {
            x,
            y,
            x_velocity: 0.0,
            y_velocity: 0.0,
            radius,
            mass: 1.0,
            restitution: 0.0,
            ball_color: macroquad::color::WHITE,
        }
    }

    //Testing is_mouse inside ball
    #[test]
    fn mouse_directly_ball_center() {
        let ball1 = make_ball(0.0, 0.0, 15.0);
        let mouse_x: f32 = 0.0;
        let mouse_y: f32 = 0.0;

        assert!(is_mouse_inside_ball(mouse_x, mouse_y, &ball1));
    }
    #[test]
    fn mouse_slightly_off_center() {
        let ball1 = make_ball(0.0, 0.0, 15.0);
        let mouse_x: f32 = 5.0;
        let mouse_y: f32 = 5.0;

        assert!(is_mouse_inside_ball(mouse_x, mouse_y, &ball1));
    }
    #[test]
    fn mouse_directly_on_edge() {
        let ball1 = make_ball(0.0, 0.0, 10.0);
        let mouse_x: f32 = 5.0;
        let mouse_y: f32 = 5.0;

        assert!(is_mouse_inside_ball(mouse_x, mouse_y, &ball1));
    }
    #[test]
    fn mouse_way_off_ball() {
        let ball1 = make_ball(0.0, 0.0, 15.0);
        let mouse_x: f32 = 30.0;
        let mouse_y: f32 = 30.0;

        assert!(!is_mouse_inside_ball(mouse_x, mouse_y, &ball1));
    }
}