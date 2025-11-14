fn main() {
    // Add a path to the linker search path - for some reason, cargo test fails without this.
    println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
}
