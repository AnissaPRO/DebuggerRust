fn main() {
    println!("cargo:rerun-if-changed=src/app-windows.slint");
    slint_build::compile("src/app-window.slint").expect("Slint build failed");
}
