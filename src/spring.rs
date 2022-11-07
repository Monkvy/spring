use sfml::{graphics::{RenderWindow, Color, RenderTarget, Vertex, PrimitiveType, RenderStates}, system::Vector2f};
use crate::Particle;



pub struct Spring {
    pub a: Particle,
    pub b: Particle,
    force: f32,
    len: f32
}

impl Spring {
    pub fn new(a: Particle, b: Particle, force: f32) -> Self {
        Self { a, b, force, len: (b.pos - a.pos).mag::<f32>() }
    }

    /// Applies the spring force to the a & b
    /// particles. The particles positions doesnt get updated.
    /// Call Particle::update(f32) to update them.
    pub fn apply(&mut self) {
        let mut f = self.b.pos - self.a.pos;
        let x = f.mag::<f32>() - self.len;

        f = f.norm::<f32>();
        f *= self.force * x;

        self.b.apply_force(f);
        f *= -1.;
        self.b.apply_force(f);
    }

    /// Draws a line between the two particles.
    pub fn draw(&mut self, window: &mut RenderWindow) {
        // FIXME: Use config.
        let line = vec![
            Vertex::new(Vector2f::new(self.a.pos.0 + self.a.mass, self.a.pos.1 + self.a.mass), Color::CYAN, Vector2f::new(0., 0.)),
            Vertex::new(Vector2f::new(self.b.pos.0 + self.a.mass, self.b.pos.1 + self.a.mass), Color::CYAN, Vector2f::new(0., 0.))
        ];
        window.draw_primitives(&line, PrimitiveType::LINES, &RenderStates::default());
    }
}
