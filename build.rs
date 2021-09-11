use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=/webapp");
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .current_dir("webapp")
            .args(["/C", "npm run build"])
            .output()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("npm run build")
            .current_dir("webapp")
            .output()
    }
    .expect("could not build site");
}
