mod core;
mod entity;
mod frame;
mod input;
mod macros;
mod slime;
mod vector;

use core::Core;
use slime::Slime;
use vector::Vector2D;

const TITLE: &str = "slime";
const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const REFRESH: usize = 240;
const FPS: usize = 60;

fn main() {
    let mut core = Core::new(TITLE, WIDTH, HEIGHT, REFRESH, FPS);
    core.add_entity(Slime::new(Vector2D::new(200.0, 300.0), 1));

    while core.is_open() {
        core.analyze_event();
        core.update();
        core.draw();
        core.next_frame();
    }
}
