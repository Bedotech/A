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


#[derive(Clone,Copy)]
// Represent a section screen area, is useful for draw single object inside the grid instead of
// using absolute coordinates.
pub struct Grid {
    // The surface of the screen.
    screen_size: quicksilver::geom::Vector,
    // How many slot the grid have, is always a square.
    pub grid_size: f32,
    // Computed tile size.
    tile_size: quicksilver::geom::Vector,
}

impl Grid {
    pub fn new(
        width: f32,
        height: f32,
        grid: f32,
    ) -> Grid {
        let screen_size = Vector::new(width, height);
        let tile_size = screen_size.times(Vector::new(
            1.0 / grid,
            1.0 / grid
        ));

        return Grid {
            screen_size,
            grid_size: grid,
            tile_size,
        }
    }

    // Check if a point is inside the grid.
    pub fn is_in(self, point: Vector) -> bool {
        return (point.x >= 0.0 && point.x < self.grid_size) && (point.y >= 0.0 && point.y < self.grid_size)
    }

    // Take a point, check if is inside the grid, translate to screen px, for center the grid.
    pub fn translate_to_screen(self, point: Vector) -> Result<Vector, &'static str> {
        if !self.is_in(point) {
            return Err("the point is outside of drawable surface")
        }
        let offset_px = Vector::new(0.5, 0.5);
        return Ok(self.tile_size.times(offset_px) + point.times(self.tile_size));
    }

    // Collide take two point and check if fit in the same grid cell,
    // in reality check the euclidian distance between the two point, if this
    // distance is below the half tile_size the collide append.
    pub fn collide(self, a: Vector, b: Vector) -> bool {
        let first = match self.translate_to_screen(a) {
            Ok(p) => p,
            Err(_) => a,
        };

        let second = match self.translate_to_screen(b) {
            Ok(p) => p,
            Err(_) => b,
        };

        return first.distance(second) < (self.tile_size.len() / 2.0)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_grid_new() {
        let grid = Grid::new(100.0, 100.0, 100.0);
        assert_eq!(grid.grid_size, 100.0);

        assert_eq!(grid.tile_size.x, 1.0);
        assert_eq!(grid.tile_size.y, 1.0);
    }

    #[test]
    fn test_is_in() {
        let grid = Grid::new(100.0, 100.0, 10.0);

        assert!(grid.is_in(Vector::new(0.0, 0.0)));
        assert!(grid.is_in(Vector::new(9.0, 9.0)));
        assert!(grid.is_in(Vector::new(5.0, 5.0)));

        assert!(!grid.is_in(Vector::new(0.0, -1.0)));
        assert!(!grid.is_in(Vector::new(10.0, 1.0)));
        assert!(!grid.is_in(Vector::new(10.0, 5.0)));

    }

    #[test]
    fn test_translate() {
        let grid = Grid::new(100.0, 100.0, 10.0);

        let point = Vector::new(2.0,2.0);
        match grid.translate_to_screen(point) {
            Ok(p) => assert_eq!(p, Vector::new(25.0, 25.0)),
            Err(e) => panic!("Fail translation: {}", e),
        }

        let point = Vector::new(12.0,8.0);
        match grid.translate_to_screen(point) {
            Ok(_) => panic!("Expected error"),
            Err(_) => (),
        }
    }

    #[test]
    fn test_collide() {
        let grid = Grid::new(100.0, 100.0, 10.0);
        assert!(
            grid.collide(Vector::new(0.2, 0.2), Vector::new(0.2, 0.2)),
            "the same point always collide."
        );

        assert!(
            grid.collide(Vector::new(0.2, 0.2), Vector::new(0.3, 0.3)),
            "points in same cell is a collision."
        );

        assert!(
            !grid.collide(Vector::new(5.0, 5.0), Vector::new(10.0, 10.0)),
            "points near but in different cell don't collide."
        );



    }
}
