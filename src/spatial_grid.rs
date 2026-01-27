use crate::vector::Vector2D;

#[derive(Debug, Default, Clone)]
struct Tile {
    tile: Vec<usize>,
    stamp: usize,
}

pub struct SpatialGrid {
    grid: Vec<Tile>,
    grid_size: usize,
    tile_size: f32,
    hash: Vector2D<usize>,
    current_stamp: usize,
}

impl SpatialGrid {
    pub fn new(grid_size: usize, tile_size: f32, hash: Vector2D<usize>) -> Self {
        Self {
            grid: vec![
                Tile {
                    tile: Vec::new(),
                    stamp: 0
                };
                grid_size
            ],
            grid_size,
            tile_size,
            hash,
            current_stamp: 0,
        }
    }

    fn ratio_to_tile(&self, n: f32) -> usize {
        (n / self.tile_size).floor() as usize
    }

    fn pos_to_tile(&self, pos: Vector2D<f32>) -> Vector2D<usize> {
        Vector2D {
            x: self.ratio_to_tile(pos.x),
            y: self.ratio_to_tile(pos.y),
        }
    }

    fn hash_tile(&self, tile: Vector2D<usize>) -> usize {
        tile.x.wrapping_mul(self.hash.x) ^ tile.y.wrapping_mul(self.hash.y)
    }

    fn get_index(&self, hashed: usize) -> usize {
        hashed % self.grid_size
    }

    fn pos_to_index(&self, pos: Vector2D<f32>) -> usize {
        let tile = self.pos_to_tile(pos);
        let hashed = self.hash_tile(tile);
        self.get_index(hashed)
    }

    fn push_tile(&mut self, index: usize, id: usize) {
        let tile: &mut Tile = match self.grid.get_mut(index) {
            Some(tile) => tile,
            None => return,
        };
        if tile.stamp != self.current_stamp {
            tile.tile.clear();
            tile.stamp = self.current_stamp;
        }
        tile.tile.push(id);
    }

    pub fn push(&mut self, id: usize, pos: Vector2D<f32>, size: f32) {
        for y in self.ratio_to_tile(pos.y - size)..self.ratio_to_tile(pos.y + size) {
            for x in self.ratio_to_tile(pos.x - size)..self.ratio_to_tile(pos.x + size) {
                let hashed = self.hash_tile(Vector2D { x, y });
                let index = self.get_index(hashed);
                self.push_tile(index, id);
            }
        }
    }

    pub fn clear(&mut self) {
        self.current_stamp += 1;
    }

    fn get_tile(&self, index: usize) -> Option<&Vec<usize>> {
        let tile: &Tile = match self.grid.get(index) {
            Some(tile) => tile,
            None => return None,
        };
        if tile.stamp != self.current_stamp {
            None
        } else {
            Some(&tile.tile)
        }
    }
    pub fn get(&self, pos: Vector2D<f32>, size: f32) -> Vec<usize> {
        let mut sum: Vec<usize> = Vec::new();
        for y in self.ratio_to_tile(pos.y - size)..self.ratio_to_tile(pos.y + size) {
            for x in self.ratio_to_tile(pos.x - size)..self.ratio_to_tile(pos.x + size) {
                let hashed = self.hash_tile(Vector2D { x, y });
                let index = self.get_index(hashed);
                let tile = match self.get_tile(index) {
                    Some(tile) => tile,
                    None => continue,
                };
                for &id in tile {
                    if !sum.contains(&id) {
                        sum.push(id);
                    }
                }
            }
        }
        sum
    }
}
