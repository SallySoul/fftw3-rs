fftw3-rs
===========
![Rust](https://github.com/sallysoul/fftw3-rs/workflows/Rust/badge.svg)

Rust bindings for the [FFTW3 C-library](http://www.fftw.org/) for computing discrete Fourier transforms, as well as discrete cosine and sine transforms.

===========

This project is a fork of [fftw](https://github.com/rust-math/fftw).
The notable additions are:

- Updated to FFTW3-3.3.10 
- Support for using [wisdom](https://www.fftw.org/doc/Wisdom.html)
- Support for multi-threaded plans.
- Support for using [Rayon](https://github.com/rayon-rs/rayon) to execute parallel plans.

This repository includes three crates:

- `fftw`: A safe wrapper in Rust
- `fftw-sys`: An unsafe wrapper in Rust
- `fftw-src`: A crate for downloading and compiling the FFTW library

Feature flags
--------------

- `source`: Download and compile FFTW (default)
    - (Linux, macOS) Needs a C-compiler and the `make` build tool to compile the FFTW library
    - (Windows) Downloads a precompiled binary from the [FFTW website](http://www.fftw.org/install/windows.html)
- `system`: Use the system's libfftw3 (experimental)
    - You must install FFTW before building this crate
    - For Linux systems, please install FFTW using your package manager, e.g. in Debian or Ubuntu run `apt install libfftw3-dev`
    - For macOS, please run `brew install fftw` by using [homebrew](https://github.com/Homebrew/brew)
    - This feature is unsupported on Windows
- `intel-mkl` Use Intel MKL backend through [intel-mkl-src](https://github.com/termoshtt/rust-intel-mkl)
    - Only Linux and Windows are supported
- `threading` Utilizes a threading-enabled FFTW3 build, with additional rust bindings.
    - Includes optional ability to register Rayon based thread callback with `fftw_threads_set_callback`.
    - Rayon callback includes a `profiling::scope`

|Feature  | Linux | Windows | macOS |
|:--------|:-----:|:-------:|:-----:|
|source   |✔️      |✔️        |✔️      |
|system   |✔️      |-        |✔️      |
|intel-mkl|✔️      |✔️        |-      |

LICENSE
--------
See [LICENSE.md](./LICENSE.md)
