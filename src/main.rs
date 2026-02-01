mod core;
mod entity;
mod frame;
mod input;
mod macros;
mod particle;
mod spatial_grid;
mod vector;

use core::Core;
use particle::ParticleSystem;
use std::time::Instant;
use vector::Vector2D;

const TITLE: &str = "slime";
const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const REFRESH: usize = 60;

fn main() {
    let particle_system = ParticleSystem::new(
        Vector2D {
            x: WIDTH,
            y: HEIGHT,
        },
        Vector2D::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0),
        10000,
        0.5,
    );
    let mut core = Core::new(TITLE, WIDTH, HEIGHT, REFRESH);
    core.add_entity(particle_system);

    let mut last = Instant::now();
    while core.is_open() {
        let now = Instant::now();
        let dt = now.duration_since(last).as_secs_f32();
        last = now;

        core.analyze_event();
        core.update(dt);
        core.draw();
        core.next_frame();
    }
}
