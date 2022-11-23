use sfml::{graphics::{RenderWindow, Color, Vertex, RenderTarget, PrimitiveType, RenderStates}, system::Vector2f};
use crate::Particle;


/// Stores the indices of both particles,
/// the initial distance and spring force (k).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spring {
    ai: usize,
    bi: usize,
    len: f32,
    k: f32
}

impl Spring {
    /// Creates a new spring connection the given particles with
    /// the given spring force (k, defaults to 200.0).
    pub fn new(a: Particle, b: Particle, k: Option<f32>) -> Self {
        Self {
            ai: a.get_i(),
            bi: b.get_i(),
            len: (b.pos - a.pos).mag(),
            k: k.unwrap_or(1000.)
        }
    }

    /// Return the index of the first particle.
    pub fn get_ai(&self) -> usize {
        self.ai
    }

    /// Return the index of the second particle.
    pub fn get_bi(&self) -> usize {
        self.bi
    }

    /// Applies force to both particles based on 
    /// the spring force (self.k).
    pub fn update(&self, particles: &mut [Particle]) {
        let mut f = particles.iter().find(|p| p.get_i() == self.bi).unwrap().pos - particles.iter().find(|p| p.get_i() == self.ai).unwrap().pos;
        let x = f.mag::<f32>() - self.len;
        f = f.norm::<f32>();
        f *= self.k * x;


        particles.iter_mut().find(|p| p.get_i() == self.ai).unwrap().apply_force(f);
        f *= -1.;
        particles.iter_mut().find(|p| p.get_i() == self.bi).unwrap().apply_force(f);
    }

    /// Draws a line from particle a to particle b.
    pub fn draw(&self, particles: Vec<Particle>, window: &mut RenderWindow) {
        let a = particles.iter().find(|p| p.get_i() == self.ai).unwrap();
        let b = particles.iter().find(|p| p.get_i() == self.bi).unwrap();
        // FIXME: Use config.
        let line = vec![
            Vertex::new(Vector2f::new(a.pos.0, a.pos.1), Color::CYAN, Vector2f::new(0., 0.)),
            Vertex::new(Vector2f::new(b.pos.0, b.pos.1), Color::CYAN, Vector2f::new(0., 0.))
        ];
        window.draw_primitives(&line, PrimitiveType::LINES, &RenderStates::default());
    }
}
