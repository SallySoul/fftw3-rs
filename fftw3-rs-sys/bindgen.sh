#!/bin/bash
set -uex

# To run:
# - Navigate to 'fftw-src/fftw-3.3.10/api'
# - Run 'zsh ../../../fftw-sys/bindgen.sh`
#
# Blacklist setting
#
# Types
# ------
# - "fftw.*_complex"
#   - Use `num_complex::Complex32` and `num_complex::Complex64`
# - "FILE"
#   - Use `libc::FILE` instead
# - "_.*"
#   - Remove unrelated
#
# Function
# ---------
# - "fftwl_.*"
#   - Disable `long double` interface
#
bindgen \
  --use-core \
  --with-derive-{default,eq,hash,ord} \
  --allowlist-type="^fftw.*" \
  --allowlist-var="^FFTW.*" \
  --allowlist-function="^fftw.*" \
  --blocklist-type="FILE" \
  --blocklist-type="_.*" \
  --blocklist-type="fftw.*_complex" \
  --blocklist-function="fftwl_.*" \
  --blocklist-type='__darwin_off_t' \
  --blocklist-type='fpos_t' \
  --default-enum-style=rust \
  fftw3.h \
  > ../../../fftw-sys/src/fftw.rs
