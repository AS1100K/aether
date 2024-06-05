fn main() {
    if cfg!(feature = "trial") {
        use std::fs;
        use std::process::Command;

        let output = Command::new("curl")
            .arg("http://worldtimeapi.org/api/timezone/Etc/UTC")
            .output()
            .expect("Failed to fetch time");

        let response = String::from_utf8_lossy(&output.stdout);
        let json: serde_json::Value =
            serde_json::from_str(&response).expect("Failed to parse JSON");

        let current_time = json["datetime"]
            .as_str()
            .expect("Failed to get datetime")
            .to_string();

        // Write the current time to a file
        fs::write(
            "src/build_time.rs",
            format!("pub const BUILD_TIME: &str = \"{}\";", current_time),
        )
        .expect("Unable to write file");
    }

    println!("cargo:rerun-if-changed=config.json");
}
