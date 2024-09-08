use std::process::Command;

fn main() {
    println!("Running SQLx migrations...");
    let status = Command::new("cargo")
        .args(&["sqlx", "migrate", "run"])
        .status()
        .expect("Failed to run SQLx migrations");

    if !status.success() {
        panic!("SQLx migrations failed.");
    }
    println!("SQLx migrations successful!");
}
