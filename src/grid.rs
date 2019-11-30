use quicksilver::graphics::Color;
use quicksilver::geom::Vector;

#[derive(Clone, Debug, PartialEq)]
pub struct Asteroid {
    pub pos: Vector,
    pub velocity: Vector,
    pub color: Color,
}

impl Asteroid {
    pub fn update(&mut self, time_delta: f32) {
        self.pos += self.velocity * time_delta;
    }
}
