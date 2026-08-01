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