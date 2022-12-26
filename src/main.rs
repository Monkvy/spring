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
    
    // SFML
    let mut _dt = 0.;
    let mut window = RenderWindow::new((800, 600), "Spring", Style::CLOSE, &Default::default());
    let mut clock = Clock::start();
    window.set_framerate_limit(MAX_FPS);
     
    // Mouse state
    let mut mouse_state = MouseState::new();
    
    // Variables
    let brush = 10.;
    let mut selected: Option<Particle> = None;
    let mut running = false;
    let mut tools = vec!["create-dyn", "create-stc", "delete"];

    // Particles & springs
    let mut particles: Vec<Particle> = Vec::new();
    let mut springs: Vec<Spring> = Vec::new();

    // Main loop
    while window.is_open() {
        // Events
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::KeyPressed { code, alt: _, ctrl: _, shift: _, system: _ }  => {
                    if code == Key::Space {
                        running = !running
                    }
                    else if code == Key::Tab {
                        let first = tools.remove(0);
                        tools.push(first);
                    }
                },
                Event::MouseButtonPressed { button, x, y } => {
                    if button == mouse::Button::Right && tools[0].starts_with("create") {
                        let pos = Vector(x, y).cast::<f32>();
                        if let Some(p) = particles.iter().find(|p| (pos - p.pos).mag::<f32>() < p.mass + brush) {
                            match selected {
                                Some(p_s) => {
                                    selected = None;
                                    springs.push(Spring::new(*p, p_s, None))
                                },
                                None => {
                                    selected = Some(*p);
                                }
                            }          
                        }
                        else {
                            selected = None;
                            Particle::create(&mut particles, pos.cast(), 16., tools[0] == "create-dyn");
                        }
                    }
                    mouse_state.update(event)
                },
                Event::MouseButtonReleased { button: _, x: _, y: _ } | Event::MouseMoved { x: _, y: _ } => {
                    mouse_state.update(event)
                },
                _ => () 
            }
        }
 
        // Mouse Events
        match mouse_state.button {
            // If no particle selected, move particle
            Some(mouse::Button::Left) if selected.is_none() => {
                let pos = mouse_state.pos.cast::<f32>();
                if let Some(p) = particles.iter_mut().find(|&&mut p| (pos - p.pos).mag::<f32>() < p.mass + brush) {
                    p.pos = pos;
                    p.vel = Vector(0., 0.);
                }
            },
            // Delete hovered particle / spring
            Some(mouse::Button::Right) if tools[0] == "delete" => {
                let pos = mouse_state.pos.cast::<f32>();
                if let Some(i) = particles.iter().position(|p| (pos - p.pos).mag::<f32>() < p.mass + brush) {
                    particles.remove(i);
                }
                // TODO: Delete hovered spring
            }
            _ => ()
        }
        
        // Remove springs without two valid particles
        for spring in springs.clone() {
            if !particles.iter().any(|p| p.get_i() == spring.get_ai()) || !particles.iter().any(|p| p.get_i() == spring.get_bi()) {
                springs.remove(springs.iter().position(|s| *s == spring).unwrap());
            }
        }

        // Update
        _dt = clock.restart().as_seconds();
        if running {
            for particle in &mut particles {
                particle.apply_force((0., 100000.));
                particle.update(_dt);
            }
            for spring in springs.clone() {
                spring.update(&mut particles);
            }
        }
        
        // Render
        // FIXME: Use config.
        window.clear(Color::BLACK);
        for spring in springs.clone() {
            spring.draw(particles.clone(), &mut window);
        }
        for particle in &mut particles {
            particle.draw(&mut window);
        }
        window.display();
    }
}


























/*mod vector;
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
                Event::MouseButtonPressed { button, x: _, y: _ } => {
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
}*/
