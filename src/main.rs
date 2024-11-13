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
    let _args = Args::parse();

    println!("Conversion complete!");
}
