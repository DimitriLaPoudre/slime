use crate::entity::{Drawable, Entity, Inputable, Updatable};
use crate::frame::Frame;
use crate::input::Input;
use crate::rgb;
use crate::vector::Vector2D;

#[derive(Default, Copy, Clone, Debug)]
struct SlimeCell {
    pos: Vector2D<f32>,
    speed: Vector2D<f32>,
}

impl SlimeCell {
    fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Vector2D::new(x, y),
            speed: Vector2D { x: 0.0, y: 0.0 },
        }
    }

    fn reset_speed(&mut self) {
        self.speed.x = 0.0;
        self.speed.y = 0.0;
    }

    fn handle_input(&mut self, input: Input) {}

    fn apply_gravity(&mut self, anchor: Vector2D<f32>) {
        let dx = anchor.x - self.pos.x;
        let dy = anchor.y - self.pos.y;

        let max_dist = 200.0;

        let dist_sq = dx * dx + dy * dy + 0.000001;
        let dist = dist_sq.sqrt().max(max_dist);

        let exponent = 1.4;
        // let min_a = 5.0;
        // let max_a = 100.0;
        // let a = min_a + (max_a - min_a) * (dist / max_dist);
        let a = (dist.powf(exponent));
        let ax = a * dx / dist;
        let ay = a * dy / dist;
        self.speed.x = ax;
        self.speed.y = ay;
    }

    fn update(&mut self, anchor: Vector2D<f32>, dt: f32) {
        self.apply_gravity(anchor);
        self.pos.x += self.speed.x * dt;
        self.pos.y += self.speed.y * dt;
    }
}

impl Drawable for SlimeCell {
    fn draw(&self, frame: &mut Frame) {
        if self.pos.x >= 0.0
            && self.pos.y >= 0.0
            && (self.pos.x as usize) < frame.width
            && (self.pos.y as usize) < frame.height
        {
            frame.buffer[self.pos.y as usize * frame.width + self.pos.x as usize] = rgb!(0, 255, 0);
        }
    }
}

pub struct Slime {
    cells: Vec<SlimeCell>,
    anchor: Vector2D<f32>,
    pinch: Option<usize>,
}

impl Slime {
    pub fn new(anchor: Vector2D<f32>, size: usize) -> Self {
        Self {
            cells: vec![SlimeCell::new(0.0, 0.0); size],
            anchor,
            pinch: None,
        }
    }

    fn set_pinch(&mut self, i: Option<usize>) {
        match i {
            Some(n) => {
                if n < self.cells.len() {
                    self.pinch = Some(n);
                }
            }
            None => self.pinch = None,
        }
    }

    fn pinch_cell(&self) -> Option<&SlimeCell> {
        self.pinch.and_then(|i| self.cells.get(i))
    }
}

impl Inputable for Slime {
    fn handle_input(&mut self, input: Input) {
        if input.mouse.left {
            self.anchor = input.mouse.pos;
            for cell in &mut self.cells {
                cell.reset_speed();
            }
            // match self.pinch_cell() {
            //     Some(cell) => {}
            //     None => {
            //         // self.set_pinch(Some(input.mouse.pos.0, input.mouse.pos.1));
            //     }
            // }
        } else {
        }
    }
}

impl Updatable for Slime {
    fn update(&mut self) {
        for cell in &mut self.cells {
            cell.update(self.anchor, 1.0 / 240.0);
        }
    }
}

impl Drawable for Slime {
    fn draw(&self, frame: &mut Frame) {
        for cell in &self.cells {
            cell.draw(frame);
        }
    }
}

impl Entity for Slime {}
