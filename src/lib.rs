use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

#[derive(Debug)]
pub struct Vertex {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug)]
pub struct Region {
    name: String,
    floor_height: f32,
    ceiling_height: f32,
}

#[derive(Debug)]
pub struct Wall {
    vertex1: usize,
    vertex2: usize,
    region1: usize,
    region2: usize,

    // Offsets are used for texture alignment
    offsetx: f32,
    offsety: f32,

    // Textures are read from the *.wdl file
    wall_texture: String,
    floor_texture: String,
    ceiling_texture: String,
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
        // Parse line and create Vertex, Region, or Wall objects
    }

    Ok((vertices, regions, walls))
}
