use a3conv::*;
use clap::Parser;

/// Convert A3 maps to mesh data using a provided *.wmp and *.wdl file
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the map (*.wmp) to convert
    #[arg(short, long)]
    map: String,

    /// Name of the output file
    #[arg(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();

    let (vertices, regions, walls) = match parse_wmp(&args.map) {
        Ok((vertices, regions, walls)) => (vertices, regions, walls),
        Err(error) => {
            eprintln!("Failed to parse map: {}", error);
            return;
        }
    };

    println!("Vertices: {:?}", vertices.len());
    println!("Regions: {:?}", regions.len());
    println!("Walls: {:?}", walls.len());
}
