// core/build.rs
fn main() {
    cc::Build::new()
        .file("weave/src/main.c")  // Link do weave C
        .compile("weave");
    println!("cargo:rerun-if-changed=weave/src/");
}
