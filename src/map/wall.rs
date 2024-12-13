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

impl Wall {
    pub fn new(
        name: String,
        vertex1_index: usize,
        vertex2_index: usize,
        region1_index: usize,
        region2_index: usize,
        offset_x: f32,
        offset_y: f32,
        wall_texture: String,
        floor_texture: String,
        ceiling_texture: String,
    ) -> Self {
        Self {
            name,
            vertex1_index,
            vertex2_index,
            region1_index,
            region2_index,
            offset_x,
            offset_y,
            wall_texture,
            floor_texture,
            ceiling_texture,
        }
    }
}
