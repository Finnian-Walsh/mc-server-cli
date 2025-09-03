fn main() {
    println!("cargo::rustc-check-cfg=cfg(config_generated)");
}
