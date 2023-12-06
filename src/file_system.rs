use bevy::prelude::*;
use bevy::math::Vec2;

use std::fs;

pub fn json_load(file_name: &str) {
    println!("Loading File {}...", file_name);

    let data = fs::read_to_string(("/save files/{}", file_name));
    println!("{}", data);
}