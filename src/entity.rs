use quicksilver::graphics::Color;
use quicksilver::geom::Vector;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use crate::grid::{Grid, Asteroid};

// Rapresent a level of A game.
pub struct Level {
    pub color: &'static str,
    pub speed_range: (f32, f32),
    pub fill_ratio_range: (f32, f32),
}

impl Level {
    pub fn generate_wave(&self, grid: Grid) -> Vec<Asteroid> {
        let mut rng = thread_rng();
        let (min, max) = self.fill_ratio_range;
        let asteroids_number = rng.gen_range(min, max) * grid.grid_size;

        let (v_min, v_max) = self.speed_range;
        let asteroids_speed = rng.gen_range(v_min, v_max);

        let mut grid_positions: Vec<i32> = (0..grid.grid_size as i32).collect();
        let slice: &mut [i32] = &mut grid_positions;
        let mut positions: Vec<i32> = Vec::new();

        for _i in 0..asteroids_number as usize {
            let pos = slice.choose(&mut rng);
            match pos {
                Some(p) => positions.push(*p),
                None => continue,
            }
        }

        positions.sort();
        // Collapse when there are more than 2 asteroids
        let positions: Vec<i32> = positions
            .iter().cloned()
            .enumerate()
            .filter(|(i, x)| {
                let left: i32;
                let right: i32;
                if i > &0 {
                    left = x - positions[i-1];
                } else {
                    left = 0;
                }

                if i < &(positions.len() - 1) {
                    right = x - positions[i+1];
                } else {
                    right = 0;
                }

                return (left.abs() > 1) && (right.abs() > 1);
            })
            .map(|(_i, x)| x)
            .collect();

        let mut asteroids = Vec::new();
        for p in positions {
            asteroids.push(Asteroid {
                pos: Vector::new(p as f32, 0.0),
                velocity: Vector::new(0, asteroids_speed),
                color: self.color,
            });
        }
        // let asteroids = pos.iter()
        //     .filter(|pos| )
        return asteroids;
    }
}

pub const LEVELS: [Level; 6] = [
    Level {
        color: "WHITE",
        speed_range: (1.5, 1.5005),
        fill_ratio_range: (0.2, 0.25),
    },
    Level {
        color: "RED",
        speed_range: (1.8, 2.0),
        fill_ratio_range: (0.25, 0.27),
    },
    Level {
        color: "INDINGO",
        speed_range: (2.0, 2.2),
        fill_ratio_range: (0.35, 0.4),
    },
    Level {
        color: "ORANGE",
        speed_range: (2.0, 2.1),
        fill_ratio_range: (0.44, 0.48),
    },
    Level {
        color: "GREEN",
        speed_range: (2.5, 3.5),
        fill_ratio_range: (0.3, 0.35),
    },
    Level {
        color: "BLUE",
        speed_range: (3.0, 3.8),
        fill_ratio_range: (0.50, 0.71),
    },
];
