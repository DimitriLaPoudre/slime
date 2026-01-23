use crate::entity::Entity;
use crate::frame::Frame;
use crate::input::Input;
use crate::rgb;
use minifb::{Window, WindowOptions};

pub struct Core {
    frame: Frame,
    window: Window,
    refresh: usize,
    fps: usize,
    next_frame: usize,
    input: Input,
    entities: Vec<Box<dyn Entity>>,
}

impl Core {
    pub fn new(title: &str, width: usize, height: usize, refresh: usize, fps: usize) -> Self {
        let mut window = Window::new(title, width, height, WindowOptions::default())
            .expect("Frame::new(): Window::new failed");

        window.set_target_fps(refresh);

        Self {
            frame: Frame::new(width, height),
            window,
            refresh,
            fps,
            next_frame: 0,
            input: Input::default(),
            entities: vec![],
        }
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn add_entity<E: Entity + 'static>(&mut self, entity: E) {
        self.entities.push(Box::new(entity))
    }

    pub fn analyze_event(&mut self) {
        self.input.refresh(&self.window);
        for entity in &mut self.entities {
            entity.handle_input(self.input);
        }
    }

    pub fn update(&mut self) {
        for entity in &mut self.entities {
            entity.update();
        }
    }

    pub fn draw(&mut self) {
        self.frame.fill(rgb!(0, 0, 0));
        for entity in &self.entities {
            entity.draw(&mut self.frame);
        }
    }

    pub fn next_frame(&mut self) {
        if self.next_frame >= self.refresh / self.fps {
            self.next_frame = 0;
            self.window
                .update_with_buffer(&self.frame.buffer, self.frame.width, self.frame.height)
                .expect("Failed to update buffer");
        } else {
            self.next_frame += 1;
            self.window.update();
        }
    }
}
