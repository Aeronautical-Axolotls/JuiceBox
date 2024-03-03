use bevy::math::Vec2;
use crate::error::Error;

use super::SimGrid;

pub type Result<T> = core::result::Result<T, Error>;

/**
    Find the weight of influence of a particle
    to a grid point.
*/
pub fn find_influence(
    particle_pos: Vec2,
    grid_point: Vec2,
    grid_scale: u16) -> f32 {

    let diff = grid_point.distance(particle_pos);

    let scaled_diff = (diff as f32) / (grid_scale as f32);

    if scaled_diff.abs() > 1.0 {
        return 0.0;
    }

    if scaled_diff > 0.0 {
        return 1.0 - scaled_diff;
    } else if scaled_diff < 0.0 {
        return 1.0 + scaled_diff;
    } else {
        return 0.0;
    }
}

// /**
//     Uses bilinear interpolation to find the velocity of the
//     particle interpolated from the nearest grid points.
//     Each grid point in grid_points includes both the
//     (u, v) components and (x, y) coordinates in that order.
// */
// pub fn interpolate_velocity(particle_pos: Vec2, grid: &SimGrid) -> Vec2 {

//     // Grid points 0..3 are the four corners of the bilinear interpolation
//     // in order of clockwise rotation around the particle point.
//     // https://en.wikipedia.org/wiki/Bilinear_interpolation

//     let cell_coords = grid.get_cell_coordinates_from_position(&particle_pos);

//     let row = cell_coords.x;
//     let col = cell_coords.y;

//     let bottom_left = Vec2::new(f32::min(row + 1.0, grid.dimensions.1 as f32), f32::max(col - 1.0, 0.0));
//     let bottom_right = Vec2::new(f32::min(row + 1.0, grid.dimensions.1 as f32), f32::min(col + 1.0, grid.dimensions.0 as f32));
//     let top_left = Vec2::new(f32::max(row - 1.0, 0.0), f32::max(col - 1.0, 0.0));
//     let top_right = Vec2::new(f32::max(row - 1.0, 0.0), f32::min(col + 1.0, grid.dimensions.0 as f32));

//     let grid_points = vec![
//         (grid.get_cell_velocity(bottom_left.x as usize, bottom_left.y as usize), grid.get_cell_position_from_coordinates(bottom_left)),
//         (grid.get_cell_velocity(top_left.x as usize, top_left.y as usize), grid.get_cell_position_from_coordinates(top_left)),
//         (grid.get_cell_velocity(top_right.x as usize, top_right.y as usize), grid.get_cell_position_from_coordinates(top_right)),
//         (grid.get_cell_velocity(bottom_right.x as usize, bottom_right.y as usize), grid.get_cell_position_from_coordinates(bottom_right)),
//     ];

//     // Linear interpolation for points above and below the particle
//     let r1 = (((grid_points[3].1.x - particle_pos.x) / (grid_points[3].1.x - grid_points[0].1.x)) * grid_points[0].0)
//         + (((particle_pos.x - grid_points[0].1.x) / (grid_points[3].1.x - grid_points[0].1.x)) * grid_points[3].0);

//     let r2 = (((grid_points[2].1.x - particle_pos.x) / (grid_points[2].1.x - grid_points[1].1.x)) * grid_points[1].0)
//         + (((particle_pos.x - grid_points[1].1.x) / (grid_points[2].1.x - grid_points[1].1.x)) * grid_points[2].0);

//     // Y weights for each previously interpolated velocities
//     let weight_y1 = (grid_points[1].1.y - particle_pos.y) / (grid_points[1].1.y - grid_points[0].1.y);
//     let weight_y2 = (grid_points[2].1.y - particle_pos.y) / (grid_points[2].1.y - grid_points[3].1.y);

//     let interp_velocity = (weight_y1 * r1) + (weight_y2 * r2);

//     interp_velocity

// }

/**
    Uses linear interpolation to find the velocity of the
    particle interpolated from the nearest grid points for
    the cell.
*/
pub fn interpolate_velocity(particle_pos: Vec2, grid: &SimGrid) -> Vec2 {

    // Grid points 0..3 are the four corners of the bilinear interpolation
    // in order of clockwise rotation around the particle point.
    // https://en.wikipedia.org/wiki/Bilinear_interpolation

    let cell_coords = grid.get_cell_coordinates_from_position(&particle_pos);
    let cell_center = grid.get_cell_position_from_coordinates(cell_coords);
    let half_cell = grid.cell_size as f32 / 2.0;

    let row = cell_coords.x as usize;
    let col = cell_coords.y as usize;

    let left_u_pos = cell_center - Vec2::new(half_cell, 0.0);
    let right_u_pos = cell_center + Vec2::new(half_cell, 0.0);
    let top_v_pos = cell_center + Vec2::new(0.0, half_cell);
    let bottom_v_pos = cell_center - Vec2::new(0.0, half_cell);

    let left_u_velocity = grid.velocity_u[row][col];
    let top_v_velocity = grid.velocity_v[row][col];
    let right_u_velocity = grid.velocity_u[row][col + 1];
    let bottom_v_velocity = grid.velocity_v[row + 1][col];


    let interp_velocity_u = (((right_u_pos.x - particle_pos.x) / (right_u_pos.x - left_u_pos.x)) * left_u_velocity) + (((particle_pos.x - left_u_pos.x) / (right_u_pos.x - left_u_pos.x)) * right_u_velocity);
    let interp_velocity_v = ((top_v_pos.y - particle_pos.y) / (top_v_pos.y - bottom_v_pos.y) * bottom_v_velocity) + (((particle_pos.y - bottom_v_pos.y) / (top_v_pos.y - bottom_v_pos.y)) * top_v_velocity);

    let interp_velocity = Vec2::new(interp_velocity_u, interp_velocity_v);


    interp_velocity

}
