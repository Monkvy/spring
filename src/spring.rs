use sfml::{graphics::{RenderWindow, Color, Vertex, RenderTarget, PrimitiveType, RenderStates}, system::Vector2f};
use crate::Particle;


/// Stores the indices of both particles,
/// the initial distance and spring force (k).
pub struct Spring {
    a_i: usize,
    b_i: usize,
    len: f32,
    k: f32
}

impl Spring {
    /// Creates a new spring connection the given particles with
    /// the given spring force (k, defaults to 200.0).
    pub fn new(a: Particle, b: Particle, k: Option<f32>) -> Self {
        Self {
            a_i: a.get_i(),
            b_i: b.get_i(),
            len: (b.pos - a.pos).mag(),
            k: k.unwrap_or(1000.)
        }
    }
    
    /// Applies force to both particles based on 
    /// the spring force (self.k).
    pub fn update(&self, all: &mut [Particle]) {
        let mut f = all[self.b_i].pos - all[self.a_i].pos;
        let x = f.mag::<f32>() - self.len;
        f = f.norm::<f32>();
        f *= self.k * x;

        all[self.a_i].apply_force(f);
        f *= -1.;
        all[self.b_i].apply_force(f);
    }

    /// Draws a line from particle a to particle b.
    pub fn draw(&self, all: Vec<Particle>, window: &mut RenderWindow) {
        let a = all[self.a_i];
        let b = all[self.b_i];

        // FIXME: Use config.
        let line = vec![
            Vertex::new(Vector2f::new(a.pos.0, a.pos.1), Color::CYAN, Vector2f::new(0., 0.)),
            Vertex::new(Vector2f::new(b.pos.0, b.pos.1), Color::CYAN, Vector2f::new(0., 0.))
        ];
        window.draw_primitives(&line, PrimitiveType::LINES, &RenderStates::default());
    }
}
