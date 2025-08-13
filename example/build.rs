//!
//!  To Download with  OpenOCD:
//! ```
//! $ cargo br && openocd
//! ```
//!

use std::env::var;
use std::fs::write;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    cargo_emit::rerun_if_changed!("build.rs");
    cargo_emit::rerun_if_changed!("build.map");
    cargo_emit::rerun_if_changed!("interfaces");

    let package = env!("CARGO_PKG_NAME");

    // Output the build.map file   : This is useful for analysis.
    cargo_emit::rustc_link_arg!(format!("-Map={}/build.map", package));

    build_external_lib();

    write_openocd_config_file(package)?;

    Ok(())
}

///
/// # Writes the OpenOCD flash configuration file.
///
fn write_openocd_config_file(name: &str) -> std::io::Result<()> {
    let workdir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from(var("OUT_DIR").unwrap());

    let executable = out_dir
        .ancestors()
        .find(|p| p.ends_with("build"))
        .map(|p| p.parent())
        .flatten()
        .map(|p| p.join(name))
        .unwrap_or_default();

    write(
        workdir.join("openocd.cfg"),
        format!(
            "source [find ../openocd.cfg]\nprogram {} preverify verify reset exit",
            executable.display()
        ),
    )?;

    Ok(())
}

///
/// # Builds the external libraries
///
/// ## Usage
///
/// Compiles the external source file located in the `interfaces` directory
/// into a static library`. This library is automatically linked to the Rust
///  project during compilation.
///
/// The Rust interface for this library is provided through the `helper::bindings` module,
/// which contains the FFI (Foreign Function Interface) declarations.
///
/// ### in the `build.rs` file:
///
/// ```
/// cc::Build::new()
///     .file("interfaces/libmath.c")
///     .compile("math");
/// ```
///
/// ### in the `interfaces/libmath.c` file:
///
/// ```
/// #include <stdint.h>
///
/// uint8_t c_buffer[16] = {0};
///
/// int32_t c_add(int32_t a, int32_t b) { return a + b; }
/// int32_t c_sub(int32_t a, int32_t b) { return a - b; }
/// int32_t c_mul(int32_t a, int32_t b) { return a * b; }
/// int32_t c_div(int32_t a, int32_t b) { return a / b; }
/// int32_t c_mod(int32_t a, int32_t b) { return a % b; }
/// ```
///
/// ### in the `helper::bindings` module:
///
/// ```
/// use {super::prelude::hal, core::ffi::*};
///
/// #[link(name = "math", kind = "static")]
/// unsafe extern "C" {
///     pub static mut c_buffer: [c_uchar; 16];
///
///     pub fn c_add(a: c_int, b: c_int) -> c_int;
///     pub fn c_sub(a: c_int, b: c_int) -> c_int;
///     pub fn c_mul(a: c_int, b: c_int) -> c_int;
///     pub fn c_div(a: c_int, b: c_int) -> c_int;
///     pub fn c_mod(a: c_int, b: c_int) -> c_int;
/// }
/// ```
///
/// > Note the `name` in the `#[link]` attribute
///
fn build_external_lib() {
    cc::Build::new()
        .file("interfaces/libmath.c")
        .compile("math");
}
