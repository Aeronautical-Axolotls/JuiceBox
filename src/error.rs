#[derive(thiserror::Error, Debug)]
pub enum Error {

    #[error("Grid layout invalidly set: `{0}`")]
    GridSizeError(&'static str),

}
