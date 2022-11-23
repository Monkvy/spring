use sfml::window::{Event, mouse};
use crate::Vector;


/// Captures the pressed mouse button &
/// current mouse position.
#[derive(Debug)]
pub struct MouseState {
    pub button: Option<mouse::Button>,
    pub pos: Vector<i32>
}

impl MouseState {
    /// Creates a new mouse state.
    pub fn new() -> MouseState {
        MouseState { button: None, pos: Vector(0, 0) }
    }

    /// Updates the button & position of the mouse state.
    pub fn update(&mut self, event: Event) {
        match event {
            Event::MouseButtonPressed { button, x, y } => {
                self.button = Some(button);
                self.pos = Vector(x, y);
            },
            Event::MouseButtonReleased { button: _, x, y } => {
                self.button = None;
                self.pos = Vector(x, y);
            }
            Event::MouseMoved { x, y } => self.pos = Vector(x, y),
            _ => ()
        }
    }
}
