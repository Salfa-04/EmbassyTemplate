//!
//!  To Download with OpenOCD:
//! openocd -f openocd.cfg -c "program ../path/to/elf preverify reset exit"
//!

fn main() {
    cargo_emit::rerun_if_changed!("build.rs");
    cargo_emit::rerun_if_changed!("build.map");
    cargo_emit::rerun_if_changed!("interfaces");

    // Output the build map file   : This is useful for analysis.
    cargo_emit::rustc_link_arg!(format!("-Map={}/build.map", env!("CARGO_PKG_NAME")));

    // cc::Build::new()
    //     .file("interfaces/libmath.c")
    //     .compile("math")
}
