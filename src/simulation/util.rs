use crate::error::Error;
use bevy::math::Vec2;

use super::SimGrid;

pub type Result<T> = core::result::Result<T, Error>;

/**
    Find the weight of influence of a particle
    to a grid point.
*/
pub fn find_influence(particle_pos: Vec2, grid_point: Vec2, grid_scale: u16) -> f32 {
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

    let interp_velocity_u = (((right_u_pos.x - particle_pos.x) / (right_u_pos.x - left_u_pos.x))
        * left_u_velocity)
        + (((particle_pos.x - left_u_pos.x) / (right_u_pos.x - left_u_pos.x)) * right_u_velocity);
    let interp_velocity_v = ((top_v_pos.y - particle_pos.y) / (top_v_pos.y - bottom_v_pos.y)
        * bottom_v_velocity)
        + (((particle_pos.y - bottom_v_pos.y) / (top_v_pos.y - bottom_v_pos.y)) * top_v_velocity);

    let interp_velocity = Vec2::new(interp_velocity_u, interp_velocity_v);

    interp_velocity
}
