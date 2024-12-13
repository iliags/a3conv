use clap::ValueEnum;
use image::ImageFormat;
use std::ffi::OsStr;
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputImageFormat {
    Png,
    Jpeg,
}

impl OutputImageFormat {
    pub fn to_image_format(&self) -> ImageFormat {
        match self {
            OutputImageFormat::Png => ImageFormat::Png,
            OutputImageFormat::Jpeg => ImageFormat::Jpeg,
        }
    }

    pub fn to_extension(&self) -> &'static str {
        match self {
            OutputImageFormat::Png => "png",
            OutputImageFormat::Jpeg => "jpeg",
        }
    }
}

// TODO: Choose between png and jpeg output
pub fn convert_image(
    file: &PathBuf,
    output_dir: &PathBuf,
    output_format: OutputImageFormat,
) -> Result<(), &'static str> {
    let name = file
        .file_stem()
        .unwrap_or(OsStr::new("Unknown"))
        .to_string_lossy()
        .to_string();

    let image = image::open(Path::new(&file)).unwrap();

    let out_path = format!(
        "{}/{}.{}",
        output_dir.to_str().unwrap(),
        name,
        output_format.to_extension()
    );

    let out = &mut File::create(Path::new(&out_path)).unwrap();

    image
        .write_to(out, output_format.to_image_format())
        .unwrap();

    Ok(())
}
