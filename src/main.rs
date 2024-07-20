use a3conv::*;
use clap::Parser;
use face_id::{calculate_face_normals, identify_faces};

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
    //println!("Vertices: {:?}", vertices);

    let graph = build_graph(vertices.clone(), walls.clone());
    //println!("Graph: {:?}", graph);

    let faces = identify_faces(&graph);
    //println!("Faces: {:?}", faces);

    let face_normals = calculate_face_normals(faces.clone(), &vertices);
    //println!("Face normals: {:?}", face_normals);

    //if let Err(error) = write_output(faces.clone(), face_normals, &args.output) {
    //    eprintln!("Failed to write output: {}", error);
    //}

    println!("Vertices: {:?}", vertices.len());
    println!("Regions: {:?}", regions.len());
    println!("Walls: {:?}", walls.len());

    if let Err(error) = write_to_obj(&faces, &vertices, &face_normals, &args.output) {
        eprintln!("Failed to write output: {}", error);
    }

    println!("Conversion complete!");
}
