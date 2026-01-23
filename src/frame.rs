#[derive(Default)]
pub struct Frame {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
}

impl Frame {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0u32; width * height],
        }
    }

    pub fn fill(&mut self, color: u32) {
        for pixel in &mut self.buffer {
            *pixel = color;
        }
    }
}
