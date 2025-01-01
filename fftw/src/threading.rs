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
//! FFTW plan generation is not thread safe, and this crates protects access
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
use rayon::prelude::*;
use sync_ptr::SyncMutPtr;

// This is a direct translation of the OpenMP example from the FFTW documentation.
// One notable issue is that '* mut' pointers are not Send + Sync,
// which prevents us from passing it into the ParallelIterator::for_each closure.
// This is why we use the sync-ptr crate to wrap the jobdata pointer in
// a struct that is explicitly Send + Sync.
//
// FWIW, this seemed to be an old decision in Rust that will likely never change.
extern "C" fn rayon_thread_callback(
    work: ::core::option::Option<
        unsafe extern "C" fn(arg1: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_void,
    >,
    jobdata: *mut ::core::ffi::c_char,
    elsize: usize,
    njobs: ::core::ffi::c_int,
    _: *mut ::core::ffi::c_void,
) {
    let sync_jobdata = unsafe { SyncMutPtr::new(jobdata) };
    (0..njobs).into_par_iter().for_each(|job| unsafe {
        let arg = sync_jobdata.inner().add(job as usize * elsize);
        work.unwrap()(arg);
    });
}

macro_rules! impl_init_threads {
    ($(#[$attr:meta])* => $name:ident, $init_exec:ident, $callback_exec:ident) => {
        $(#[$attr])*
        pub fn $name() -> Result<()> {
            let result: i32;
            excall! {
                result = $init_exec()
            }
            if result != 1 {
                return Err(Error::InitThreadError);
            }
            excall! {
                $callback_exec(Some(rayon_thread_callback), core::ptr::null_mut::<core::ffi::c_void>())
            }
            Ok(())
        }
    };
}

impl_init_threads!(
/// Initialize multi-threading for single precision FFTW3 plans
/// using rayon thread pool.
=>
init_threads_f32,
fftwf_init_threads,
fftwf_threads_set_callback
);

impl_init_threads!(
/// Initialize multi-threading for double precision FFTW3 planners
/// using rayon thread pool.
=>
init_threads_f64,
fftw_init_threads,
fftw_threads_set_callback
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
