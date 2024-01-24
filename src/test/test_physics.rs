use bevy::math::Vec2;
use crate::simulation::util::interpolate_velocity;
use crate::simulation::SimGrid;

#[test]
fn interpolation_test() {
   let particle_pos = Vec2::new(12.0, 25.0);

   let mut grid = SimGrid::default();

   for row in 0..(grid.dimensions.1 + 1) as usize {
       for col in 0..grid.dimensions.0 as usize {
           grid.velocity_v[row][col] = -9.8;
       }
   }

   let interpolated_velocity = interpolate_velocity(particle_pos, &grid);

   assert_eq!(interpolated_velocity, Vec2::new(0.0, -9.8));
}
