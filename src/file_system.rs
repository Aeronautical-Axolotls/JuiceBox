// https://docs.rs/serde_json/latest/serde_json/

use std::fs;
use std::io::BufWriter;
use bevy::math::Vec2;
use serde_json;
use crate::simulation::sim_state_manager::SimGrid;
use crate::simulation::sim_state_manager::SimGridCellType;
use crate::simulation::sim_state_manager::SimConstraints;
use crate::simulation::sim_state_manager::SimParticles;

pub fn json_load(file_name: &str, sim_grid_ref: &mut SimGrid, sim_constraints_ref: &mut SimConstraints, sim_particles_ref: &mut SimParticles) {

    // Connecting to the file and creating a serde_json object
    println!("Loading File {}...", file_name);

    let file_path = format!("./saves/{}", file_name);
    let file = fs::File::open(file_path)
        .expect("file should be opened");
    let json: serde_json::Value = serde_json::from_reader(file)
        .expect("file should be correct format");

    // Setting the data to the saved data in the json.
    sim_grid_ref.dimensions = (json["dimensions"].as_array().unwrap().to_vec()[0].as_u64().unwrap() as u16,
                               json["dimensions"].as_array().unwrap().to_vec()[1].as_u64().unwrap() as u16);
    sim_grid_ref.cell_size = json["cell_size"].as_u64().unwrap() as u16;
    sim_grid_ref.cell_type = json["cell_type"].as_array().unwrap().to_vec().iter()
                             .map(|a| celltype_from_str(a.as_str().unwrap())).collect();    // See sim_particles_ref.particle_position to understand what is happening here
    sim_grid_ref.velocity = vec![[Vec2::new(0.0, 0.0); 4]; 625];

    sim_constraints_ref.grid_particle_ratio = json["iterations_per_frame"].as_f64().unwrap() as f32;
    sim_constraints_ref.iterations_per_frame = json["iterations_per_frame"].as_u64().unwrap() as u8;
    sim_constraints_ref.gravity = vec2_from_vec(json["gravity"].as_array().unwrap().to_vec());

    sim_particles_ref.particle_position = json["particle_position"].as_array() // Getting the json particle_position as an optional of a vector of Serde::Values.
                                                                                                   //   serde_json::Values are generalized values that can be Numbers, Strings, etc.
                                                                                                   //   In this case it's a vector of vectors of 2 floats. But Serde doesn't know that. 
        .unwrap()                                                                       // Unwrapping the optional
        .to_vec()                                                                        //
        .iter().map(|a| vec2_from_vec(a.as_array().unwrap().to_vec())).collect();           // Getting an iterator from the vector so I can then map over it.
                                                                                                    //   For each value in the array (which is a vector of 2 floats),
                                                                                                    //   I am converting them into a Bevy::Vec2 of f32s.
    sim_particles_ref.particle_velocity = json["particle_velocity"].as_array().unwrap().to_vec().iter()
        .map(|a| vec2_from_vec(a.as_array().unwrap().to_vec())).collect();
    sim_particles_ref.particle_count = json["particle_count"].as_u64().unwrap() as usize;
}

pub fn json_save(file_name: &str, sim_grid_ref: &mut SimGrid, sim_constraints_ref: &mut SimConstraints, sim_particles_ref: &mut SimParticles) {

    let file_path = format!("./saves/{}", file_name);
    let file = fs::File::create(file_path)
        .expect("file should have successfully been made");
    let writer = BufWriter::new(file);

    let json = serde_json::json!({
        "dimensions": sim_grid_ref.dimensions,
        "cell_size": sim_grid_ref.cell_size,
        "cell_type": sim_grid_ref.cell_type.iter().map(|a| str_from_celltype(a)).collect::<Vec<_>>(),
        "cell_velocity": "TODO",
        "grid_particle_ratio": sim_constraints_ref.grid_particle_ratio,
        "iterations_per_frame": sim_constraints_ref.iterations_per_frame,
        "gravity": sim_constraints_ref.gravity,
        "particle_count": sim_particles_ref.particle_count,
        "particle_position": sim_particles_ref.particle_position.iter().map(|a| vec_from_vec2(a)).collect::<Vec<_>>(),
        "particle_velocity": sim_particles_ref.particle_velocity.iter().map(|a| vec_from_vec2(a)).collect::<Vec<_>>()
    });

    serde_json::to_writer_pretty(writer, &json)
        .expect("json file should be created");
}

/// Special helper functions for particle_position, particle_velocity, and gravity
fn vec2_from_vec(vec: Vec<serde_json::Value>) -> Vec2 {
    let x_val = vec[0].as_f64().unwrap() as f32;
    let y_val = vec[1].as_f64().unwrap() as f32;
    return Vec2 { x: x_val, y: y_val };
}
fn vec_from_vec2(vec2: &Vec2) -> Vec<f32> {
    return Vec::from([vec2.x, vec2.y]);
}

fn celltype_from_str(str: &str) -> SimGridCellType {
    match str {
        "Air" => return SimGridCellType::Air,
        "Fluid" => return SimGridCellType::Fluid,
        "Solid" => return SimGridCellType::Solid,
        _ => return SimGridCellType::Air,
    }
}
fn str_from_celltype(cell_type: &SimGridCellType) -> &'static str {
    match cell_type {
        SimGridCellType::Air => return "Air",
        SimGridCellType::Fluid => return "Fluid",
        SimGridCellType::Solid => return "Solid",
        _ => return "Air",
    }
}