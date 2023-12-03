use bevy::math::Vec2;
use std::f32::consts::PI;

/// Converts degrees to radians; returns radians.
pub fn degrees_to_radians(degrees: f32) -> f32 {
	let radians: f32 = degrees * (PI / 180.0);
	radians
}

/// Converts radians to degrees; returns degrees.
pub fn radians_to_degrees(radians: f32) -> f32 {
	let degrees: f32 = radians * (180.0 / PI);
	degrees
}

/** Converts a polar vector with direction and magnitude into a cartesian vector with x and y 
	components; returns said cartesian vector.  **Note: polar vectors are of the form 
	(magnitude, angle-in-radians).** */
pub fn polar_to_cartesian(polar_vector: Vec2) -> Vec2 {
	let radius:	f32 = polar_vector[0];
	let theta:	f32 = polar_vector[1];
	
	let result: Vec2 = Vec2 {
		x: radius * theta.cos(), 
		y: radius * theta.sin(), 
	};
	
	result
}

/** Converts a cartesian vector with x and y components into a polar vector with direction and 
	magnitude; returns said polar vector.  **Note: polar vectors are of the form 
	(magnitude, angle-in-radians).** */
pub fn cartesian_to_polar(cartesian_vector: Vec2) -> Vec2 {
	let cx: f32 = cartesian_vector[0];
	let cy: f32 = cartesian_vector[1];
	
	let sum_of_squares: f32 = (cx * cx) + (cy * cy);
	
	let result: Vec2 = Vec2 {
		x: sum_of_squares.sqrt(),	// r = sqrt(x^2 + y^2)
		y: cy.atan2(cx),			// theta = arctan(y / x)
	};
	
	result
}