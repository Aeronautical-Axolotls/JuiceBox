#[derive(thiserror::Error, Debug)]
pub enum Error {

    #[error("Grid layout invalidly set: `{0}`")]
    GridSizeError(&'static str),
	
	#[error("Mismatched vector lengths: `{0}`")]
	VectorLengthMismatch(&'static str),
	
	#[error("No particles found: `{0}`")]
	NoParticlesFound(&'static str),
	
	#[error("Out of grid bounds: `{0}`")]
	OutOfGridBounds(&'static str),
	
	#[error("Invalid cell to create particles: `{0}`")]
	InvalidCellParticleCreation(&'static str),
	
	#[error("Invalid entity ID: `{0}`")]
	InvalidEntityID(&'static str),
}
