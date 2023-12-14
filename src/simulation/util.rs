use bevy::math::Vec2;

pub fn find_influence(
    particle_pos_component: i32,
    grid_point_component: i32,
    grid_scale: i32) -> i32 {

    let diff = particle_pos_u - grid_point_u;

    if diff / grid_scale > 0 {
        return 1 - diff;
    } else if diff / grid_scale < 0 {
        return 1 + diff;
    } else {
        return 0;
    }
}
