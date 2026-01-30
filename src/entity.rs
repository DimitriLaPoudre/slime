use crate::frame::Frame;
use crate::input::Input;

pub trait Inputable {
    fn handle_input(&mut self, input: Input);
}

pub trait Updatable {
    fn update(&mut self, dt: f32);
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}

pub trait Entity: Inputable + Updatable + Drawable {}
