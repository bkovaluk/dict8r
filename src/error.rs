use thiserror::Error;

use std::io;
use serde_json::Error as SerdeError;

/// An enumeration of all potential errors that can occur in the dictionary processing functions.
///
/// This enum uses `thiserror::Error` to automatically implement `std::error::Error`
/// based on the annotations provided. It simplifies error handling by categorizing
/// and encapsulating common error types with helpful messages.
///
/// # Variants
///
/// * `IOError` - Wraps `std::io::Error`, indicating failures related to IO operations.
/// * `JSONError` - Wraps `serde_json::Error`, indicating failures in JSON parsing.
/// * `ProcessingError` - Represents errors that occur during processing, with an included source error.
#[derive(Error, Debug)]
pub enum Dict8rError {
    #[error("error reading file: {0}")]
    IOError(#[from] io::Error),

    #[error("error parsing JSON: {0}")]
    JSONError(#[from] SerdeError),

    #[error("error in processing: {source}")]
    ProcessingError { source: io::Error },
}
