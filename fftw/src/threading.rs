//! Threading
//!
//! FFTW3 supports multi-threaded plan execution.
//! This module essentially exports several
//! raw FFTW3 operations, so it's important to understand
//! how to use multi-threaded FFT, and what this crate takes care of.
//! The [Multi-threaded FFTW](http://www.fftw.org/fftw3_doc/Multi_002dthreaded-FFTW.html) doc is
//! worth a read.
//!
//! FFTW3 supports single precision (f32) and double-precision (f64) by
//! providing two separate libraries, `fftwf` and `fftw`.
//! This detail is hidden by the crate's plan generation module,
//! but not here.
//! The main thing to keep in mind is that the planners are separate for these
//! two libraries, so use the f32 and f64 variants as needed by your application.
//!
//! When using mutli-threaded plans, you must first call `init_threads_*`.
//! Then, we need to register the thread callback we want to use.
//! Currently a rayon based implementation is provided with the `rayon` feature.
//! Other threading backends can easily be added.
//!
//! FFTW plan generation is single threaded, and this crates protects access
//! to these single threaded operations with a mutex.
//! Plan execution does not modify the plan when data buffers are provided,
//! as this crate does, which makes plan execution safe to to
//! call from multiple threads.
//!
//! The planner is also a state machine, so prior to creating a multi-threaded plan,
//! set the desired number threads with `plan_with_nthreads_*`.
//! You can inspect the planners current thread setting with `planner_nthreads_*`.
use crate::error::*;
use crate::ffi::*;
use crate::*;

macro_rules! impl_init_threads {
    ($(#[$attr:meta])* => $name:ident, $exec:ident) => {
        $(#[$attr])*
        pub fn $name() -> Result<()> {
            let result: i32;
            excall! {
                result = $exec()
            }
            if result != 1 {
                return Err(Error::InitThreadError);
            }
            Ok(())
        }
    };
}

impl_init_threads!(
/// Initialize multi-threading for single precision FFTW3 plans.
=>
init_threads_f32,
fftwf_init_threads
);

impl_init_threads!(
/// Initialize multi-threading for double precision FFTW3 plans.
=>
init_threads_f64,
fftw_init_threads
);

macro_rules! impl_plan_with_nthreads {
    ($(#[$attr:meta])* => $name:ident, $exec:ident) => {
        $(#[$attr])*
        pub fn $name(n: usize) {
            excall! {
                $exec(n as core::ffi::c_int)
            }
        }
    };
}

impl_plan_with_nthreads!(
/// Set FFTW planner to use `n` threads when creating single precision plans.
=>
plan_with_nthreads_f32,
fftwf_plan_with_nthreads
);

impl_plan_with_nthreads!(
/// Set FFTW planner to use `n` threads when creating double precision plans.
=>
plan_with_nthreads_f64,
fftw_plan_with_nthreads
);

macro_rules! impl_planner_nthreads {
    ($(#[$attr:meta])* => $name:ident, $exec:ident) => {
        $(#[$attr])*
        pub fn $name() -> i32{
            let result: i32;
            excall! {
                result = $exec()
            }
            result as i32
        }
    };
}

impl_planner_nthreads!(
/// Query how many threads the planner will use for single precision plans.
=>
planner_nthreads_f32,
fftwf_planner_nthreads
);

impl_planner_nthreads!(
/// Query how many threads the planner will use for double precision plans.
=>
planner_nthreads_f64,
fftw_planner_nthreads
);
