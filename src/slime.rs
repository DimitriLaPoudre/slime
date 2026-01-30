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
    fix: bool,
}

impl SlimeCell {
    fn new(x: f32, y: f32, size: f32) -> Self {
        Self {
            pos: Vector2D::new(x, y),
            speed: Vector2D { x: 0.0, y: 0.0 },
            size,
            fix: false,
        }
    }

    fn with_fix(mut self) -> Self {
        self.fix = true;
        self
    }

    // fn reset_speed(&mut self) {
    //     self.speed.x = 0.0;
    //     self.speed.y = 0.0;
    // }

    fn apply_gravity(&mut self, anchor: Vector2D<f32>, dt: f32) {
        if !self.fix {
            // simulate simple gravity
            let dir = Vector2D {
                x: anchor.x - self.pos.x,
                y: anchor.y - self.pos.y,
            };

            let dist_sq: f32 = dir.x * dir.x + dir.y * dir.y;

            let a = 1000.0;
            if dist_sq != 0.0 {
                let dist = dist_sq.sqrt();
                let normale = Vector2D {
                    x: dir.x / dist,
                    y: dir.y / dist,
                };
                let velocity = Vector2D {
                    x: normale.x * a,
                    y: normale.y * a,
                };
                self.speed.x += velocity.x * dt;
                self.speed.y += velocity.y * dt;
            }

            // let dx = anchor.x - self.pos.x;
            // let dy = anchor.y - self.pos.y;
            //
            // let dist_sq = dx * dx + dy * dy;
            // let dist = dist_sq.sqrt();
            //
            // let exponent: f32 = 2.1;
            // let a = dist.powf(exponent); // a = dist ^ exponent
            //
            // if dist == 0.0 {
            //     self.speed.x = a * 0.5 * dt;
            //     self.speed.y = a * 0.5 * dt;
            // } else {
            //     self.speed.x = a * (dx / dist) * dt;
            //     self.speed.y = a * (dy / dist) * dt;
            // }
        }

        let damping = 0.95;
        self.speed.x *= damping;
        self.speed.y *= damping;
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
            if !other.fix {
                let correction = overlap * 0.5;

                self.pos.x -= nx * correction;
                self.pos.y -= ny * correction;

                other.pos.x += nx * correction;
                other.pos.y += ny * correction;
            } else {
                let correction = overlap;

                self.pos.x -= nx * correction;
                self.pos.y -= ny * correction;
            }

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
            if self.fix {
                frame.buffer[self.pos.y as usize * frame.width + self.pos.x as usize] =
                    rgb!(255, 0, 0);
            } else {
                frame.buffer[self.pos.y as usize * frame.width + self.pos.x as usize] =
                    rgb!(0, 255, 0);
            }
        }
    }
}

pub struct Slime {
    cells: Vec<SlimeCell>,
    anchor: SlimeCell,
    // pinch: Option<usize>,
    grid: SpatialGrid,
}

impl Slime {
    pub fn new(
        grid_size: Vector2D<usize>,
        anchor: Vector2D<f32>,
        mut cell_nb: usize,
        cell_size: f32,
    ) -> Self {
        // create cell all around the anchor pos
        cell_nb -= 1;
        let mut cells: Vec<SlimeCell> = Vec::new();
        let radius = (cell_nb as f32).sqrt().ceil() / 2.0;
        let mut cell_count = 0;
        for y in (anchor.y - radius * cell_size) as usize..(anchor.y + radius * cell_size) as usize
        {
            for x in
                (anchor.x - radius * cell_size) as usize..(anchor.x + radius * cell_size) as usize
            {
                if x == anchor.x.floor() as usize && y == anchor.y.floor() as usize {
                    continue;
                } else if cell_count < cell_nb {
                    cells.push(SlimeCell::new(x as f32, y as f32, cell_size));
                    cell_count += 1;
                } else {
                    break;
                }
            }
            if cell_count >= cell_nb {
                break;
            }
        }

        Self {
            cells,
            anchor: SlimeCell::new(anchor.x, anchor.y, cell_size).with_fix(),
            // pinch: None,
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

    fn valid_cell(&self, index: usize) -> bool {
        if index == 0 {
            true
        } else {
            if let Some(_) = self.cells.get(index - 1) {
                true
            } else {
                false
            }
        }
    }

    fn get_two_cell(
        &mut self,
        mut index1: usize,
        mut index2: usize,
    ) -> Option<(&mut SlimeCell, &mut SlimeCell)> {
        if index1 == index2 {
            None
        } else if index1 == 0 {
            index2 -= 1;
            if let Some(cell2) = self.cells.get_mut(index2) {
                Some((&mut self.anchor, cell2))
            } else {
                None
            }
        } else if index2 == 0 {
            index1 -= 1;
            if let Some(cell1) = self.cells.get_mut(index1) {
                Some((cell1, &mut self.anchor))
            } else {
                None
            }
        } else {
            index1 -= 1;
            index2 -= 1;
            if index1 >= index2 {
                let (left, right) = self.cells.split_at_mut(index1);

                let cell1 = if let Some(cell1) = right.get_mut(0) {
                    cell1
                } else {
                    return None;
                };
                let cell2 = if let Some(cell2) = left.get_mut(index2) {
                    cell2
                } else {
                    return None;
                };

                Some((cell1, cell2))
            } else {
                let (left, right) = self.cells.split_at_mut(index2);

                let cell1 = if let Some(cell1) = left.get_mut(index1) {
                    cell1
                } else {
                    return None;
                };
                let cell2 = if let Some(cell2) = right.get_mut(0) {
                    cell2
                } else {
                    return None;
                };

                Some((cell1, cell2))
            }
        }
    }

    // fn set_pinch(&mut self, i: Option<usize>) {
    //     match i {
    //         Some(n) => {
    //             if n < self.cells.len() {
    //                 self.pinch = Some(n);
    //             }
    //         }
    //         None => self.pinch = None,
    //     }
    // }
    //
    // fn pinch_cell(&self) -> Option<&SlimeCell> {
    //     self.pinch.and_then(|i| self.cells.get(i))
    // }
}

impl Inputable for Slime {
    fn handle_input(&mut self, input: Input) {
        if input.mouse.left {
            self.anchor.pos = Vector2D {
                x: input.mouse.pos.x,
                y: input.mouse.pos.y,
            };

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
    fn update(&mut self, dt: f32) {
        self.grid.clear();
        self.anchor.update(self.anchor.pos, dt);
        self.grid.push(0, self.anchor.pos, self.anchor.size);
        for (i, cell) in &mut self.cells.iter_mut().enumerate() {
            cell.update(self.anchor.pos, dt);
            self.grid.push(i + 1, cell.pos, cell.size);
        }
        for i in 0..self.cells.len() {
            let collisions = self.grid.get(self.cells[i].pos, self.cells[i].size);
            for collision in collisions {
                if collision <= i + 1 && collision != 0 {
                    continue;
                }

                if !self.valid_cell(collision) {
                    continue;
                }

                let (cell, other) = self.get_two_cell(i + 1, collision).unwrap();

                cell.resolve_collision(other);
            }
        }
    }
}

impl Drawable for Slime {
    fn draw(&self, frame: &mut Frame) {
        self.anchor.draw(frame);
        for cell in &self.cells {
            cell.draw(frame);
        }
    }
}

impl Entity for Slime {}
