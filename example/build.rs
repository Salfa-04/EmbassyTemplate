fn main() {
    cargo_emit::rerun_if_changed!("build.rs");
    cargo_emit::rerun_if_changed!("interfaces");

    cc::Build::new()
        .file("interfaces/libmath.c")
        .compile("math")
}
