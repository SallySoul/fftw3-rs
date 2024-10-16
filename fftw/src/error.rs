use crate::array::Alignment;
use std::ffi::NulError;
use thiserror::Error;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid Plan")]
    InvalidPlanError {},

    #[error("Input array mismatch: expect={:?}, actual={:?}", expect, actual)]
    InputArrayMismatch {
        expect: (usize, Alignment),
        actual: (usize, Alignment),
    },

    #[error("Output array mismatch: expect={:?}, actual={:?}", expect, actual)]
    OutputArrayMismatch {
        expect: (usize, Alignment),
        actual: (usize, Alignment),
    },

    #[error("Failed to convert path into str: {:?}", path)]
    PathToStrConversionError { path: std::path::PathBuf },

    #[error("Failed to convert path into CString: {}", conversion_error)]
    PathToCStringConversionError { conversion_error: NulError },

    #[error("Failed to import wisdom file: {:?}", path)]
    ImportWisdomError { path: std::path::PathBuf },

    #[error("Failed to export wisdom file: {:?}", path)]
    ExportWisdomError { path: std::path::PathBuf },
}
