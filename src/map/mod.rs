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
    objects: Vec<Object>,
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

#[derive(Debug, Default, Clone)]
pub enum ObjectType {
    #[default]
    Actor,
    PlayerStart,
    Thing,
}

impl From<&str> for ObjectType {
    fn from(s: &str) -> Self {
        match s {
            "PLAYER_START" => ObjectType::PlayerStart,
            "THING" => ObjectType::Thing,
            _ => ObjectType::Actor,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Object {
    object_type: ObjectType,
    name: String,
    position: Vertex,
    angle: f32,
    region: usize,
}

#[derive(Debug, Clone)]
enum MapDataType {
    Vertex,
    Region,
    Wall,
    Comment,
    Object,
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

        self.vertices = Vec::new();
        self.regions = Vec::new();
        self.walls = Vec::new();

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
                "VERTEX" => MapDataType::Vertex,
                "REGION" => MapDataType::Region,
                "WALL" => MapDataType::Wall,
                "PLAYER_START" | "THING" | "ACTOR" => MapDataType::Object,
                _ => MapDataType::Comment,
            };

            match line_type {
                MapDataType::Vertex => {
                    // Parse vertex data
                    let x: f32 = parts[1].parse().unwrap_or_default();
                    let y: f32 = parts[2].parse().unwrap_or_default();
                    let z: f32 = parts[3].parse().unwrap_or_default();
                    self.vertices.push(Vector3::new(x, y, z));
                }
                MapDataType::Region => {
                    // Parse region data
                    let name = parts[1].to_string();
                    let floor_hgt: f32 = parts[2].parse().unwrap_or_default();
                    let ceil_hgt: f32 = parts[3].parse().unwrap_or_default();
                    self.regions.push(Region {
                        name,
                        floor_height: floor_hgt,
                        ceiling_height: ceil_hgt,
                    });
                }
                MapDataType::Wall => {
                    // Parse wall data
                    let name = parts[1].to_string();
                    let vertex1_index: usize = parts[2].parse().unwrap_or_default();
                    let vertex2_index: usize = parts[3].parse().unwrap_or_default();
                    let region1_index: usize = parts[4].parse().unwrap_or_default();
                    let region2_index: usize = parts[5].parse().unwrap_or_default();
                    let offset_x: f32 = parts[6].parse().unwrap_or_default();
                    let offset_y: f32 = parts[7].parse().unwrap_or_default();
                    self.walls.push(Wall {
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
                MapDataType::Object => {
                    let offset: usize = match parts[0] {
                        "PLAYER_START" => 1,
                        _ => 0,
                    };
                    self.objects.push(Object {
                        object_type: ObjectType::from(parts[0]),
                        name: parts[1 - offset].to_string(),
                        position: Vector3::new(
                            parts[2 - offset].parse().unwrap_or_default(),
                            parts[3 - offset].parse().unwrap_or_default(),
                            0.0,
                        ),
                        angle: parts[4 - offset].parse().unwrap_or_default(),
                        region: parts[5 - offset].parse().unwrap_or_default(),
                    });
                }
                _ => {}
            }
        }

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
