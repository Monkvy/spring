use sfml::graphics::{RenderWindow, CircleShape, Shape, Transformable, RenderTarget};
use crate::{Vector, util, config};


pub struct Particle {
    pos: Vector<f32>,
    vel: Vector<f32>,
    acc: Vector<f32>,
    pub mass: f32,
    pub dynamic: bool,
}

impl Particle {
    pub fn new<V: Into<Vector<f32>>>(pos: V, mass: f32, dynamic: bool) -> Self {
        Self { pos: pos.try_into().unwrap(),
            vel: Vector(0., 0.),
            acc: Vector(0., 0.),
            mass, dynamic
        }
    }

    pub fn apply_force<V: Into<Vector<f32>>>(&mut self, f: V) {
        if !self.dynamic { return }
        let mut force: Vector<f32> = f.try_into().unwrap();
        force /= self.mass;
        self.acc += force;
    }

    pub fn update(&mut self, dt: f32) {
        if !self.dynamic { return }
        self.vel += self.acc * dt;
        self.pos += self.vel * dt;
        self.acc = Vector(0., 0.);
    } 

    pub fn draw(&mut self, window: &mut RenderWindow) {
        let mut c = CircleShape::new(self.mass, 20);
        c.set_position((self.pos.0, self.pos.1));
        
        unsafe {
            if self.dynamic {
                c.set_fill_color(util::tuple_to_color(config::CONFIG.color.p_dynamic));
            } else {
                c.set_fill_color(util::tuple_to_color(config::CONFIG.color.p_static));
            }
        }
        window.draw(&c);
    }
}
