#![allow(dead_code, unused_imports)]
use a3conv::image::OutputImageFormat;
use clap::{Parser, ValueEnum};
use core::arch;
use std::{
    env,
    fs::{self},
    path::{Path, PathBuf},
    result, vec,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(flatten)]
    input: InputGroup,

    /// Output directory for the extracted files
    #[arg(short, long)]
    output: Option<String>,

    /// Output image format, default is PNG
    #[arg(value_enum, default_value = "png")]
    image_format: Option<OutputImageFormat>,

    #[arg(short, long, default_value = "false")]
    convert_files: bool,
}

#[derive(Debug, clap::Args)]
#[group(required = true, multiple = false)]
pub struct InputGroup {
    /// Game directory to extract files from
    #[clap(short, long)]
    game_dir: Option<String>,

    /// The archive to extract
    #[clap(short, long)]
    archive: Option<String>,
}

enum ExtractMode {
    GameDir,
    Archive,
}

fn main() {
    let args = Args::parse();

    let extract_mode = match args.input.game_dir.is_some() {
        true => ExtractMode::GameDir,
        false => ExtractMode::Archive,
    };

    let game_dir = PathBuf::from(args.input.game_dir.unwrap_or("".to_string()));
    let archive = PathBuf::from(args.input.archive.unwrap_or("".to_string()));

    let output = match args.output {
        Some(output) => output,
        None => {
            if game_dir.exists() {
                format! {"{}/output", game_dir.to_str().unwrap()}
            } else {
                format! {"{}/output", archive.parent().unwrap().to_str().unwrap()}
            }
        }
    };

    let archives: Vec<String> = match extract_mode {
        ExtractMode::GameDir => {
            // Scan current directory for archives
            let mut results = Vec::new();
            let files = fs::read_dir(&game_dir).unwrap();
            files
                .filter_map(Result::ok)
                .filter(|d| {
                    if let Some(e) = d.path().extension() {
                        e == "wrs" || e == "WRS"
                    } else {
                        false
                    }
                })
                .for_each(|f| results.push(f.file_name().to_str().unwrap().to_string()));

            results
        }
        ExtractMode::Archive => {
            vec![archive.file_name().unwrap().to_str().unwrap().to_string()]
        }
    };

    println!("Archives: {:?}", archives);

    for archive in archives {
        let archive_name = archive.split('.').next().unwrap();
        let output_directory = format!("{}/{}", output, archive_name);
        let original_directory = format!("{}/original", output_directory);
        println!("Extracting archive: {}", archive_name);

        // Create the output directory if it doesn't exist
        if !Path::new(&original_directory).exists() {
            std::fs::create_dir_all(&original_directory).unwrap();
        }

        // Extract the archive
        match a3conv::wrs::extract_archive(&archive, &original_directory) {
            Ok(_) => (),
            Err(e) => eprintln!("Error: {}", e),
        }

        // Convert the extracted files

        if args.convert_files {
            println!("Converting files...");

            let converted_directory = format!("{}/converted", output_directory);
            let image_dir = format!("{}/images", converted_directory);
            let sound_dir = format!("{}/sound", converted_directory);
            let script_dir = format!("{}/script", converted_directory);

            let vec = vec![&converted_directory, &image_dir, &sound_dir, &script_dir];

            for dir in vec {
                if !Path::new(&dir).exists() {
                    std::fs::create_dir_all(&dir).unwrap();
                }
            }

            let files = fs::read_dir(&original_directory).unwrap();
            // Images
            files
                .filter_map(Result::ok)
                .for_each(|f| match f.path().extension() {
                    Some(e) => {
                        let file = f.file_name().to_str().unwrap().to_string();
                        let file = format!("{}/{}", original_directory, file);

                        match e.to_str().unwrap() {
                            "pcx" => {
                                let image_format =
                                    args.image_format.unwrap_or(OutputImageFormat::Png);

                                match a3conv::image::convert_image(
                                    &PathBuf::from(file),
                                    &PathBuf::from(&image_dir),
                                    image_format,
                                ) {
                                    Ok(_) => (),
                                    Err(e) => eprintln!("Image Error: {}", e),
                                }
                            }
                            "wav" => {
                                let target_file = format!(
                                    "{}/{}",
                                    sound_dir,
                                    f.file_name().to_str().unwrap().to_string()
                                );

                                let mut source = match std::fs::File::open(&file) {
                                    Ok(f) => f,
                                    Err(e) => {
                                        eprintln!("Error: {}", e);
                                        return;
                                    }
                                };
                                let mut target = match std::fs::File::create(&target_file) {
                                    Ok(f) => f,
                                    Err(e) => {
                                        eprintln!("Error: {}", e);
                                        return;
                                    }
                                };
                                match std::io::copy(&mut source, &mut target) {
                                    Ok(_) => (),
                                    Err(e) => eprintln!("Error: {}", e),
                                }
                            }
                            "wdl" | "wmp" => {
                                // TODO: Convert WDL and WMP files, for now just copy them

                                let target_file = format!(
                                    "{}/{}",
                                    script_dir,
                                    f.file_name().to_str().unwrap().to_string()
                                );

                                let mut source = match std::fs::File::open(&file) {
                                    Ok(f) => f,
                                    Err(e) => {
                                        eprintln!("Error: {}", e);
                                        return;
                                    }
                                };
                                let mut target = match std::fs::File::create(&target_file) {
                                    Ok(f) => f,
                                    Err(e) => {
                                        eprintln!("Error: {}", e);
                                        return;
                                    }
                                };
                                match std::io::copy(&mut source, &mut target) {
                                    Ok(_) => (),
                                    Err(e) => eprintln!("Error: {}", e),
                                }

                                /*
                                let mut map = a3conv::map::Map::default();

                                let path = PathBuf::from(DEBUG_MAP);
                                match map.parse_wmp(&path) {
                                    Ok(_) => {
                                        let output_file = format!("{}/{}.txt", output, map.name());
                                        println!("Writing to file: {:?}", output_file);
                                        fs::write(output_file, map.create_vertex_csv()).unwrap();
                                    }
                                    Err(e) => eprintln!("Error: {}", e),
                                }
                                */
                            }
                            _ => {}
                        }
                    }
                    None => {}
                });
        }
    }

    println!("Conversion complete!");
}
