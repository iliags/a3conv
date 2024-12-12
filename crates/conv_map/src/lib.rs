#![allow(dead_code)]
use nalgebra::Vector3;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::PathBuf;

type Vertex = Vector3<f32>;

#[derive(Debug, Default, Clone)]
pub struct Map {
    name: String,
    vertices: Vec<Vertex>,
    regions: Vec<Region>,
    walls: Vec<Wall>,
}

#[derive(Debug, Default, Clone)]
pub struct Region {
    name: String,
    floor_height: f32,
    ceiling_height: f32,
}

impl Region {
    pub fn floor_height(&self) -> f32 {
        self.floor_height
    }

    pub fn ceiling_height(&self) -> f32 {
        self.ceiling_height
    }
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

impl Map {
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Load a map from a WMP file
    pub fn parse_wmp(&mut self, filename: &PathBuf) -> Result<(), std::io::Error> {
        self.name = filename
            .file_stem()
            .unwrap_or(OsStr::new("Unknown"))
            .to_string_lossy()
            .to_string();

        //println!("Parsing WMP file: {:?}", self.name);

        let file = match File::open(filename) {
            Ok(file) => Ok(file),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => Err(std::io::Error::new(
                    ErrorKind::NotFound,
                    format!("File not found: {:?}", filename.to_str()),
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

        self.vertices = vertices;
        self.regions = regions;
        self.walls = walls;

        Ok(())
    }

    /// Create a list of vertices from the map data
    pub fn create_vertex_list(&self) -> Vec<String> {
        let mut output_data: Vec<String> = Vec::new();
        for (vertex, region) in self.vertices.iter().zip(self.regions.iter()) {
            let vert1 = format!("{},{},{}", vertex.x, vertex.y, region.floor_height());
            let vert2 = format!("{},{},{}", vertex.x, vertex.y, region.ceiling_height());

            output_data.push(vert1);
            output_data.push(vert2);
        }

        output_data
    }

    /// Creates a CSV string from the vertex data
    pub fn create_vertex_csv(&self) -> String {
        let mut output = Vec::new();

        output.push("x,y,z".to_string());
        output.append(&mut self.create_vertex_list());

        output.join("\n")
    }
}
