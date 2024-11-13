use std::path::Path;

use clap::Parser;

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

const DEBUG_FILE: &str = "target/debug/apathy.wrs";
const DEBUG_OUTPUT: &str = "target/debug/apathy";

fn main() {
    let args = Args::parse();

    let archive = args.archive.unwrap_or(DEBUG_FILE.to_string());
    let output = args.output.unwrap_or(DEBUG_OUTPUT.to_string());

    println!("Extracting archive: {}", archive);

    // Create the output directory if it doesn't exist
    if !Path::new(&output).exists() {
        std::fs::create_dir(&output).unwrap();
    }

    // Extract the archive
    match conv_archive::extract_archive(&archive, &output) {
        Ok(_) => println!("Extraction complete!"),
        Err(e) => eprintln!("Error: {}", e),
    }

    //println!("Conversion complete!");
}
