use std::ffi::OsStr;
use std::fs::File;
use std::path::{Path, PathBuf};

use image::ImageFormat;

// TODO: Choose between png and jpeg output
pub fn convert_image(file: &PathBuf) -> Result<(), &'static str> {
    let name = file
        .file_stem()
        .unwrap_or(OsStr::new("Unknown"))
        .to_string_lossy()
        .to_string();

    let image = image::open(Path::new(&file)).unwrap();

    let out_path = format!("{}/{}.png", file.parent().unwrap().to_str().unwrap(), name);

    let out = &mut File::create(Path::new(&out_path)).unwrap();

    image.write_to(out, ImageFormat::Png).unwrap();

    Ok(())
}
