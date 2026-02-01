use crate::entity::{Drawable, Entity, Inputable, Updatable};
use crate::frame::Frame;
use crate::input::Input;
use crate::rgb;
use crate::spatial_grid::SpatialGrid;
use crate::vector::Vector2D;

const X_HASH: usize = 6287364878;
const Y_HASH: usize = 2731859790;

#[derive(Default, Clone, Debug)]
struct SlimeCell {
    pos: Vector2D<f32>,
    speed: Vector2D<f32>,
    size: f32,
    fix: bool,
    links: Vec<(usize, f32)>,
}

impl SlimeCell {
    fn new(x: f32, y: f32, size: f32) -> Self {
        Self {
            pos: Vector2D::new(x, y),
            speed: Vector2D { x: 0.0, y: 0.0 },
            size,
            fix: false,
            links: Vec::new(),
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
        if !self.fix {
            self.pos.x += self.speed.x * dt;
            self.pos.y += self.speed.y * dt;
        }
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
    pinch: Option<usize>,
    grid: SpatialGrid,
    recovery_speed: usize,
    radius: f32,
}

impl Slime {
    pub fn new(
        grid_size: Vector2D<usize>,
        center: Vector2D<f32>,
        cell_size: f32,
        rigidity: f32,
        recovery_speed: usize,
        radius: f32,
    ) -> Self {
        let mut cells: Vec<SlimeCell> = Vec::new();
        let origin = Vector2D {
            x: center.x.floor() as isize - radius as isize,
            y: center.y.floor() as isize - radius as isize,
        };

        for offset_y in 0..(2 * radius as usize) {
            for offset_x in 0..(2 * radius as usize) {
                let x = (origin.x + offset_x as isize) as f32;
                let y = (origin.y + offset_y as isize) as f32;
                let dx = x - center.x;
                let dy = y - center.y;

                if dx * dx + dy * dy <= radius as f32 * radius as f32 {
                    cells.push(SlimeCell::new(
                        (origin.x + offset_x as isize) as f32,
                        (origin.y + offset_y as isize) as f32,
                        cell_size,
                    ));
                }
            }
        }

        let mut center = Vector2D::default();
        for cell in &cells {
            center = center.add(cell.pos);
        }
        center = center.vdiv(cells.len() as f32);

        let mut avg_radius = 0.0;
        for cell in &cells {
            avg_radius += cell.pos.sub(center).length();
        }
        avg_radius /= cells.len() as f32;

        let cell_nb = cells.len();
        for i in 0..cell_nb {
            for j in (i + 1)..cell_nb {
                let (left, right) = cells.split_at_mut(j);
                let first = &mut left[i];
                let second = &mut right[0];
                let length = first.pos.delta(second.pos).length();
                if length <= (first.size * 2.0) * rigidity {
                    first.links.push((j, length));
                }
            }
        }

        Self {
            cells,
            pinch: None,
            grid: SpatialGrid::new(
                grid_size.x * grid_size.y,
                cell_size,
                Vector2D {
                    x: X_HASH,
                    y: Y_HASH,
                },
            ),
            recovery_speed,
            radius: avg_radius,
        }
    }

    fn set_pinch(&mut self, i: Option<usize>) {
        if let Some(n) = self.pinch {
            self.cells[n].fix = false;
        }
        match i {
            Some(n) => {
                if n < self.cells.len() {
                    self.pinch = Some(n);
                    self.cells[n].fix = true;
                }
            }
            None => self.pinch = None,
        }
    }

    fn pinch_cell(&mut self) -> Option<&mut SlimeCell> {
        self.pinch.and_then(|i| self.cells.get_mut(i))
    }
}

impl Inputable for Slime {
    fn handle_input(&mut self, input: Input) {
        if input.mouse.left {
            match self.pinch_cell() {
                Some(cell) => cell.pos = input.mouse.pos,
                None => {
                    let list = self.grid.get(input.mouse.pos, 0.1);
                    self.set_pinch(None);
                    for id in list {
                        let cell = &self.cells[id];
                        if cell.pos.delta(input.mouse.pos).length() < cell.size {
                            self.set_pinch(Some(id));
                            break;
                        }
                    }
                }
            }
        } else {
            self.set_pinch(None);
        }
    }
}

impl Slime {
    fn solve_length_links(&mut self) {
        let links: Vec<(usize, usize, f32)> = self
            .cells
            .iter()
            .enumerate()
            .flat_map(|(i, cell)| {
                cell.links
                    .iter()
                    .map(move |(j, rest_length)| (i, *j, *rest_length))
            })
            .collect();
        for (first, second, rest_length) in links {
            let (a, b) = if first < second {
                let (left, right) = self.cells.split_at_mut(second);
                (&mut left[first], &mut right[0])
            } else {
                let (left, right) = self.cells.split_at_mut(first);
                (&mut right[0], &mut left[second])
            };

            let delta = a.pos.delta(b.pos);
            let dist = delta.length();

            if dist == 0.0 {
                continue;
            }

            let diff = (dist - rest_length) / dist;
            let correction = delta.vmul(0.5 * diff);

            if !a.fix {
                a.pos = a.pos.add(correction);
            }
            if !b.fix {
                b.pos = b.pos.sub(correction);
            }
        }
    }

    fn solve_area(&mut self) {
        let mut center = Vector2D::default();
        for cell in &self.cells {
            center = center.add(cell.pos);
        }
        center = center.vdiv(self.cells.len() as f32);

        let mut avg_radius = 0.0;
        for cell in &self.cells {
            avg_radius += cell.pos.sub(center).length();
        }
        avg_radius /= self.cells.len() as f32;

        let compression = self.radius - avg_radius;
        let stiffness = 0.2;

        for cell in &mut self.cells {
            let dir = cell.pos.sub(center).normalize();
            cell.pos = cell.pos.add(dir.vmul(compression).vmul(stiffness));
        }
    }
}

impl Updatable for Slime {
    fn update(&mut self, dt: f32) {
        for (id, cell) in self.cells.iter_mut().enumerate() {
            cell.update(dt);
            self.grid.push(id, cell.pos, cell.size);
        }

        for _ in 0..self.recovery_speed {
            self.solve_length_links();
            self.solve_area();
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
