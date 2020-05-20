use std::time::{Duration, Instant};
use quicksilver::{
    Result,
    geom::{Vector},
    graphics::{Color, Font, FontStyle, Image},
    lifecycle::{Settings, State, Window, run},
    prelude::*,
};

mod grid;
mod utils;
mod entity;

#[derive(Clone, Debug, PartialEq)]
struct Tile {
    pos: Vector,
    glyph: char,
    color: Color,
}

fn generate_map(size: Vector) -> Vec<Tile> {
    let width = size.x as usize;
    let height = size.y as usize;
    let mut map = Vec::with_capacity(width * height);
    for x in 0..width {
        for y in 0..height {
            let mut tile = Tile {
                pos: Vector::new(x as f32, y as f32),
                glyph: '.',
                color: Color::BLACK,
            };

            if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                tile.glyph = '#';
            };
            map.push(tile);
        }
    }
    map
}

#[derive(Clone, Debug, PartialEq)]
struct Entity {
    pos: Vector,
    color: Color,
}


struct Game {
    player: Entity,
    player_asset: Image,
    font: Font,
    asteroids: Vec<grid::Asteroid>,
    asteroid_asset: Image,
    last_instant: Instant,
    time_delta: Duration,
    screen_size: Vector,
    grid: grid::Grid,
    score: i32,
}

impl State for Game {
    fn new() -> Result<Self> {
        let style = FontStyle::new(48.0, Color::WHITE);
        let o_style = FontStyle::new(48.0, Color::WHITE);
        let font = Font::load("clacon.ttf").wait().unwrap();
        let player_asset = font.render("A", &style).unwrap();
        let asteroid_asset = font.render("O", &o_style).unwrap();

        let asteroids = vec![];
        let last_instant = Instant::now();
        let time_delta = Duration::from_secs(0);
        let screen_size = Vector::new(1000.0, 1000.0);
        let grid = grid::Grid::new(screen_size.x, screen_size.y, 30.0);
        let score = 500;

        let player = Entity {
            pos: Vector::new(30.0 / 2.0, 30.0 - 1.0),
            color: Color::WHITE,
        };

        Ok(Self {
            player,
            player_asset,
            font,
            asteroids,
            asteroid_asset,
            last_instant,
            time_delta,
            screen_size,
            grid,
            score,
        })
    }

    /// Process keyboard and mouse, update the game state
    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.update_time_step();
        self.update_asteroids();
        self.clear_asteroids();
        self.generate_asteroids();

        use ButtonState::*;


        let mut position = self.player.pos;
        if window.keyboard()[Key::Left] == Pressed {
            position.x -= 1.0
        }
        if window.keyboard()[Key::Right] == Pressed {
            position.x += 1.0
        }
        if window.keyboard()[Key::Up] == Pressed {
            position.y -= 1.0
        }
        if window.keyboard()[Key::Down] == Pressed {
            position.y += 1.0
        }

        if self.grid.is_in(position) {
            self.player.pos = position
        }

        for a in self.asteroids.iter() {
            if self.grid.collide(a.pos, self.player.pos) {
                panic!("La muerte te a matado!!")
            }
        }

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;

        let player = &mut self.player;

        let pos_px = match self.grid.translate_to_screen(player.pos) {
            Ok(p) => p,
            Err(_) => Vector::new(0.0, 0.0),
        };

        // Draw player
        window.draw(
            &self.player_asset.area().with_center(pos_px),
            &self.player_asset,
        );

        // Draw asteroids
        let asteroids = &mut self.asteroids;
        for asteroid in asteroids {
            let asteroid_pos = match self.grid.translate_to_screen(asteroid.pos) {
                Ok(p) => p,
                Err(_) => Vector::new(0.0, 0.0),
            };
            window.draw(
                &self.asteroid_asset.area().with_center(asteroid_pos),
                &self.asteroid_asset,
            );
        }

        // Draw scores
        let score_label = utils::create_score_label(self.score, &self.font);
        let score_area = score_label.area().with_center(
                self.screen_size - score_label.area().size().times(Vector::new(0.5, 0.5))
        );

        window.draw(
            &score_area,
            &score_label
        );

        Ok(())
    }
}

impl Game {
    // Calulate last time step.
    fn update_time_step(&mut self) {
        let now = Instant::now();
        let time_step = now.duration_since(self.last_instant.clone());
        self.last_instant = now;
        self.time_delta = time_step;
    }

    // Move asteroid
    fn update_asteroids(&mut self) {
        let asteroids = &mut self.asteroids;

        for asteroid in asteroids {
            let time_unit = self.time_delta.as_micros() as f32 * f32::powf(10.0, -6.0);
            asteroid.update(time_unit);
        }
    }

    // Remove the asteroids when are out of sight.
    fn clear_asteroids(&mut self) {
        let asteroids = self.asteroids.to_vec();
        self.asteroids = asteroids
            .into_iter()
            .filter(|a| self.grid.is_in(a.pos))
            .collect();

        let score = self.asteroids.len();

        self.score += score as i32;
    }

    // Remove the asteroids when are out of sight.
    fn generate_asteroids(&mut self) {
        // If all asteroinds in the current line are gone
        let remain = self.asteroids
            .iter()
            .filter(|a| a.pos.y < 1.0)
            .count();

        if remain > 0 {
            return;
        }

        let new_wave = utils::get_level(self.score).generate_wave(self.grid);
        self.asteroids.extend(new_wave.iter().cloned());
        // self.asteroids.append(new_wave);

    }
}



fn main() {
    env_logger::init();

    let settings = Settings {
        ..Default::default()
    };
    run::<Game>("A Game", Vector::new(1000, 1000), settings);
}
