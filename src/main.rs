#![allow(dead_code, unused_imports)]
use clap::Parser;
use std::{
    fs::{self},
    path::{Path, PathBuf},
};

// TODO: Subcommands for each step of the conversion process
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the archive to extract
    #[arg(short, long)]
    archive: Option<String>,

    /// Name of the archive to extract
    #[arg(short, long)]
    output: Option<String>,
}

const DEBUG_OUTPUT: &str = "target/debug/apathy";
const DEBUG_FILE: &str = "target/debug/apathy.wrs";
const DEBUG_MAP: &str = "target/debug/apathy.WMP";
const DEBUG_IMG: &str = "target/debug/apathy/palblack.pcx";

fn main() {
    let args = Args::parse();

    let _archive = args.archive.unwrap_or(DEBUG_FILE.to_string());
    let output = args.output.unwrap_or(DEBUG_OUTPUT.to_string());

    // Create the output directory if it doesn't exist
    if !Path::new(&output).exists() {
        std::fs::create_dir(&output).unwrap();
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
