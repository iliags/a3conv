#[derive(Debug, Default, Clone)]
pub struct Region {
    name: String,
    floor_height: f32,
    ceiling_height: f32,
}

impl Region {
    pub fn new(name: String, floor_height: f32, ceiling_height: f32) -> Self {
        Self {
            name,
            floor_height,
            ceiling_height,
        }
    }

    pub fn floor_height(&self) -> f32 {
        self.floor_height
    }

    pub fn ceiling_height(&self) -> f32 {
        self.ceiling_height
    }
}
