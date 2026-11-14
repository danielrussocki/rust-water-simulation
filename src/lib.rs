use std::{fs::File, io::BufReader, path::Path};

pub const REQUIRED_ASSETS: [&str; 6] = [
    "tiles.jpg",
    "xneg.jpg",
    "xpos.jpg",
    "ypos.jpg",
    "zneg.jpg",
    "zpos.jpg",
];

#[derive(Debug)]
pub struct AssetError {
    pub name: &'static str,
    pub cause: String,
}

pub enum StartupDecision {
    Blocked(Vec<AssetError>),
    Ready,
}

pub fn startup_decision(directory: &Path) -> StartupDecision {
    let errors = validate_assets(directory);
    if errors.is_empty() {
        StartupDecision::Ready
    } else {
        StartupDecision::Blocked(errors)
    }
}

pub fn validate_assets(directory: &Path) -> Vec<AssetError> {
    REQUIRED_ASSETS
        .into_iter()
        .filter_map(|name| {
            let path = directory.join(name);
            let result = File::open(&path)
                .map_err(|error| error.to_string())
                .and_then(|file| {
                    image::load(BufReader::new(file), image::ImageFormat::Jpeg)
                        .map(|_| ())
                        .map_err(|error| error.to_string())
                });
            result.err().map(|cause| AssetError { name, cause })
        })
        .collect()
}
