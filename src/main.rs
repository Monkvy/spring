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
    
    let mut a = Particle::new((200., 100.), 16., true);
    let mut b = Particle::new((600., 400.), 16., true);
    let mut s = Spring::new(Box::new(a), Box::new(b), 1.);

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
            a.apply_force((0., 2500.));
            b.apply_force((0., 2500.));
            s.apply();
            a.update(_dt);
            b.update(_dt);
        }
 
        // Render
        // FIXME: Use config.
        window.clear(Color::BLACK);
        s.draw(&mut window);
        a.draw(&mut window);
        b.draw(&mut window);
        window.display();
    }
}
