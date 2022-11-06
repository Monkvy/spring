mod util;
mod config;
mod vector;
mod particle;
mod spring;

use sfml::{
    graphics::{RenderWindow, RenderTarget, Color},
    window::{Style, Event, Key},
    system::Clock
};
use vector::Vector;
use particle::Particle;
use spring::Spring;

const MAX_FPS: u32 = u32::MAX;


fn main() {
    // Window
    let mut _dt = 0.;
    let mut running = false;
    let mut window = RenderWindow::new((800, 600), "Spring", Style::CLOSE, &Default::default());
    window.set_framerate_limit(MAX_FPS);
    
    config::load("examples/config.toml");

    let mut p = Particle::new((400., 100.), 16., true);

    // Main loop
    let mut clock = Clock::start();
    while window.is_open() {

        // Events
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::KeyPressed { code, alt: _, ctrl: _, shift: _, system: _ } => {
                    match code {
                        Key::Space => running = !running,
                        Key::Tab => {
                            // TODO: Step
                        },
                        _ => ()
                    }
                },
                _ => ()
            }
        }

        // Update
        _dt = clock.restart().as_seconds();
        if running {
            // TODO: Step
            p.apply_force((0., 2500.));
            p.update(_dt);
        }
 
        // Render
        window.clear(util::tuple_to_color(unsafe { config::CONFIG.color.bg}));
        p.draw(&mut window);
        window.display();
    }
}
