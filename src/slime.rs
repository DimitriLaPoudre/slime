use crate::entity::{Drawable, Entity, Inputable, Updatable};
use crate::frame::Frame;
use crate::input::Input;
use crate::rgb;
use crate::spatial_grid::SpatialGrid;
use crate::vector::Vector2D;

const X_HASH: usize = 6287364878;
const Y_HASH: usize = 2731859790;

#[derive(Default, Copy, Clone, Debug)]
struct SlimeCell {
    pos: Vector2D<f32>,
    speed: Vector2D<f32>,
    size: f32,
}

impl SlimeCell {
    fn new(x: f32, y: f32, size: f32) -> Self {
        Self {
            pos: Vector2D::new(x, y),
            speed: Vector2D { x: 0.0, y: 0.0 },
            size,
        }
    }

    fn reset_speed(&mut self) {
        self.speed.x = 0.0;
        self.speed.y = 0.0;
    }

    fn resolve_collision(&mut self, other: &mut SlimeCell) {
        let dx = other.pos.x - self.pos.x;
        let dy = other.pos.y - self.pos.y;

        let dist_sq = dx * dx + dy * dy;

        let (nx, ny, dist) = if dist_sq == 0.0 {
            (1.0, 1.0, 0.0)
        } else {
            let d = dist_sq.sqrt();
            (dx / d, dy / d, d)
        };

        let r_sum = self.size + other.size;
        let overlap = r_sum - dist;

        if overlap > 0.0 {
            let correction = overlap * 0.5;

            self.pos.x -= nx * correction;
            self.pos.y -= ny * correction;

            other.pos.x += nx * correction;
            other.pos.y += ny * correction;

            // let v1n = self.speed.x * nx + self.speed.y * ny;
            // let v2n = other.speed.x * nx + other.speed.y * ny;
            //
            // if v1n > 0.0 {
            //     self.speed.x -= v1n * nx;
            //     self.speed.y -= v1n * ny;
            // }
            //
            // if v2n < 0.0 {
            //     other.speed.x -= v2n * nx;
            //     other.speed.y -= v2n * ny;
            // }
        }
    }

    fn apply_gravity(&mut self, anchor: Vector2D<f32>, dt: f32) {
        let dx = anchor.x - self.pos.x;
        let dy = anchor.y - self.pos.y;

        let dist_sq = dx * dx + dy * dy;
        let dist = dist_sq.sqrt();

        let exponent: f32 = 2.1;
        let a = dist.powf(exponent); // a = dist ^ exponent

        if dist == 0.0 {
            self.speed.x = a * 0.5 * dt;
            self.speed.y = a * 0.5 * dt;
        } else {
            self.speed.x = a * (dx / dist) * dt;
            self.speed.y = a * (dy / dist) * dt;
        }
    }

    fn update(&mut self, anchor: Vector2D<f32>, dt: f32) {
        self.apply_gravity(anchor, dt);
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
    grid: SpatialGrid,
}

impl Slime {
    pub fn new(
        grid_size: Vector2D<usize>,
        anchor: Vector2D<f32>,
        cell_nb: usize,
        cell_size: f32,
    ) -> Self {
        let mut cells: Vec<SlimeCell> = Vec::new();
        let radius = (cell_nb as f32).sqrt().ceil() / 2.0;
        for y in (anchor.y - radius * cell_size) as usize..(anchor.y + radius * cell_size) as usize
        {
            for x in
                (anchor.x - radius * cell_size) as usize..(anchor.x + radius * cell_size) as usize
            {
                cells.push(SlimeCell::new(x as f32, y as f32, cell_size));
            }
        }
        Self {
            // cells: vec![SlimeCell::new(anchor.x, anchor.y, cell_size); cell_nb],
            cells,
            anchor,
            pinch: None,
            grid: SpatialGrid::new(
                grid_size.x * grid_size.y,
                cell_size,
                Vector2D {
                    x: X_HASH,
                    y: Y_HASH,
                },
            ),
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
        self.grid.clear();
        for (i, cell) in &mut self.cells.iter_mut().enumerate() {
            cell.update(self.anchor, 1.0 / 120.0);
            self.grid.push(i, cell.pos, cell.size);
        }
        for i in 0..self.cells.len() {
            let collisions = self.grid.get(self.cells[i].pos, self.cells[i].size);
            for collision in collisions {
                if collision <= i {
                    continue;
                }
                if let None = self.cells.get(collision) {
                    continue;
                }
                let (left, right) = self.cells.split_at_mut(collision);

                let cell = match left.get_mut(i) {
                    Some(cell) => cell,
                    None => continue,
                };
                let other = match right.get_mut(0) {
                    Some(other) => other,
                    None => continue,
                };
                cell.resolve_collision(other);
            }
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
