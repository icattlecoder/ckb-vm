use cc::Build;

fn main() {
    Build::new()
        .file("src/asm.x64.c")
        .include("dynasm")
        .compile("asm");
}
