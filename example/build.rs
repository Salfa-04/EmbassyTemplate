fn main() {
    println!("cargo:rerun-if-changed=links");
    println!("cargo:rerun-if-changed=build.rs");

    cc::Build::new().file("links/libmath.c").compile("math")
}
