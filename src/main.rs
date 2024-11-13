use clap::Parser;

// TODO: Subcommands for each step of the conversion process
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the archive to extract
    #[arg(short, long)]
    archive: Option<String>,
}

const DEBUG_FILE: &str = "target/debug/apathy.wrs";

fn main() {
    let args = Args::parse();

    let archive = args.archive.unwrap_or(DEBUG_FILE.to_string());

    println!("Extracting archive: {}", archive);

    match conv_archive::extract_archive(&archive, &"TODO".to_string()) {
        Ok(_) => println!("Extraction complete!"),
        Err(e) => eprintln!("Error: {}", e),
    }

    //println!("Conversion complete!");
}
