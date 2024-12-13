use super::Vertex;

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

impl Object {
    pub fn new(
        object_type: ObjectType,
        name: String,
        position: Vertex,
        angle: f32,
        region: usize,
    ) -> Self {
        Object {
            object_type,
            name,
            position,
            angle,
            region,
        }
    }
}
