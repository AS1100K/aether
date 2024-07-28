use std::env;

fn main() {
    let mut enabled_features = vec![];

    if env::var("CARGO_FEATURE_CRATES_IO").is_ok() {
        enabled_features.push("crates_io");
    }
    if env::var("CARGO_FEATURE_GIT").is_ok() {
        enabled_features.push("git");
    }
    if env::var("CARGO_FEATURE_GIT_1_20_6").is_ok() {
        enabled_features.push("git_1_20_6");
    }

    if enabled_features.len() > 1 {
        panic!(
            "Multiple azalea features enabled: {:?}. Only one feature can be enabled at a time.",
            enabled_features
        );
    } else if enabled_features.len() == 0 {
        panic!(
            "One azalea features needs to be enabled in order to determine the azalea version to use."
        );
    }
}