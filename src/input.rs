use crate::vector::Vector2D;
use minifb::{MouseButton, MouseMode, Window};

#[derive(Default, Copy, Clone, Debug)]
pub struct Mouse {
    pub pos: Vector2D<f32>,
    pub abs_pos: Vector2D<f32>,
    pub left: bool,
    pub middle: bool,
    pub right: bool,
}

impl Mouse {
    fn refresh(&mut self, window: &Window) {
        if let Some(pos) = window.get_mouse_pos(MouseMode::Clamp) {
            self.pos.x = pos.0;
            self.pos.y = pos.1;
        }
        if let Some(pos) = window.get_mouse_pos(MouseMode::Pass) {
            self.abs_pos.x = pos.0;
            self.abs_pos.y = pos.1;
        }
        self.left = window.get_mouse_down(MouseButton::Left);
        self.middle = window.get_mouse_down(MouseButton::Middle);
        self.right = window.get_mouse_down(MouseButton::Right);
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Input {
    pub mouse: Mouse,
}

impl Input {
    pub fn refresh(&mut self, window: &Window) {
        self.mouse.refresh(window);
    }
}
