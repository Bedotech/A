use log::{info};
use std::time::{Duration, Instant};
use rand::{Rng, ThreadRng};
use quicksilver::{
    Result,
    geom::{Line, Transform, Vector},
    graphics::{Background::Col, Color, Font, FontStyle, Image},
    lifecycle::{Asset, Settings, State, Window, run},
    prelude::*,
};

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

#[derive(Clone, Debug, PartialEq)]
struct Asteroid {
    pos: Vector,
    velocity: Vector,
    color: Color,
}


fn generate_asteroids() -> Vec<Asteroid> {
    vec![
        Asteroid {
            pos: Vector::new(0,0),
            velocity: Vector::new(0.0, 0.1),
            color: Color::WHITE,
        },

        Asteroid {
            pos: Vector::new(3,0),
            velocity: Vector::new(0.3, 0.2),
            color: Color::WHITE,
        },

        Asteroid {
            pos: Vector::new(5,0),
            velocity: Vector::new(0.0, 1.6),
            color: Color::WHITE,
        },
    ]
}

struct Game {
    player: Entity,
    player_asset: Asset<Image>,
    asteroids: Vec<Asteroid>,
    asteroid_asset: Asset<Image>,
    last_instant: Instant,
    time_delta: Duration,
    screen_size: Vector,
    grid: f32,
    tile_size_px: Vector,
    rng: ThreadRng,
}

impl State for Game {
    fn new() -> Result<Self> {
        let player_asset = Asset::new(Font::load("font.ttf")
            .and_then(|font| {
                let style = FontStyle::new(72.0, Color::WHITE);
                result(font.render("A", &style))
            })
        );

        let asteroid_asset = Asset::new(Font::load("font.ttf")
            .and_then(|font| {
                let style = FontStyle::new(72.0, Color::WHITE);
                result(font.render("O", &style))
            })
        );

        let player = Entity {
            pos: Vector::new(0,0),
            color: Color::WHITE,
        };

        let asteroids = generate_asteroids();
        let last_instant = Instant::now();
        let time_delta = Duration::from_secs(0);
        let screen_size = Vector::new(1000.0, 1000.0);
        let grid = 12.0;
        let tile_size_px = screen_size.times(Vector::new(
            1.0 / grid,
            1.0 / grid
        ));
        let mut rng = rand::thread_rng();

        Ok(Self {
            player,
            player_asset,
            asteroids,
            asteroid_asset,
            last_instant,
            time_delta,
            screen_size,
            grid,
            tile_size_px,
            rng,
        })
    }

    /// Process keyboard and mouse, update the game state
    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.update_time_step();
        self.update_asteroids();
        self.clear_asteroids();
        self.generate_asteroids();

        use ButtonState::*;

        let player = &mut self.player;

        if window.keyboard()[Key::Left] == Pressed {
            player.pos.x -= 1.0
        }
        if window.keyboard()[Key::Right] == Pressed {
            player.pos.x += 1.0
        }
        if window.keyboard()[Key::Up] == Pressed {
            player.pos.y -= 1.0
        }
        if window.keyboard()[Key::Down] == Pressed {
            player.pos.y += 1.0
        }

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;

        for x in 1..self.grid as i32 {
            let y_row = x as f32 * self.tile_size_px.y;
            window.draw_ex(
                &Line::new((0, y_row),(self.screen_size.x, y_row)).with_thickness(2.0),
                Col(Color::RED),
                Transform::IDENTITY,
                5
            );

            let x_row = x as f32 * self.tile_size_px.y;
            window.draw_ex(
                &Line::new((x_row, 0),(x_row, self.screen_size.y)).with_thickness(2.0),
                Col(Color::RED),
                Transform::IDENTITY,
                5
            );
        }

        let player = &mut self.player;
        let offset_px = Vector::new(0.5, 0.5);
        let pos_px = self.tile_size_px.times(offset_px) + player.pos.times(self.tile_size_px);

        // Draw player
        self.player_asset.execute(|image| {
            window.draw(&image.area().with_center(pos_px), Img(&image));
            Ok(())
        });
        // Draw asteroids
        let asteroids = &mut self.asteroids;
        for asteroid in asteroids {
            let asteroid_pos = self.tile_size_px.times(offset_px) + asteroid.pos.times(self.tile_size_px);
            self.asteroid_asset.execute(|image| {
                window.draw(&image.area().with_center(asteroid_pos), Img(&image));
                Ok(())
            });
        }

        Ok(())
    }
}

impl Game {
    // Calulate last time step.
    fn update_time_step(&mut self) -> Result<()> {
        let now = Instant::now();
        let time_step = now.duration_since(self.last_instant.clone());
        self.last_instant = now;
        self.time_delta = time_step;

        Ok(())
    }

    // Move asteroid
    fn update_asteroids(&mut self) -> Result<()> {
        let asteroids = &mut self.asteroids;

        for asteroid in asteroids {
            let time_unit = self.time_delta.as_micros() as f32 * f32::powf(10.0, -6.0);
            asteroid.pos += asteroid.velocity * time_unit;
            if asteroid.pos.x > self.grid || asteroid.pos.y > self.grid {
                info!("DELETE!!!")
            }
        }

        Ok(())
    }

    // Remove the asteroids when are out of sight.
    fn clear_asteroids(&mut self) -> Result<()> {
        let asteroids = self.asteroids.to_vec();
        let grid = self.grid;
        asteroids
            .iter()
            .enumerate()
            .filter(|(_, a)| a.pos.x > grid || a.pos.y > grid)
            .map(|(e, _)| self.asteroids.remove(e))
            .collect::<Vec<_>>();

        Ok(())
    }

    // Remove the asteroids when are out of sight.
    fn generate_asteroids(&mut self) -> Result<()> {
        // If asteroids are 10 wait
        for i in self.asteroids.len()..20 {
            let a = Asteroid {
                pos: Vector::new(self.rng.gen_range(0, 10) ,0),
                velocity: Vector::new(0.0, self.rng.gen_range(0.8, 2.5)),
                color: Color::WHITE,
            };

            self.asteroids.push(a);
        }


        Ok(())
    }
}

fn main() {
    env_logger::init();

    let settings = Settings {
        ..Default::default()
    };
    run::<Game>("A Game", Vector::new(1000, 1000), settings);
}