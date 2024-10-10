//! Wisdom Files
//!
//! FFTW3 supports importing and exporting "wisdom" about plan execution.
//! See [Words of Wisdom - Saving
//! Plans](http://www.fftw.org/fftw3_doc/Words-of-Wisdom_002dSaving-Plans.html)
//! for more information.

use crate::error::*;
use crate::ffi::*;
use std::ffi::CString;
use std::path::Path;

/// Import an existing wisdom file
///
/// See [Wisdom Import](http://www.fftw.org/fftw3_doc/Wisdom-Import.html)
/// for more information.
pub fn import_wisdom_file<P: AsRef<Path>>(filename: &P) -> Result<()> {
    let path_str = match filename.as_ref().to_str() {
        Some(str) => str,
        None => {
            return Err(Error::PathToStrConversionError {
                path: filename.as_ref().to_owned(),
            })
        }
    };
    let c_path_string = match CString::new(path_str) {
        Ok(c_str) => c_str,
        Err(e) => {
            return Err(Error::PathToCStringConversionError {
                conversion_error: e,
            })
        }
    };

    let result: i32;
    excall! {
            result = fftw_import_wisdom_from_filename(c_path_string.as_ptr())
    }
    if result != 1 {
        return Err(Error::ImportWisdomError {
            path: filename.as_ref().to_owned(),
        });
    }
    Ok(())
}

/// Export wisdom to file
///
/// See [Wisdom Export](http://www.fftw.org/fftw3_doc/Wisdom-Export.html)
/// for more information.
pub fn export_wisdom_file<P: AsRef<Path>>(filename: &P) -> Result<()> {
    let path_str = match filename.as_ref().to_str() {
        Some(str) => str,
        None => {
            return Err(Error::PathToStrConversionError {
                path: filename.as_ref().to_owned(),
            })
        }
    };
    let c_path_string = match CString::new(path_str) {
        Ok(c_str) => c_str,
        Err(e) => {
            return Err(Error::PathToCStringConversionError {
                conversion_error: e,
            })
        }
    };

    let result: i32;
    excall! {
        result = fftw_export_wisdom_to_filename(c_path_string.as_ptr())
    }
    if result != 1 {
        return Err(Error::ExportWisdomError {
            path: filename.as_ref().to_owned(),
        });
    }
    Ok(())
}
