use sfml::graphics::{RenderWindow, CircleShape, Shape, Transformable, RenderTarget, Color};
use crate::Vector;


#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub pos: Vector<f32>,
    pub mass: f32,
    pub vel: Vector<f32>,
    acc: Vector<f32>,
    dynamic: bool,
    i: usize,
}

impl Particle {
    /// Creates a particle with the given attributes and
    /// appends it to the given list.
    /// (self.i = index of self inside 'all': Vec).
    pub fn create<V: Into<Vector<f32>>>(particles: &mut Vec<Particle>, pos: V, mass: f32, dynamic: bool) {
        particles.push(Self {
            i: particles.len(),
            pos: pos.try_into().unwrap(),
            vel: Vector(0., 0.),
            acc: Vector(0., 0.),
            mass, dynamic
        });
    }

    /// Returns the index of self inside the list
    /// this particle was created in.
    pub fn get_i(&self) -> usize {
        self.i
    }

    /// Applies force to this particle.
    /// The position gets updated when calling update().
    pub fn apply_force<V: Into<Vector<f32>>>(&mut self, f: V) {
        if !self.dynamic { return }
        let mut force: Vector<f32> = f.try_into().unwrap();
        force /= self.mass;
        self.acc += force;
    }

    /// Updates the position of this particle based
    /// on the current accelaration & the given delta time.
    pub fn update(&mut self, dt: f32) {
        if !self.dynamic { return }
        self.vel *= 0.99;
        self.vel += self.acc * dt;
        self.pos += self.vel * dt;
        self.acc = Vector(0., 0.);
    } 

    /// Draws a circle to self.pos onto the given window.
    pub fn draw(&mut self, window: &mut RenderWindow) {
        let mut c = CircleShape::new(self.mass, 20);
        c.set_position((self.pos.0 - 20., self.pos.1 - 20.));
        
        // FIXME: Use config.
        if self.dynamic {
            c.set_fill_color(Color::BLUE);
        } else {
            c.set_fill_color(Color::RED);
        }
        window.draw(&c);
    }
}
