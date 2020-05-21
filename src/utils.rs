use quicksilver::{
    geom::Vector,
    graphics::{Font, Image, FontStyle, Color},
    prelude::*,
};
use std::collections::HashMap;


use crate::entity;

fn fix_vector_pixel(vector: Vector) -> Vector {
    return Vector::new(vector.x as u32, vector.y as u32);
}

pub fn create_score_label(score: i32, font: &Font) -> Image {
    let style = FontStyle::new(32.0, Color::WHITE);
    let msg: String = score.to_string();
    return font.render(&msg, &style).unwrap();
}

pub fn get_level(score: i32) -> &'static entity::Level {
    match Some(score) {
        Some(x) if x < 100 => return &entity::LEVELS[0],
        Some(x) if x < 200 => return &entity::LEVELS[1],
        Some(x) if x < 350 => return &entity::LEVELS[2],
        Some(x) if x < 450 => return &entity::LEVELS[3],
        Some(x) if x < 550 => return &entity::LEVELS[4],
        Some(x) if x >= 550 => return &entity::LEVELS[5],
        Some(_) => panic!(),
        None => panic!(),
    }
}

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

pub fn get_asteroids_asset() -> HashMap<&'static str, Image> {
    let mut asteroids = HashMap::new();

    let colors = map!{
        "WHITE" => Color::WHITE,
        "BLACK" => Color::BLACK,
        "RED" => Color::RED,
        "ORANGE" => Color::ORANGE,
        "YELLOW" => Color::YELLOW,
        "GREEN" => Color::GREEN,
        "CYAN" => Color::CYAN,
        "BLUE" => Color::BLUE,
        "PURPLE" => Color::PURPLE,
        "INDIGO" => Color::INDIGO
    };

    let font = Font::load("clacon.ttf").wait().unwrap();

    for (label, color) in colors {
        let o_style = FontStyle::new(48.0, color);
        let asteroid_asset = font.render("O", &o_style).unwrap();
        asteroids.entry(label).or_insert(asteroid_asset);
    }

    return asteroids
}
