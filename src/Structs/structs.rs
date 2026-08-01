use macroquad::color::Color;

pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub x_velocity: f32,
    pub y_velocity: f32,
    pub radius: f32,
    pub mass: f32,
    pub restitution: f32,
    pub ball_color: Color,
}
pub struct WindowData {
    pub width: f32,
    pub height: f32,
}
