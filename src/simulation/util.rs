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
