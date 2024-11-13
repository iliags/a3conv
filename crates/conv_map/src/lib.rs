#![allow(dead_code, unused_variables)]
use face_id::Face;
use nalgebra::Vector3;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Error, ErrorKind, Write};

pub mod face_id;

type Vertex = Vector3<f32>;

#[derive(Debug)]
pub struct Region {
    name: String,
    floor_height: f32,
    ceiling_height: f32,
}

#[derive(Debug, Default, Clone)]
pub struct Wall {
    name: String,
    vertex1_index: usize,
    vertex2_index: usize,
    region1_index: usize,
    region2_index: usize,

    // Offsets are used for texture alignment
    offset_x: f32,
    offset_y: f32,

    // Textures are read from the *.wdl file
    wall_texture: String,
    floor_texture: String,
    ceiling_texture: String,
}

#[derive(Debug)]
enum LineType {
    Vertex,
    Region,
    Wall,
    Comment,
}

#[derive(Debug)]
pub struct Graph {
    vertices: Vec<Vertex>,
    edges: HashMap<usize, Vec<usize>>, // Adjacency list
}

pub fn parse_wmp(filename: &str) -> Result<(Vec<Vertex>, Vec<Region>, Vec<Wall>), std::io::Error> {
    //Error::new(std::io::ErrorKind::NotFound, "File not found")
    //let file = File::open(filename).unwrap_or_else(|_| panic!("Failed to open file: {}", filename));
    let file = match File::open(filename) {
        Ok(file) => Ok(file),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => Err(std::io::Error::new(
                ErrorKind::NotFound,
                format!("File not found: {}", filename),
            )),
            other_error => Err(Error::new(other_error, "Failed to open file")),
        },
    };

    // The file should be safe to unwrap as errors are handled above
    let reader = BufReader::new(file.unwrap());

    let mut vertices = Vec::new();
    let mut regions = Vec::new();
    let mut walls = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        let line = line.split_once(';').map_or(line, |(before, _)| before); // Trim everything after ";"
        let line = line.trim_end();

        if line.is_empty() || line.starts_with('#') {
            continue; // Skip empty lines and comments
        }

        let parts: Vec<&str> = line.split_whitespace().collect();

        let line_type = match parts[0] {
            "VERTEX" => LineType::Vertex,
            "REGION" => LineType::Region,
            "WALL" => LineType::Wall,
            _ => LineType::Comment,
        };

        match line_type {
            LineType::Vertex => {
                // Parse vertex data
                let x: f32 = parts[1].parse().unwrap_or_default();
                let y: f32 = parts[2].parse().unwrap_or_default();
                let z: f32 = parts[3].parse().unwrap_or_default();
                //vertices.push(Vertex { x, y, z });
                vertices.push(Vector3::new(x, y, z));
            }
            LineType::Region => {
                // Parse region data
                let name = parts[1].to_string();
                let floor_hgt: f32 = parts[2].parse().unwrap_or_default();
                let ceil_hgt: f32 = parts[3].parse().unwrap_or_default();
                regions.push(Region {
                    name,
                    floor_height: floor_hgt,
                    ceiling_height: ceil_hgt,
                });
            }
            LineType::Wall => {
                // Parse wall data
                let name = parts[1].to_string();
                let vertex1_index: usize = parts[2].parse().unwrap_or_default();
                let vertex2_index: usize = parts[3].parse().unwrap_or_default();
                let region1_index: usize = parts[4].parse().unwrap_or_default();
                let region2_index: usize = parts[5].parse().unwrap_or_default();
                let offset_x: f32 = parts[6].parse().unwrap_or_default();
                let offset_y: f32 = parts[7].parse().unwrap_or_default();
                walls.push(Wall {
                    name,
                    vertex1_index,
                    vertex2_index,
                    region1_index,
                    region2_index,
                    offset_x,
                    offset_y,
                    ..Default::default()
                });
            }
            _ => {}
        }
    }

    Ok((vertices, regions, walls))
}

pub fn build_graph(vertices: Vec<Vertex>, walls: Vec<Wall>) -> Graph {
    let mut graph = Graph {
        vertices,
        edges: HashMap::new(),
    };

    for wall in walls {
        graph
            .edges
            .entry(wall.vertex1_index)
            .or_default()
            .push(wall.vertex2_index);
        graph
            .edges
            .entry(wall.vertex2_index)
            .or_default()
            .push(wall.vertex1_index);
    }

    graph
}

pub fn write_output(
    faces: Vec<Face>,
    normals: Vec<Vector3<f32>>,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // ... implementation
    Ok(())
}

pub fn write_to_obj(
    faces: &[Face],
    vertices: &[Vertex],
    normals: &[Vector3<f32>],
    output_file: &str,
) -> Result<(), std::io::Error> {
    let file = File::create(output_file)?;
    let mut writer = BufWriter::new(file);

    // Write vertices
    for (i, vertex) in vertices.iter().enumerate() {
        write!(writer, "v {} {} {}\n", vertex.x, vertex.y, vertex.z)?;
    }

    // Write normals
    for (i, normal) in normals.iter().enumerate() {
        write!(writer, "vn {} {} {}\n", normal.x, normal.y, normal.z)?;
    }

    // Write faces
    for face in faces {
        write!(writer, "f")?;
        for (i, vertex_index) in face.outer_loop.iter().enumerate() {
            write!(writer, " {}//{}", vertex_index + 1, i + 1)?;
        }
        writeln!(writer)?;
    }

    Ok(())
}

/* Old code from main.rs
struct Args {
    /// Name of the map (*.wmp) to convert
    #[arg(short, long)]
    map: String,

    /// Name of the output file
    #[arg(short, long)]
    output: String,
}

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
*/