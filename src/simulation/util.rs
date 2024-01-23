use bevy::math::Vec2;
use crate::error::Error;

use super::SimGrid;

pub type Result<T> = core::result::Result<T, Error>;

pub fn find_influence(
    particle_pos: Vec2,
    grid_point: Vec2,
    grid_scale: u16) -> f32 {

    let diff = grid_point.distance(particle_pos);

    let scaled_diff = (diff as f32) / (grid_scale as f32);

    if scaled_diff > 1.5 {
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

pub fn linear_interpolate(pos_1: Vec2, pos_2: Vec2, velocity_1: f32, velocity_2: f32, particle_pos: Vec2, horizontal: bool) -> f32 {

    let mut interp_velocity;

    if horizontal {
        interp_velocity = ((velocity_1 * (pos_2[0] - pos_1[0])) + (velocity_2 * (particle_pos[0] - pos_1[0]))) / (pos_2[0] - pos_1[0]);
    } else {
        interp_velocity = ((velocity_1 * (pos_2[1] - pos_1[1])) + (velocity_2 * (particle_pos[1] - pos_1[1]))) / (pos_2[1] - pos_1[1]);
    }

    interp_velocity

}

/** Uses bilinear interpolation to find the velocity of the particle interpolated from the nearest grid points.
Each grid point in grid_points includes both the (u, v) components and (x, y) coordinates in that order. */
pub fn interpolate_velocity(particle_pos: Vec2, grid_points: &Vec<(Vec2, Vec2)>) -> Result<Vec2> {

    // Grid points 0..3 are the four corners of the bilinear interpolation
    // in order of clockwise rotation around the particle point.
    // https://en.wikipedia.org/wiki/Bilinear_interpolation


    if grid_points.len() != 4 {
        return Err(Error::Interpolation("incorrect number of grid points to interpolate!"))
    }

    let r1_u = (
            (
                (grid_points[2].1.x - particle_pos.x) / (grid_points[3].1.x - grid_points[0].1.x)
            ) *
            grid_points[0].0.x
        ) +
        (
            (
                (particle_pos.x - grid_points[0].1.x) / (grid_points[3].1.x - grid_points[0].1.x)
            ) *
            grid_points[1].0.x
        );

    let r1_v = (
            (
                (grid_points[2].1.x - particle_pos.x) / (grid_points[2].1.x - grid_points[0].1.x)
            ) *
            grid_points[0].0.y
        ) +
        (
            (
                (particle_pos.x - grid_points[0].1.x) / (grid_points[2].1.x - grid_points[0].1.x)
            ) *
            grid_points[1].0.y
        );

    let r2_u = (
            (
                (grid_points[2].1.x - particle_pos.x) / (grid_points[2].1.x - grid_points[0].1.x)
            ) *
            grid_points[2].0.x
        ) +
        (
            (
                (particle_pos.x - grid_points[0].1.x) / (grid_points[2].1.x - grid_points[0].1.x)
            ) *
            grid_points[3].0.x
        );

    let r2_v = (
            (
                (grid_points[2].1.x - particle_pos.x) / (grid_points[2].1.x - grid_points[0].1.x)
            ) *
            grid_points[2].0.y
        ) +
        (
            (
                (particle_pos.x - grid_points[0].1.x) / (grid_points[2].1.x - grid_points[0].1.x)
            ) *
            grid_points[3].0.y
        );

    let weight_y1 = (grid_points[2].1.y - particle_pos.y) / (grid_points[2].1.y - grid_points[0].1.y);
    let weight_y2 = (particle_pos.y - grid_points[0].1.y) / (grid_points[2].1.y - grid_points[0].1.y);

    let interp_velocity_u = (
            (
                weight_y1
            ) *
            r1_u
        ) +
        (
            (
                weight_y2
            ) *
            r2_u
        );

    let interp_velocity_v = (
            (
                weight_y1
            ) *
            r1_v
        ) +
        (
            (
                weight_y2
            ) *
            r2_v
        );


    let interp_velocity = Vec2::new(interp_velocity_u, interp_velocity_v);

    Ok(interp_velocity)

}
