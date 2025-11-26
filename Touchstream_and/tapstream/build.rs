// build.rs - Compile Slint UI files

fn main() {
    slint_build::compile("ui/tapstream.slint").unwrap();
}
