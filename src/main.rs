mod core;
mod entity;
mod frame;
mod input;
mod macros;
mod slime;
mod spatial_grid;
mod vector;

use core::Core;
use slime::Slime;
use vector::Vector2D;

const TITLE: &str = "slime";
const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const REFRESH: usize = 120;
const FPS: usize = 60;

fn main() {
    let slime = Slime::new(
        Vector2D {
            x: WIDTH,
            y: HEIGHT,
        },
        Vector2D::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0),
        900,
        1.0,
    );
    let mut core = Core::new(TITLE, WIDTH, HEIGHT, REFRESH, FPS);
    core.add_entity(slime);

    while core.is_open() {
        core.analyze_event();
        core.update();
        core.draw();
        core.next_frame();
    }
}
