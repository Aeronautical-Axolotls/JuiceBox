use bevy::math::Vec2;
use crate::simulation::util::interpolate_velocity;

#[test]
fn interpolation_test() {
   let particle_pos = Vec2::new(12.0, 25.0);

   let grid_points = vec![
        (Vec2::new(0.0, 9.8), Vec2::new(7.0, 20.0)),
        (Vec2::new(0.0, 9.8), Vec2::new(7.0, 30.0)),
        (Vec2::new(0.0, 9.8), Vec2::new(17.0, 30.0)),
        (Vec2::new(0.0, 9.8), Vec2::new(17.0, 20.0))
   ];

   let interpolated_velocity = interpolate_velocity(particle_pos, grid_points).unwrap();

   assert_eq!(interpolated_velocity, Vec2::new(0.0, 9.8));
}
