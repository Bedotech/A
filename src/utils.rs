use quicksilver::{
    geom::Vector,
    graphics::{Font, Image, FontStyle, Color}
};

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
