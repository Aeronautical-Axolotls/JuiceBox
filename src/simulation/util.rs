use bevy::math::Vec2;
use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

pub fn find_influence(
    particle_pos_component: f32,
    grid_point_component: f32,
    grid_scale: u16) -> f32 {

    let diff = particle_pos_component - grid_point_component;

    let scaled_diff = (diff as f32) / (grid_scale as f32);

    if scaled_diff > 0.0 {
        return 1.0 - scaled_diff;
    } else if scaled_diff < 0.0 {
        return 1.0 + scaled_diff;
    } else {
        return 0.0;
    }
}

pub fn linear_interpolate(pos_1: Vec2, pos_2: Vec2, velocity_1: f32, velocity_2: f32, particle_pos: Vec2, horizontal: bool) -> f32 {

    let mut interp_velocity = 0.0;

    if horizontal {
        interp_velocity = ((velocity_1 * (pos_2[0] - pos_1[0])) + (velocity_2 * (particle_pos[0] - pos_1[0]))) / (pos_2[0] - pos_1[0]);
    } else {
        interp_velocity = ((velocity_1 * (pos_2[1] - pos_1[1])) + (velocity_2 * (particle_pos[1] - pos_1[1]))) / (pos_2[1] - pos_1[1]);
    }

    interp_velocity

}
