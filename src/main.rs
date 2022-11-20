mod vector;
mod mouse_state;
mod particle;
mod spring;

use sfml::{
    graphics::{RenderWindow, RenderTarget, Color},
    window::{Style, Event, Key, mouse},
    system::Clock
};
use vector::Vector;
use mouse_state::MouseState;
use particle::Particle;
use spring::Spring;

const MAX_FPS: u32 = 144;


fn main() {
    // Window
    let mut _dt = 0.;
    let mut running = false;
    let mut window = RenderWindow::new((800, 600), "Spring", Style::CLOSE, &Default::default());
    window.set_framerate_limit(MAX_FPS);
    
    let brush = 10.;

    let mut mouse_state = MouseState::new();
    let mut all: Vec<Particle> = Vec::new();
    Particle::create(&mut all, (200., 100.), 16., false);
    Particle::create(&mut all, (300., 200.), 16., true);
    
    let s = Spring::new(all[0], all[1], None);

    // Main loop
    let mut clock = Clock::start();
    while window.is_open() {

        // Events
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::KeyPressed { code, alt: _, ctrl: _, shift: _, system: _ } if code == Key::Space => {
                    running = !running
                },
                Event::MouseButtonPressed { button, x, y } => {
                    // let pos = Vector(x, y).cast::<f32>();
                    
                    // TODO: Create, delete, connect particles.
                    if button == mouse::Button::Left {
                        // for particle in all {
                        //     if (pos - particle.pos).mag::<f32>() < particle.mass + brush {

                        //     }
                        // }
                    }

                    mouse_state.update(event)
                },
                _ => () 
            }
        }
        
        // Mouse Events
        if mouse_state.button == Some(mouse::Button::Left) {
            let pos = mouse_state.pos.cast::<f32>();
            for particle in &mut all {
                if (pos - particle.pos).mag::<f32>() < particle.mass + brush {
                    particle.pos = pos;
                } 
            }
        }

        // Update
        _dt = clock.restart().as_seconds();
        if running {
            // TODO: Step
            s.update(&mut all);
            for particle in &mut all {
                particle.apply_force((0., 15000.));
                particle.update(_dt);
            }
        }
 
        // Render
        // FIXME: Use config.
        window.clear(Color::BLACK);
        s.draw(all.clone(), &mut window);
        for particle in &mut all {
            particle.draw(&mut window);
        }
        window.display();
    }
}
