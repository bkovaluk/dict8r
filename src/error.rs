use thiserror::Error;
use std::io;
use serde_json::Error as SerdeError;

#[derive(Error, Debug)]
pub enum Dict8rError {
    #[error("error reading file: {0}")]
    IOError(#[from] io::Error),
    #[error("error parsing JSON: {0}")]
    JSONError(#[from] SerdeError),
    #[error("error in processing: {source}")]
    ProcessingError { source: io::Error },
}

// Remove the explicit Display impl below
// impl fmt::Display for Dict8rError {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        write!(f, "{}", self)
//    }
// }
