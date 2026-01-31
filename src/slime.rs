use crate::entity::{Drawable, Entity, Inputable, Updatable};
use crate::frame::Frame;
use crate::input::Input;
use crate::spatial_grid::SpatialGrid;
use crate::vector::Vector2D;
use crate::{dot, rgb};

const X_HASH: usize = 6287364878;
const Y_HASH: usize = 2731859790;

#[derive(Default, Clone, Debug)]
struct SlimeCell {
    pos: Vector2D<f32>,
    speed: Vector2D<f32>,
    size: f32,
    fix: bool,
    neighbour: Vec<(usize, f32)>,
}

impl SlimeCell {
    fn new(x: f32, y: f32, size: f32) -> Self {
        Self {
            pos: Vector2D::new(x, y),
            speed: Vector2D { x: 0.0, y: 0.0 },
            size,
            fix: false,
            neighbour: Vec::new(),
        }
    }

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
        }

        let damping = 0.95;
        self.speed.x *= damping;
        self.speed.y *= damping;
    }

    fn update(&mut self, dt: f32) {
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
    // pinch: Option<usize>,
    grid: SpatialGrid,
}

impl Slime {
    pub fn new(
        grid_size: Vector2D<usize>,
        center: Vector2D<f32>,
        cell_radius: usize,
        cell_size: f32,
    ) -> Self {
        // create cell all around the anchor pos
        let mut cells: Vec<SlimeCell> = Vec::new();
        let origin = Vector2D {
            x: center.x.floor() as isize - cell_radius as isize,
            y: center.y.floor() as isize - cell_radius as isize,
        };

        for y in 0..(2 * cell_radius) {
            for x in 0..(2 * cell_radius) {
                cells.push(SlimeCell::new(
                    (origin.x + x as isize) as f32,
                    (origin.y + y as isize) as f32,
                    cell_size,
                ));
            }
        }

        let cell_nb = cells.len();
        for i in 0..cell_nb {
            for j in (i + 1)..cell_nb {
                let (left, right) = cells.split_at_mut(j);
                let first = &mut left[i];
                let second = &mut right[0];
                let length = first.pos.distance(second.pos);
                if length <= first.size + second.size {
                    first.neighbour.push((j, length));
                    println!("test {}, {}", i, j);
                }
            }
        }

        Self {
            cells,
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
    fn update(&mut self, dt: f32) {}
}

impl Drawable for Slime {
    fn draw(&self, frame: &mut Frame) {
        for cell in &self.cells {
            cell.draw(frame);
        }
    }
}

impl Entity for Slime {}
