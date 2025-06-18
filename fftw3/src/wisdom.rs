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

macro_rules! impl_wisdom_call {
    ($(#[$attr:meta])* => $name:ident, $exec:ident, $err:ident) => {
        $(#[$attr])*
        pub fn $name<P: AsRef<Path>>(filename: &P) -> Result<()> {
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
                    result = $exec(c_path_string.as_ptr())
            }
            if result != 1 {
                return Err(Error::$err {
                    path: filename.as_ref().to_owned(),
                });
            }
            Ok(())
        }
    };
} // impl_wisdom_call!

impl_wisdom_call!(
/// Import an existing wisdom file for double precision plans
///
/// See [Wisdom Import](http://www.fftw.org/fftw3_doc/Wisdom-Import.html)
/// for more information.
=>
    import_wisdom_file_f64,
    fftw_import_wisdom_from_filename,
    ImportWisdomError
);

impl_wisdom_call!(
/// Import an existing wisdom file for single precision plans
///
/// See [Wisdom Import](http://www.fftw.org/fftw3_doc/Wisdom-Import.html)
/// for more information.
=>
    import_wisdom_file_f32,
    fftwf_import_wisdom_from_filename,
    ImportWisdomError
);

impl_wisdom_call!(
/// Export wisdom for double precision plans to file
///
/// See [Wisdom Export](http://www.fftw.org/fftw3_doc/Wisdom-Export.html)
/// for more information.
=>
    export_wisdom_file_f64,
    fftw_export_wisdom_to_filename,
    ExportWisdomError
);

impl_wisdom_call!(
/// Export wisdom for single precision plans to file
///
/// See [Wisdom Export](http://www.fftw.org/fftw3_doc/Wisdom-Export.html)
/// for more information.
=>
    export_wisdom_file_f32,
    fftwf_export_wisdom_to_filename,
    ExportWisdomError
);
