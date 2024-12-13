#![allow(dead_code, unused_imports)]
use a3conv::image::OutputImageFormat;
use clap::{Parser, ValueEnum};
use core::arch;
use std::{
    env,
    fs::{self},
    path::{Path, PathBuf},
    result,
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
    #[arg(value_enum)]
    image_format: Option<OutputImageFormat>,
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

const DEBUG_OUTPUT: &str = "target/debug/apathy";
const DEBUG_FILE: &str = "target/debug/apathy.wrs";
const DEBUG_MAP: &str = "target/debug/apathy.WMP";
const DEBUG_IMG: &str = "target/debug/apathy/palblack.pcx";

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

    println!("Output: {:?}", output);

    // Create the output directory if it doesn't exist
    if !Path::new(&output).exists() {
        std::fs::create_dir(&output).unwrap();
    }

    let archives: Vec<String> = match extract_mode {
        ExtractMode::GameDir => {
            // Scan current directory for archives
            let mut results = Vec::new();
            let files = fs::read_dir(&game_dir).unwrap();
            files
                .filter_map(Result::ok)
                .filter(|d| {
                    if let Some(e) = d.path().extension() {
                        e == "wrs"
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

    for archive in archives {
        let archive_name = archive.split('.').next().unwrap();
        let output_directory = format!("{}/{}", output, archive_name);
        //println!("Extracting archive: {}", archive_name);

        // Create the output directory if it doesn't exist
        if !Path::new(&output_directory).exists() {
            std::fs::create_dir(&output_directory).unwrap();
        }

        // Extract the archive
        match a3conv::wrs::extract_archive(&archive, &output_directory) {
            Ok(_) => println!("Extracted archive: {}", archive),
            Err(e) => eprintln!("Error: {}", e),
        }

        // Convert the extracted files
    }

    /*
    println!("Extracting archive: {}", archive);

    // Extract the archive
    match a3conv::wrs::extract_archive(&archive, &output) {
        Ok(_) => println!("Extraction complete!"),
        Err(e) => eprintln!("Error: {}", e),
    }

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

    match a3conv::image::convert_image(&PathBuf::from(DEBUG_IMG)) {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
     */

    //println!("Conversion complete!");
}
